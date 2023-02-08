use std::{net::IpAddr, str::FromStr, ops::Deref, time::Duration, num::NonZeroUsize, sync::Arc};

use async_compatibility_layer::{logging::{setup_logging, setup_backtrace}, art::{TcpStream, async_sleep}};
use async_lock::RwLock;
use clap::Parser;
use hotshot::{types::{SignatureKey, HotShotHandle}, traits::{NetworkError, election::static_committee::{GeneralStaticCommittee, StaticElectionConfig}, implementations::{Libp2pCommChannel, CentralizedCommChannel, Libp2pNetwork}}, demos::dentry::{DEntryTypes, DEntryNode, DEntryState, DEntryTransaction}};
use hotshot_centralized_server::{Run, TcpStreamUtil, TcpStreamUtilWithRecv, ToServer, FromServer, TcpStreamUtilWithSend, NetworkConfig};
use hotshot_types::{traits::{node_implementation::NodeType, network::CommunicationChannel, metrics::NoMetrics}, data::{ValidatingLeaf, ValidatingProposal}, HotShotConfig};
use libp2p::{Multiaddr, multiaddr::{self, Protocol}, PeerId, identity::{Keypair, ed25519::{SecretKey, Keypair as EdKeypair}}};
use libp2p_networking::network::{NetworkNodeType, MeshParams, NetworkNodeConfigBuilder};
use nll::nll_todo::nll_todo;
use tracing::{instrument, error};

type ThisLeaf = ValidatingLeaf<DEntryTypes>;
type ThisElection =
GeneralStaticCommittee<DEntryTypes, ThisLeaf, <DEntryTypes as NodeType>::SignatureKey>;
type Libp2pNetworking = Libp2pCommChannel<DEntryTypes, ThisLeaf, ThisProposal>;
type CentralizedNetworking = CentralizedCommChannel<DEntryTypes>;
type ThisProposal = ValidatingProposal<DEntryTypes, ThisElection>;
type Libp2pNode = DEntryNode<Libp2pNetworking, ThisElection>;
type CentralizedNode = DEntryNode<CentralizedNetworking, ThisElection>;

/// yeesh maybe we should just implement SignatureKey for this...
pub fn libp2p_generate_indexed_identity(seed: [u8; 32], index: u64) -> Keypair {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(&index.to_le_bytes());
    let new_seed = *hasher.finalize().as_bytes();
    let sk_bytes = SecretKey::from_bytes(new_seed).unwrap();
    let ed_kp = <EdKeypair as From<SecretKey>>::from(sk_bytes);
    Keypair::Ed25519(ed_kp)
}

/// libp2p helper function
/// convert node string into multi addr
/// node string of the form: "$IP:$PORT"
pub fn parse_dns(s: &str) -> Result<Multiaddr, multiaddr::Error> {
    let mut i = s.split(':');
    let ip = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    let port = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    Multiaddr::from_str(&format!("/dns/{ip}/tcp/{port}"))
}

/// libp2p helper function
pub fn parse_ip(s: &str) -> Result<Multiaddr, multiaddr::Error> {
    let mut i = s.split(':');
    let ip = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    let port = i.next().ok_or(multiaddr::Error::InvalidMultiaddr)?;
    Multiaddr::from_str(&format!("/ip4/{ip}/tcp/{port}"))
}

pub const LIBP2P_BOOTSTRAPS_LOCAL_IPS: &[&str] = &[
    "127.0.0.1:9100",
    "127.0.0.1:9101",
    "127.0.0.1:9102",
    "127.0.0.1:9103",
    "127.0.0.1:9104",
    "127.0.0.1:9105",
    "127.0.0.1:9106",
];

pub const LIBP2P_BOOTSTRAPS_REMOTE_IPS: &[&str] = &[
    "0.ap-south-1.cluster.aws.espresso.network:9000",
    "1.ap-south-1.cluster.aws.espresso.network:9000",
    "0.us-east-2.cluster.aws.espresso.network:9000",
    "1.us-east-2.cluster.aws.espresso.network:9000",
    "2.us-east-2.cluster.aws.espresso.network:9000",
    "0.us-west-2.cluster.aws.espresso.network:9000",
    "1.us-west-2.cluster.aws.espresso.network:9000",
];

#[derive(clap::Args, Debug, Clone)]
struct CliOrchestrated {

    /// The address to connect to
    host: IpAddr,

    /// The port to connect to
    port: u16,
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Multi-machine consensus",
    about = "Simulates consensus among multiple machines"
    )]
// FIXME make this CliLibp2p/Orchestrated
enum CliOpt {
    Libp2p(CliOrchestrated),
    Centralized(CliOrchestrated),
}

impl CliOpt {
    pub async fn init(&self) -> Result<Config, NetworkError> {
        match self {
            CliOpt::Libp2p(opt) =>
            {
                let stream = TcpStream::connect(format!("{}:{}", opt.host, opt.port))
                    .await
                    .expect("Could not reach server");
                let mut stream = TcpStreamUtil::new(stream);
                stream.send(ToServer::<<DEntryTypes as NodeType>::SignatureKey>::GetConfig).await.unwrap();
                error!("Waiting for server config...");
                let (mut config, run) = match stream.recv().await.expect("Could not get Libp2pConfig") {
                    FromServer::<<DEntryTypes as NodeType>::SignatureKey, <DEntryTypes as NodeType>::ElectionConfigType>::Config { config, run } => (config, run),
                    x => panic!("Expected Libp2pConfig, got {x:?}"),
                };
                error!("Received server config: {config:?}");
                let (pubkey, privkey) = <<DEntryTypes as NodeType>::SignatureKey as SignatureKey>::generated_from_seed_indexed(config.seed, config.node_index);

                stream
                    .send(ToServer::Identify { key: pubkey })
                    .await
                    .expect("Could not identify with server");

                let libp2p_config = config
                    .libp2p_config
                    .take()
                    .expect("Server is not configured as a libp2p server");
                let bs_len = libp2p_config.bootstrap_nodes.len();
                let bootstrap_nodes : Vec<(PeerId, Multiaddr)> = libp2p_config
                    .bootstrap_nodes
                    .iter()
                    .map(|(addr, pair)| {
                        let kp = Keypair::from_protobuf_encoding(pair).unwrap();
                        let peer_id = PeerId::from_public_key(&kp.public());
                        let mut multiaddr = Multiaddr::from(addr.ip());
                        multiaddr.push(Protocol::Tcp(addr.port()));
                        (peer_id, multiaddr)
                    })
                .collect();
                let identity = libp2p_generate_indexed_identity(
                    config.seed,
                    config.node_index
                    );
                let node_type =
                    if (config.node_index as usize) < bs_len {
                        NetworkNodeType::Bootstrap
                    } else {
                        NetworkNodeType::Regular
                    };
                let node_index = config.node_index;
                let bound_addr =  format!(
                    "/{}/{}/tcp/{}",
                    if libp2p_config.public_ip.is_ipv4() {
                        "ip4"
                    } else {
                        "ip6"
                    },
                    libp2p_config.public_ip,
                    libp2p_config.base_port + node_index as u16
                    )
                    .parse()
                    .unwrap();
                // generate network
                let mut config_builder = NetworkNodeConfigBuilder::default();
                assert!(config.config.total_nodes.get() > 2);
                let replicated_nodes = NonZeroUsize::new(config.config.total_nodes.get() - 2).unwrap();
                config_builder.replication_factor(replicated_nodes);
                config_builder.identity(identity.clone());
                let mesh_params =
                    // NOTE I'm arbitrarily choosing these.
                    match node_type {
                        NetworkNodeType::Bootstrap => MeshParams {
                            mesh_n_high: libp2p_config.bootstrap_mesh_n_high,
                            mesh_n_low: libp2p_config.bootstrap_mesh_n_low,
                            mesh_outbound_min: libp2p_config.bootstrap_mesh_outbound_min,
                            mesh_n: libp2p_config.bootstrap_mesh_n,
                        },
                        NetworkNodeType::Regular => MeshParams {
                            mesh_n_high: libp2p_config.mesh_n_high,
                            mesh_n_low: libp2p_config.mesh_n_low,
                            mesh_outbound_min: libp2p_config.mesh_outbound_min,
                            mesh_n: libp2p_config.mesh_n,
                        },
                        NetworkNodeType::Conductor => unreachable!(),
                    };
                config_builder.mesh_params(Some(mesh_params));

                let node_config = config_builder.build().unwrap();
                let network =
                    Libp2pNetwork::new(
                        NoMetrics::new(),
                        node_config,
                        pubkey,
                        Arc::new(RwLock::new(
                                bootstrap_nodes
                                .iter()
                                .map(|(peer_id, addr)| (Some(*peer_id), addr.clone()))
                                .collect(),
                                )),
                                bs_len,
                                config.node_index as usize,
                                nll_todo(),
                                )
                    .await
                    .map(Libp2pCommChannel::<DEntryTypes, ThisLeaf, ThisProposal>::new);









                config.libp2p_config = Some(libp2p_config);
                // TODO do we want base ports to be the same?? This breaks it for local testing.
                // Maybe that's ok?
                Ok(Config::Libp2pConfig(Libp2pClientConfig {
                    config: *config,
                    //TODO do we need this
                    run,
                    privkey,
                    pubkey,
                    bootstrap_nodes,
                    node_type,
                    identity,
                    bound_addr,
                    socket: stream,
                    network: nll_todo()
                }))
            }
            CliOpt::Centralized(opt) =>
            {
                nll_todo()
                    // Ok(Config::CentralizedConfig(opt.clone()))
            }
        }
    }
}

pub struct Libp2pClientConfig {
    bootstrap_nodes: Vec<(PeerId, Multiaddr)>,
    /// for hotshot layer
    privkey: <<DEntryTypes as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
    /// for hotshot layer
    pubkey: <DEntryTypes as NodeType>::SignatureKey,
    node_type: NetworkNodeType,
    bound_addr: Multiaddr,
    /// for libp2p layer
    identity: Keypair,

    socket: TcpStreamUtil,
    network: Libp2pNetworking,
    //TODO do we need this? I don't think so
    run: Run,
    config: NetworkConfig<<DEntryTypes as NodeType>::SignatureKey, <DEntryTypes as NodeType>::ElectionConfigType>
}

pub enum Config {
    Libp2pConfig(Libp2pClientConfig),
    CentralizedConfig(CentralizedConfig),
}

pub struct CentralizedConfig {
    config: NetworkConfig<<DEntryTypes as NodeType>::SignatureKey, ThisElection>,
    network: CentralizedNetworking,
    run: Run
}

impl Config {
    pub async fn get_network(&self) -> Box<dyn CommunicationChannel<DEntryTypes, ThisLeaf, ThisProposal, ThisElection>> {
        match self {
            Config::Libp2pConfig(c) => Box::new(c.network),
            Config::CentralizedConfig(c) => Box::new(c.network),
        }
    }


    pub async fn wait_for_ready(&self) {
        match self {
            Config::Libp2pConfig(config) => {
                nll_todo()
            }
            Config::CentralizedConfig(config) => {
                let node_count = config.config.config.total_nodes;
                // we leave this as a check + sleep
                // for a finer grained logging about which nodes
                // are "ready to go"TM
                // FIXME we should add this to the commchannel/network API
                while !config.network.run_ready() {
                    let connected_clients = config.network.get_connected_client_count().await;
                    error!("{} / {}", connected_clients, node_count);
                    async_sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    pub async fn init_state_and_hotshot<CommChannel: CommunicationChannel<DEntryTypes, ThisLeaf, ThisProposal, ThisElection>>(
        &self
        ) -> (DEntryState, HotShotHandle<DEntryTypes, DEntryNode<CommChannel, ThisElection>>) {
        match self {
            Config::Libp2pConfig(config) => {
                nll_todo()
            },
            Config::CentralizedConfig(config) => {
                nll_todo()
            },
        }
    }

    pub async fn run_consensus(&self) {
        // let size = mem::size_of::<<DEntryTypes as NodeType>::Transaction>();
        // let adjusted_padding = if padding < size { 0 } else { padding - size };
        nll_todo()
    }
}

#[cfg_attr(
    feature = "tokio-executor",
    tokio::main(flavor = "multi_thread", worker_threads = 2)
    )]
#[cfg_attr(feature = "async-std-executor", async_std::main)]
#[instrument]
async fn main() {
    setup_logging();
    setup_backtrace();

    let mut rng = rand::thread_rng();

    let args = CliOpt::parse();

    let config = args.init().await.unwrap();

    config.wait_for_ready().await;

    config.run_consensus().await;

    // let config: Config = match args {
    //     CliOpt::Libp2p(_) => todo!(),
    //     CliOpt::Centralized(_) => todo!(),
    // };
}


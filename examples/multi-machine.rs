use std::{net::IpAddr, str::FromStr, ops::Deref, time::Duration};

use async_compatibility_layer::{logging::{setup_logging, setup_backtrace}, art::{TcpStream, async_sleep}};
use clap::Parser;
use hotshot::{types::{SignatureKey, HotShotHandle}, traits::{NetworkError, election::static_committee::{GeneralStaticCommittee, StaticElectionConfig}, implementations::{Libp2pCommChannel, CentralizedCommChannel}}, demos::dentry::{DEntryTypes, DEntryNode, DEntryState}};
use hotshot_centralized_server::{Run, TcpStreamUtil, TcpStreamUtilWithRecv, ToServer, FromServer, TcpStreamUtilWithSend, NetworkConfig};
use hotshot_types::{traits::{node_implementation::NodeType, network::CommunicationChannel}, data::{ValidatingLeaf, ValidatingProposal}, HotShotConfig};
use libp2p::{Multiaddr, multiaddr::{self, Protocol}, PeerId, identity::Keypair};
use libp2p_networking::network::NetworkNodeType;
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

pub const BOOTSTRAPS_LOCAL_IPS: &[&str] = &[
    "127.0.0.1:9100",
    "127.0.0.1:9101",
    "127.0.0.1:9102",
    "127.0.0.1:9103",
    "127.0.0.1:9104",
    "127.0.0.1:9105",
    "127.0.0.1:9106",
];

pub const BOOTSTRAPS_REMOTE_IPS: &[&str] = &[
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
                let bootstrap_nodes = libp2p_config
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
                let (node_type, identity) =
                    if (config.node_index as usize) < libp2p_config.bootstrap_nodes.len() {
                        (
                            NetworkNodeType::Bootstrap,
                            Some(
                                Keypair::from_protobuf_encoding(
                                    &libp2p_config.bootstrap_nodes[config.node_index as usize].1,
                                    )
                                .unwrap(),
                                ),
                                )
                    } else {
                        (NetworkNodeType::Regular, None)
                    };
                Ok(Config::Libp2pConfig(Libp2pConfig {
                    run,
                    privkey,
                    pubkey,
                    bootstrap_nodes,
                    node_index: config.node_index,
                    node_type,
                    identity,
                    bound_addr: format!(
                        "/{}/{}/tcp/{}",
                        if libp2p_config.public_ip.is_ipv4() {
                            "ip4"
                        } else {
                            "ip6"
                        },
                        libp2p_config.public_ip,
                        libp2p_config.base_port + config.node_index as u16
                        )
                        .parse()
                        .unwrap(),
                        num_nodes: config.config.total_nodes.get() as _,
                        bootstrap_mesh_n_high: libp2p_config.bootstrap_mesh_n_high,
                        bootstrap_mesh_n_low: libp2p_config.bootstrap_mesh_n_low,
                        bootstrap_mesh_outbound_min: libp2p_config.bootstrap_mesh_outbound_min,
                        bootstrap_mesh_n: libp2p_config.bootstrap_mesh_n,
                        mesh_n_high: libp2p_config.mesh_n_high,
                        mesh_n_low: libp2p_config.mesh_n_low,
                        mesh_outbound_min: libp2p_config.mesh_outbound_min,
                        mesh_n: libp2p_config.mesh_n,
                        next_view_timeout: libp2p_config.next_view_timeout,
                        propose_min_round_time: libp2p_config.propose_min_round_time,
                        propose_max_round_time: libp2p_config.propose_max_round_time,
                        online_time: libp2p_config.online_time,
                        num_txn_per_round: libp2p_config.num_txn_per_round,
                        socket: stream
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

pub struct Libp2pConfig {
    bootstrap_nodes: Vec<(PeerId, Multiaddr)>,
    privkey: <<DEntryTypes as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
    pubkey: <DEntryTypes as NodeType>::SignatureKey,
    node_type: NetworkNodeType,
    bound_addr: Multiaddr,
    identity: Option<Keypair>,
    num_nodes: u64,

    /// FIXME START DUPLICATION with libp2pconfig
    node_index: u64,
    bootstrap_mesh_n_high: usize,
    bootstrap_mesh_n_low: usize,
    bootstrap_mesh_outbound_min: usize,
    bootstrap_mesh_n: usize,
    mesh_n_high: usize,
    mesh_n_low: usize,
    mesh_outbound_min: usize,
    mesh_n: usize,
    next_view_timeout: u64,
    propose_min_round_time: u64,
    propose_max_round_time: u64,
    online_time: u64,
    num_txn_per_round: u64,
    /// FIXME END DUPLICATION

    socket: TcpStreamUtil,
    network: Libp2pNetworking,
    run: Run,
}

pub enum Config {
    Libp2pConfig(Libp2pConfig),
    CentralizedConfig(CentralizedConfig),
}

pub struct CentralizedConfig {
    config: NetworkConfig<<DEntryTypes as NodeType>::SignatureKey, ThisElection>,
    network: CentralizedNetworking,
    run: Run
}

impl Config {
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
        // networking: CommChannel,
        // config: HotShotConfig<<DEntryTypes as NodeType>::SignatureKey, ThisElection>,
        // seed: [u8; 32],
        // node_id: u64,
    ) -> (DEntryState, HotShotHandle<DEntryTypes, DEntryNode<CommChannel, ThisElection>>) {
        match self {
            Config::Libp2pConfig(config) => {
            },
            Config::CentralizedConfig(config) => {
            },
        }
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

    config.wait_for_ready();

    // let config: Config = match args {
    //     CliOpt::Libp2p(_) => todo!(),
    //     CliOpt::Centralized(_) => todo!(),
    // };
}


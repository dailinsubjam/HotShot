use std::{net::IpAddr, str::FromStr, ops::Deref, time::{Duration, Instant}, num::NonZeroUsize, sync::Arc, mem, collections::VecDeque, cmp};

use async_compatibility_layer::{logging::{setup_logging, setup_backtrace}, art::{TcpStream, async_sleep}};
use async_lock::RwLock;
use async_trait::async_trait;
use clap::Parser;
use hotshot::{types::{SignatureKey, HotShotHandle}, traits::{NetworkError, election::static_committee::{GeneralStaticCommittee, StaticElectionConfig}, implementations::{Libp2pCommChannel, CentralizedCommChannel, Libp2pNetwork}, NodeImplementation}, demos::dentry::{DEntryTypes, DEntryNode, DEntryState, DEntryTransaction}};
use hotshot_centralized_server::{Run, TcpStreamUtil, TcpStreamUtilWithRecv, ToServer, FromServer, TcpStreamUtilWithSend, NetworkConfig, RunResults, config::Libp2pConfig};
use hotshot_types::{traits::{node_implementation::NodeType, network::CommunicationChannel, metrics::NoMetrics, state::{TestableState, TestableBlock}, election::Election}, data::{ValidatingLeaf, ValidatingProposal, TestableLeaf}, HotShotConfig};
use libp2p::{Multiaddr, multiaddr::{self, Protocol}, PeerId, identity::{Keypair, ed25519::{SecretKey, Keypair as EdKeypair}}, kad::kbucket::Node};
use libp2p_networking::network::{NetworkNodeType, MeshParams, NetworkNodeConfigBuilder};
use nll::nll_todo::nll_todo;
use tracing::{instrument, error, debug};

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

/// TODO only constrain networking
impl CliConfig<_, _, _, _> for Libp2pConfig {
}

impl CliConfig<_, _, _, _> for CentralizedConfig {
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

pub struct Libp2pClientConfig<TYPES: NodeType> {
    bootstrap_nodes: Vec<(PeerId, Multiaddr)>,
    /// for hotshot layer
    privkey: <<TYPES as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
    /// for hotshot layer
    pubkey: <TYPES as NodeType>::SignatureKey,
    node_type: NetworkNodeType,
    bound_addr: Multiaddr,
    /// for libp2p layer
    identity: Keypair,

    socket: TcpStreamUtil,
    network: Libp2pNetworking,
    //TODO do we need this? I don't think so
    run: Run,
    config: NetworkConfig<<TYPES as NodeType>::SignatureKey, <TYPES as NodeType>::ElectionConfigType>
}

pub enum Config<TYPES: NodeType> {
    Libp2pConfig(Libp2pClientConfig<TYPES>),
    CentralizedConfig(CentralizedConfig<TYPES>),
}

pub struct CentralizedConfig<TYPES: NodeType> {
    config: NetworkConfig<TYPES::SignatureKey, TYPES::ElectionConfigType>,
    network: CentralizedNetworking,
    run: Run
}

impl Config {
    pub async fn wait_for_ready(&self) {
        // FIXME there's absolutely got to be a better way to do this
        // I tried `get_network` but network isn't object safe
        // so that's no good.
        // maybe a trait based approach?
        match self {
            Config::Libp2pConfig(config) => {
                while !CommunicationChannel::<_, _, _, ThisElection>::ready_nonblocking(&config.network).await {
                    async_sleep(Duration::from_secs(1)).await;
                }
            }
            Config::CentralizedConfig(config) => {
                let node_count = config.config.config.total_nodes;
                // we leave this as a check + sleep
                // for a finer grained logging about which nodes
                // are "ready to go"TM
                // FIXME we should add this to the commchannel/network API
                while !CommunicationChannel::<DEntryTypes, ThisLeaf, ThisProposal, ThisElection>::ready_nonblocking(&config.network).await {
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

}

#[async_trait]
pub trait CliConfig<
    TYPES: NodeType,
    ELECTION: Election<TYPES>,
    NETWORK: CommunicationChannel<DEntryTypes, ThisLeaf, ThisProposal, ThisElection>,
    NODE: NodeImplementation<TYPES, Leaf=ValidatingLeaf<TYPES>, Proposal = ValidatingProposal<TYPES, ELECTION>>
>
    where
        <TYPES as NodeType>::StateType : TestableState,
        <TYPES as NodeType>::BlockType : TestableBlock,
        ValidatingLeaf<TYPES> : TestableLeaf,
{
    async fn wait_for_ready(&self);

    async fn init_state_and_hotshot(&self) -> (DEntryState, HotShotHandle<DEntryTypes, DEntryNode<NETWORK, ThisElection>>);

    async fn run_consensus(&self, mut hotshot: HotShotHandle<TYPES, NODE>) -> RunResults {
        let NetworkConfig {
            padding,
            rounds,
            transactions_per_round,
            node_index,
            config: HotShotConfig { total_nodes, .. },
            ..
        } = self.get_config();

        let size = mem::size_of::<TYPES::Transaction>();
        let adjusted_padding = if padding < size { 0 } else { padding - size };
        let mut txns: VecDeque<TYPES::Transaction> = VecDeque::new();
        let state = hotshot.get_state().await;

        // This assumes that no node will be a leader more than 5x the expected number of times they should be the leader
        // FIXME  is this a reasonable assumption when we start doing DA?
        let tx_to_gen = transactions_per_round * (cmp::max(rounds / total_nodes, 1) + 5);
        error!("Generated {} transactions", tx_to_gen);
        {
            let mut txn_rng = rand::thread_rng();
            for _ in 0..tx_to_gen {
                // TODO make this u64...
                let txn = <<TYPES as NodeType>::StateType as TestableState>::create_random_transaction(&state, &mut txn_rng, padding as u64);
                txns.push_back(txn);
            }
        }

        error!("Adjusted padding size is = {:?}", adjusted_padding);
        let mut timed_out_views: u64 = 0;
        let mut round = 1;
        let mut total_transactions = 0;

        let start = Instant::now();

        error!("Starting hotshot!");
        hotshot.start().await;
        while round <= rounds {
            debug!(?round);
            error!("Round {}:", round);

            let num_submitted = if node_index == ((round % total_nodes) as u64) {
                tracing::info!("Generating txn for round {}", round);

                for _ in 0..transactions_per_round {
                    let txn = txns.pop_front().unwrap();
                    tracing::info!("Submitting txn on round {}", round);
                    hotshot.submit_transaction(txn).await.unwrap();
                }
                transactions_per_round
            } else {
                0
            };
            error!("Submitting {} transactions", num_submitted);

            // Start consensus
            error!("  - Waiting for consensus to occur");
            debug!("Waiting for consensus to occur");

            let view_results = hotshot.collect_round_events().await;

            match view_results {
                Ok((state, blocks)) => {
                    if let Some(state) = state.get(0) {
                        // for (account, balance) in &state.balances {
                            // debug!("    - {}: {}", account, balance);
                        // }
                    }
                    for block in blocks {
                        total_transactions += block.txn_count();
                    }
                }
                Err(e) => {
                    timed_out_views += 1;
                    error!("View: {:?}, failed with : {:?}", round, e);
                }
            }

            round += 1;
        }

        let total_time_elapsed = start.elapsed();
        let expected_transactions = transactions_per_round * rounds;
        let total_size = total_transactions * (padding as u64);
        error!("All {rounds} rounds completed in {total_time_elapsed:?}");
        error!("{timed_out_views} rounds timed out");

        // This assumes all submitted transactions make it through consensus:
        error!(
            "{} total bytes submitted in {:?}",
            total_size, total_time_elapsed
            );
        debug!("All rounds completed");

        RunResults {
            // FIXME nuke this field since we're not doing this anymore.
            run: nll_todo(),
            node_index,

            transactions_submitted: total_transactions as usize,
            transactions_rejected: expected_transactions - (total_transactions as usize),
            transaction_size_bytes: (total_size as usize),

            rounds_succeeded: rounds as u64 - timed_out_views,
            rounds_timed_out: timed_out_views,
            total_time_in_seconds: total_time_elapsed.as_secs_f64(),
        }
    }

    fn get_config(&self) -> NetworkConfig<<DEntryTypes as NodeType>::SignatureKey, <DEntryTypes as NodeType>::ElectionConfigType>;
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

    // let mut rng = rand::thread_rng();
    //
    // let args = CliOpt::parse();
    //
    // let config = args.init().await.unwrap();
    //
    // config.wait_for_ready().await;

    // config.run_consensus().await;

    // let config: Config = match args {
    //     CliOpt::Libp2p(_) => todo!(),
    //     CliOpt::Centralized(_) => todo!(),
    // };
}


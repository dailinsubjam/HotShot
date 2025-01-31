// use futures::FutureExt;
// use hotshot::{certificate::QuorumCertificate, traits::TestableNodeImplementation, SystemContext};
//
// use async_compatibility_layer::art::async_spawn;
// use hotshot::demos::sdemo::SDemoBlock;
// use hotshot::demos::sdemo::SDemoState;
// use hotshot::demos::sdemo::SDemoTransaction;
// use hotshot::rand::SeedableRng;
// use hotshot::traits::election::static_committee::GeneralStaticCommittee;
// use hotshot::traits::election::static_committee::StaticCommittee;
// use hotshot::traits::election::static_committee::StaticElectionConfig;
// use hotshot::traits::election::static_committee::StaticVoteToken;
// use hotshot::traits::election::vrf::JfPubKey;
// use hotshot::traits::implementations::MemoryCommChannel;
// use hotshot::traits::implementations::MemoryStorage;
// use hotshot::traits::Block;
// use hotshot::types::SignatureKey;
// use hotshot::HotShotInitializer;
// use hotshot::HotShotSequencingConsensusApi;
// use hotshot_task::event_stream::ChannelStream;
// use hotshot_task::task::FilterEvent;
// use hotshot_task::task::HotShotTaskTypes;
// use hotshot_task::task::{HandleEvent, HotShotTaskCompleted};
// use hotshot_task::task_impls::TaskBuilder;
// use hotshot_task::task_launcher::TaskRunner;
// use hotshot_task_impls::consensus::ConsensusTaskTypes;
// use hotshot_task_impls::consensus::SequencingConsensusTaskState;
// use hotshot_task_impls::events::SequencingHotShotEvent;
// use hotshot_testing::test_builder::TestMetadata;
// use hotshot_types::certificate::ViewSyncCertificate;
// use hotshot_types::data::DAProposal;
// use hotshot_types::data::QuorumProposal;
// use hotshot_types::data::SequencingLeaf;
// use hotshot_types::data::ViewNumber;
// use hotshot_types::message::Message;
// use hotshot_types::message::SequencingMessage;
// use hotshot_types::traits::election::CommitteeExchange;
// use hotshot_types::traits::election::Membership;
// use hotshot_types::traits::election::QuorumExchange;
// use hotshot_types::traits::metrics::NoMetrics;
// use hotshot_types::traits::node_implementation::ChannelMaps;
// use hotshot_types::traits::node_implementation::CommitteeEx;
// use hotshot_types::traits::node_implementation::ExchangesType;
// use hotshot_types::traits::node_implementation::QuorumEx;
// use hotshot_types::traits::node_implementation::SequencingExchanges;
// use hotshot_types::traits::node_implementation::SequencingQuorumEx;
// use hotshot_types::traits::node_implementation::ViewSyncEx;
// use hotshot_types::traits::{
//     election::ConsensusExchange,
//     node_implementation::{NodeImplementation, NodeType},
//     state::ConsensusTime,
// };
// use hotshot_types::vote::DAVote;
// use hotshot_types::vote::QuorumVote;
// use hotshot_types::{certificate::DACertificate, vote::ViewSyncData};
// use jf_primitives::signatures::BLSSignatureScheme;
// #[allow(deprecated)]
// use nll::nll_todo::nll_todo;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::marker::PhantomData;
// use std::sync::Arc;
//
// #[derive(
//     Copy,
//     Clone,
//     Debug,
//     Default,
//     Hash,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     serde::Serialize,
//     serde::Deserialize,
// )]
// pub struct SequencingTestTypes;
// impl NodeType for SequencingTestTypes {
//     type Time = ViewNumber;
//     type BlockType = SDemoBlock;
//     type SignatureKey = JfPubKey<BLSSignatureScheme>;
//     type VoteTokenType = StaticVoteToken<Self::SignatureKey>;
//     type Transaction = SDemoTransaction;
//     type ElectionConfigType = StaticElectionConfig;
//     type StateType = SDemoState;
// }
//
// type StaticDAComm = MemoryCommChannel<
//     SequencingTestTypes,
//     SequencingMemoryImpl,
//     DAProposal<SequencingTestTypes>,
//     DAVote<SequencingTestTypes>,
//     StaticMembership,
// >;
// type StaticQuroumComm = MemoryCommChannel<
//     SequencingTestTypes,
//     SequencingMemoryImpl,
//     QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
//     QuorumVote<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
//     StaticMembership,
// >;
//
// // #[cfg_attr(
// //     feature = "tokio-executor",
// //     tokio::test(flavor = "multi_thread", worker_threads = 2)
// // )]
// // #[cfg_attr(feature = "async-std-executor", async_std::test)]
// // #[instrument]
// // pub fn test_basic() {}
//
// type StaticMembership = StaticCommittee<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;
// #[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
// pub struct SequencingMemoryImpl {}
// use hotshot_types::vote::ViewSyncVote;
//
// type StaticViewSyncComm = MemoryCommChannel<
//     SequencingTestTypes,
//     SequencingMemoryImpl,
//     ViewSyncCertificate<SequencingTestTypes>,
//     ViewSyncVote<SequencingTestTypes>,
//     StaticMembership,
// >;
// use hotshot_types::traits::election::ViewSyncExchange;
// impl NodeImplementation<SequencingTestTypes> for SequencingMemoryImpl {
//     type Storage = MemoryStorage<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>;
//     type Leaf = SequencingLeaf<SequencingTestTypes>;
//     type Exchanges = SequencingExchanges<
//         SequencingTestTypes,
//         Message<SequencingTestTypes, Self>,
//         QuorumExchange<
//             SequencingTestTypes,
//             Self::Leaf,
//             QuorumProposal<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
//             StaticMembership,
//             StaticQuroumComm,
//             Message<SequencingTestTypes, Self>,
//         >,
//         CommitteeExchange<
//             SequencingTestTypes,
//             StaticMembership,
//             StaticDAComm,
//             Message<SequencingTestTypes, Self>,
//         >,
//         ViewSyncExchange<
//             SequencingTestTypes,
//             ViewSyncCertificate<SequencingTestTypes>,
//             StaticMembership,
//             StaticViewSyncComm,
//             Message<SequencingTestTypes, Self>,
//         >,
//     >;
//     type ConsensusMessage = SequencingMessage<SequencingTestTypes, Self>;
//
//     fn new_channel_maps(
//         start_view: ViewNumber,
//     ) -> (
//         ChannelMaps<SequencingTestTypes, Self>,
//         Option<ChannelMaps<SequencingTestTypes, Self>>,
//     ) {
//         (
//             ChannelMaps::new(start_view),
//             Some(ChannelMaps::new(start_view)),
//         )
//     }
// }
//
// // TODO (Keyao) `pub` was added to avoid the `function never used` warning, but we can remove it
// // once we have unit tests using this function.
// pub async fn build_consensus_task<
//     TYPES: NodeType<
//         ElectionConfigType = StaticElectionConfig,
//         SignatureKey = JfPubKey<BLSSignatureScheme>,
//         Time = ViewNumber,
//     >,
//     I: TestableNodeImplementation<
//         TYPES,
//         Leaf = SequencingLeaf<TYPES>,
//         ConsensusMessage = SequencingMessage<TYPES, I>,
//         Storage = MemoryStorage<SequencingTestTypes, SequencingLeaf<SequencingTestTypes>>,
//     >,
// >(
//     task_runner: TaskRunner,
//     event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
// ) -> TaskRunner
// where
//     I::Exchanges: ExchangesType<
//         TYPES,
//         I::Leaf,
//         Message<TYPES, I>,
//         ElectionConfigs = (StaticElectionConfig, StaticElectionConfig),
//     >,
//     SequencingQuorumEx<TYPES, I>: ConsensusExchange<
//         TYPES,
//         Message<TYPES, I>,
//         Proposal = QuorumProposal<TYPES, SequencingLeaf<TYPES>>,
//         Certificate = QuorumCertificate<TYPES, SequencingLeaf<TYPES>>,
//         Commitment = SequencingLeaf<TYPES>,
//         Membership = StaticMembership,
//     >,
//     CommitteeEx<TYPES, I>: ConsensusExchange<
//         TYPES,
//         Message<TYPES, I>,
//         Proposal = DAProposal<TYPES>,
//         Certificate = DACertificate<TYPES>,
//         Commitment = TYPES::BlockType,
//         Membership = StaticMembership,
//     >,
//     ViewSyncEx<TYPES, I>: ConsensusExchange<
//         TYPES,
//         Message<TYPES, I>,
//         Proposal = ViewSyncCertificate<TYPES>,
//         Certificate = ViewSyncCertificate<TYPES>,
//         Commitment = ViewSyncData<TYPES>,
//         Membership = StaticMembership,
//     >,
//     // Why do we need this?
//     GeneralStaticCommittee<
//         SequencingTestTypes,
//         SequencingLeaf<SequencingTestTypes>,
//         JfPubKey<BLSSignatureScheme>,
//     >: Membership<TYPES>,
// {
//     let builder = TestMetadata::default_multiple_rounds();
//
//     let launcher = builder.gen_launcher::<SequencingTestTypes, SequencingMemoryImpl>();
//
//     let node_id = 1;
//     // let network_generator = Arc::new((launcher.resource_generator.network_generator)(node_id));
//     // let quorum_network = (launcher.resource_generator.quorum_network)(network_generator.clone());
//     // let committee_network =
//     //     (launcher.resource_generator.committee_network)(network_generator.clone());
//     // let view_sync_network = (launcher.resource_generator.view_sync_network)(network_generator);
//     let storage = (launcher.resource_generator.storage)(node_id);
//     let config = launcher.resource_generator.config.clone();
//     let initializer =
//         HotShotInitializer::<TYPES, I::Leaf>::from_genesis(I::block_genesis()).unwrap();
//
//     let known_nodes = config.known_nodes.clone();
//     let private_key = I::generate_test_key(node_id);
//     let public_key = TYPES::SignatureKey::from_private(&private_key);
//     let ek =
//         jf_primitives::aead::KeyPair::generate(&mut rand_chacha::ChaChaRng::from_seed([0u8; 32]));
//     let quorum_election_config = config.election_config.clone().unwrap_or_else(|| {
//         <QuorumEx<TYPES,I> as ConsensusExchange<
//                 TYPES,
//                 Message<TYPES, I>,
//             >>::Membership::default_election_config(config.total_nodes.get() as u64)
//     });
//
//     let committee_election_config = config.election_config.clone().unwrap_or_else(|| {
//         <CommitteeEx<TYPES,I> as ConsensusExchange<
//                 TYPES,
//                 Message<TYPES, I>,
//             >>::Membership::default_election_config(config.total_nodes.get() as u64)
//     });
//     let exchanges = I::Exchanges::create(
//         known_nodes.clone(),
//         (quorum_election_config, committee_election_config),
//         (nll_todo(), nll_todo(), nll_todo()),
//         public_key.clone(),
//         private_key.clone(),
//         ek.clone(),
//     );
//     let handle = SystemContext::init(
//         public_key,
//         private_key,
//         node_id,
//         config,
//         storage,
//         exchanges,
//         initializer,
//         NoMetrics::boxed(),
//     )
//     .await
//     .expect("Could not init hotshot");
//
//     let consensus = handle.get_consensus();
//     let c_api: HotShotSequencingConsensusApi<TYPES, I> = HotShotSequencingConsensusApi {
//         inner: handle.hotshot.inner.clone(),
//     };
//
//     let committee_exchange = c_api.inner.exchanges.committee_exchange().clone();
//
//     let registry = task_runner.registry.clone();
//     #[allow(deprecated)]
//     let consensus_state =
//         SequencingConsensusTaskState::<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>> {
//             registry: registry.clone(),
//             consensus,
//             timeout: nll_todo(),
//             cur_view: TYPES::Time::new(0),
//             block: TYPES::BlockType::new(),
//             quorum_exchange: c_api.inner.exchanges.quorum_exchange().clone().into(),
//             api: c_api.clone(),
//             committee_exchange: committee_exchange.clone().into(),
//             _pd: PhantomData,
//             vote_collector: None,
//             timeout_task: async_spawn(async move {}),
//             event_stream: event_stream.clone(),
//             output_event_stream: nll_todo(),
//             certs: HashMap::new(),
//             current_proposal: None,
//             id: nll_todo(),
//             qc: None,
//         };
//     let consensus_event_handler = HandleEvent(Arc::new(
//         move |event,
//               mut state: SequencingConsensusTaskState<
//             TYPES,
//             I,
//             HotShotSequencingConsensusApi<TYPES, I>,
//         >| {
//             async move {
//                 if let SequencingHotShotEvent::Shutdown = event {
//                     (Some(HotShotTaskCompleted::ShutDown), state)
//                 } else {
//                     state.handle_event(event).await;
//                     (None, state)
//                 }
//             }
//             .boxed()
//         },
//     ));
//     let consensus_name = "Consensus Task";
//     let consensus_event_filter = FilterEvent::default();
//
//     let consensus_task_builder = TaskBuilder::<
//         ConsensusTaskTypes<TYPES, I, HotShotSequencingConsensusApi<TYPES, I>>,
//     >::new(consensus_name.to_string())
//     .register_event_stream(event_stream.clone(), consensus_event_filter)
//     .await
//     .register_registry(&mut registry.clone())
//     .await
//     .register_state(consensus_state)
//     .register_event_handler(consensus_event_handler);
//     // impossible for unwrap to fail
//     // we *just* registered
//     let consensus_task_id = consensus_task_builder.get_task_id().unwrap();
//     let consensus_task = ConsensusTaskTypes::build(consensus_task_builder).launch();
//
//     task_runner.add_task(
//         consensus_task_id,
//         consensus_name.to_string(),
//         consensus_task,
//     )
// }

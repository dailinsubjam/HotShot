//! Provides a number of tasks that run continuously on a [`HotShot`]

use crate::{SystemContext, ViewRunner};
use async_compatibility_layer::{
    art::{async_sleep, async_spawn_local, async_timeout},
    channel::{UnboundedReceiver, UnboundedSender},
};
use async_lock::RwLock;
use futures::FutureExt;
use hotshot_task::{
    boxed_sync,
    event_stream::ChannelStream,
    task::{
        FilterEvent, HandleEvent, HandleMessage, HotShotTaskCompleted, HotShotTaskTypes, PassType,
        TaskErr, TS,
    },
    task_impls::{HSTWithEvent, TaskBuilder},
    task_launcher::TaskRunner,
    GeneratedStream, Merge,
};
use hotshot_task_impls::{
    events::SequencingHotShotEvent,
    network::{NetworkTaskState, NetworkTaskTypes},
};
use hotshot_types::message::{Message, Messages, SequencingMessage};
use hotshot_types::traits::election::{ConsensusExchange, Membership};
use hotshot_types::{
    constants::LOOK_AHEAD,
    data::{ProposalType, SequencingLeaf, ViewNumber},
    traits::{
        consensus_type::sequencing_consensus::SequencingConsensus,
        network::{CommunicationChannel, TransmitType},
        node_implementation::{ExchangesType, NodeImplementation, NodeType},
        signature_key::EncodedSignature,
        state::ConsensusTime,
    },
    vote::VoteType,
};
use nll::nll_todo::nll_todo;
use snafu::Snafu;
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tracing::{error, info};

#[cfg(feature = "async-std-executor")]
use async_std::task::{yield_now, JoinHandle};
#[cfg(feature = "tokio-executor")]
use tokio::task::{yield_now, JoinHandle};
#[cfg(not(any(feature = "async-std-executor", feature = "tokio-executor")))]
std::compile_error! {"Either feature \"async-std-executor\" or feature \"tokio-executor\" must be enabled for this crate."}

/// A handle with senders to send events to the background runners.
#[derive(Default)]
pub struct TaskHandle<TYPES: NodeType> {
    /// Inner struct of the [`TaskHandle`]. This is `None` by default but should be initialized
    /// early on in the [`SystemContext`] struct. It should be safe to `unwrap` this.
    pub(crate) inner: RwLock<Option<TaskHandleInner>>,
    /// Reference to the [`NodeType`] used in this configuration
    _types: PhantomData<TYPES>,
}
impl<TYPES: NodeType> TaskHandle<TYPES> {
    /// Start the round runner. This will make it run until `pause` is called
    ///
    /// # Panics
    ///
    /// If the [`TaskHandle`] has not been properly initialized.
    pub async fn start(&self) {
        let handle = self.inner.read().await;
        if handle.is_some() {
            let handle = handle.as_ref().unwrap();
            handle.started.store(true, Ordering::Relaxed);
        }
    }

    /// Make the round runner run 1 round.
    /// Does/should not block.
    ///
    /// # Panics
    ///
    /// If the [`TaskHandle`] has not been properly initialized.
    pub async fn start_one_round(&self) {
        let handle = self.inner.read().await;
        if handle.is_some() {
            let handle = handle.as_ref().unwrap();
            if let Some(s) = &handle.run_view_channels {
                handle.started.store(true, Ordering::Relaxed);
                let _: Result<_, _> = s.send(()).await;
            } else {
                error!("Run one view channel not configured for this hotshot instance");
            }
        }
    }

    /// Wait until all underlying handles are shut down
    ///
    /// # Panics
    ///
    /// If the [`TaskHandle`] has not been properly initialized.
    pub async fn wait_shutdown(&self, send_network_lookup: UnboundedSender<Option<TYPES::Time>>) {
        let inner = self.inner.write().await.take().unwrap();

        // this shuts down the networking task
        if send_network_lookup.send(None).await.is_err() {
            error!("network lookup task already shut down!");
        }

        // shutdown_timeout == the hotshot's view timeout
        // in case the round_runner task is running for `view_timeout`
        // (exponential timeout maxed out)
        // then this needs to be slightly longer such that it ends up being checked
        let long_timeout = inner.shutdown_timeout + Duration::new(20, 0);

        for (handle, name) in [
            (
                inner.network_broadcast_task_handle,
                "network_broadcast_task_handle",
            ),
            (
                inner.network_direct_task_handle,
                "network_direct_task_handle",
            ),
            (inner.consensus_task_handle, "network_change_task_handle"),
        ] {
            assert!(
                async_timeout(long_timeout, handle).await.is_ok(),
                "{name} did not shut down within a second",
            );
        }

        if let Some(committee_network_broadcast_task_handle) =
            inner.committee_network_broadcast_task_handle
        {
            assert!(
                async_timeout(long_timeout, committee_network_broadcast_task_handle)
                    .await
                    .is_ok(),
                "committee_network_broadcast_task_handle did not shut down within a second",
            );
            if let Some(committee_network_direct_task_handle) =
                inner.committee_network_direct_task_handle
            {
                assert!(
                    async_timeout(long_timeout, committee_network_direct_task_handle)
                        .await
                        .is_ok(),
                    "committee_network_direct_task_handle did not shut down within a second",
                );
            }
        }
    }
}
/// Inner struct of the [`TaskHandle`]
pub(crate) struct TaskHandleInner {
    /// for the client to indicate "increment a view"
    /// only Some in Incremental exeuction mode
    /// otherwise None
    pub run_view_channels: Option<UnboundedSender<()>>,

    /// Join handle for `network_broadcast_task`
    pub network_broadcast_task_handle: JoinHandle<()>,

    /// Join handle for `network_direct_task`
    pub network_direct_task_handle: JoinHandle<()>,

    /// Join Handle for committee broadcast network task
    pub committee_network_broadcast_task_handle: Option<JoinHandle<()>>,

    /// Join Handle for committee direct network task
    pub committee_network_direct_task_handle: Option<JoinHandle<()>>,

    /// Join handle for `consensus_task`
    pub consensus_task_handle: JoinHandle<()>,

    /// Global to signify the `HotShot` should be started
    pub(crate) started: Arc<AtomicBool>,

    /// same as hotshot's view_timeout such that
    /// there is not an accidental race between the two
    pub(crate) shutdown_timeout: Duration,
}

/// main thread driving consensus
/// TODO `run_view` refactor: delete
pub async fn view_runner_old<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    hotshot: SystemContext<TYPES::ConsensusType, TYPES, I>,
    started: Arc<AtomicBool>,
    shut_down: Arc<AtomicBool>,
    run_once: Option<UnboundedReceiver<()>>,
) where
    SystemContext<TYPES::ConsensusType, TYPES, I>: ViewRunner<TYPES, I>,
{
    while !shut_down.load(Ordering::Relaxed) && !started.load(Ordering::Relaxed) {
        yield_now().await;
    }

    while !shut_down.load(Ordering::Relaxed) && started.load(Ordering::Relaxed) {
        if let Some(ref recv) = run_once {
            let _: Result<(), _> = recv.recv().await;
        }
        let _: Result<_, _> =
            SystemContext::<TYPES::ConsensusType, TYPES, I>::run_view(hotshot.clone()).await;
    }
}

/// Task to look up a node in the future as needed
pub async fn network_lookup_task<TYPES: NodeType, I: NodeImplementation<TYPES>>(
    hotshot: SystemContext<TYPES::ConsensusType, TYPES, I>,
    shut_down: Arc<AtomicBool>,
) {
    info!("Launching network lookup task");
    let networking = hotshot.inner.exchanges.quorum_exchange().network().clone();
    let inner = hotshot.inner.clone();

    let mut completion_map: HashMap<TYPES::Time, Arc<AtomicBool>> = HashMap::default();

    while !shut_down.load(Ordering::Relaxed) {
        let lock = hotshot.inner.recv_network_lookup.lock().await;

        if let Ok(Some(cur_view)) = lock.recv().await {
            // Injecting consensus data into the networking implementation
            let _result = networking
                .inject_consensus_info((
                    (*cur_view),
                    inner.exchanges.quorum_exchange().is_leader(cur_view),
                    inner.exchanges.quorum_exchange().is_leader(cur_view + 1),
                ))
                .await;

            let view_to_lookup = cur_view + LOOK_AHEAD;

            // perform pruning
            // TODO in the future btreemap would be better
            completion_map = completion_map
                .drain()
                .filter(|(view, is_done)| {
                    if !is_done.load(Ordering::Relaxed) {
                        // we are past the view where this is useful
                        if cur_view >= *view {
                            is_done.store(true, Ordering::Relaxed);
                            return true;
                        }
                        // we aren't done
                        return false;
                    }
                    true
                })
                .collect();

            // logic to look ahead
            if !inner.exchanges.quorum_exchange().is_leader(view_to_lookup) {
                let is_done = Arc::new(AtomicBool::new(false));
                completion_map.insert(view_to_lookup, is_done.clone());
                let inner = inner.clone();
                let networking = networking.clone();
                async_spawn_local(async move {
                    info!("starting lookup for {:?}", view_to_lookup);
                    let _result = networking
                        .lookup_node(inner.exchanges.quorum_exchange().get_leader(view_to_lookup))
                        .await;
                    info!("finished lookup for {:?}", view_to_lookup);
                });
            }
        }
    }

    // shut down all child tasks
    for (_, is_done) in completion_map {
        is_done.store(true, Ordering::Relaxed);
    }
}

/// event for global event stream
#[derive(Clone, Debug)]
pub enum GlobalEvent {
    /// shut everything down
    Shutdown,
    /// dummy (TODO delete later)
    Dummy,
}
impl PassType for GlobalEvent {}

/// Consensus Task Error
#[derive(Snafu, Debug)]
pub struct ConsensusTaskError {}
impl TaskErr for ConsensusTaskError {}

/// consensus task state
#[derive(Debug)]
pub struct ConsensusTaskState {}
impl TS for ConsensusTaskState {}

/// consensus task types
pub type ConsensusTaskTypes =
    HSTWithEvent<ConsensusTaskError, GlobalEvent, ChannelStream<GlobalEvent>, ConsensusTaskState>;

/// Data Availability task error
#[derive(Snafu, Debug)]
pub struct DATaskError {}
impl TaskErr for DATaskError {}

/// Data availability task state
#[derive(Debug)]
pub struct DATaskState {}
impl TS for DATaskState {}

/// Data Availability task types
pub type DATaskTypes =
    HSTWithEvent<DATaskError, GlobalEvent, ChannelStream<GlobalEvent>, DATaskState>;

/// view sync error type
#[derive(Snafu, Debug)]
pub struct ViewSyncTaskError {}
impl TaskErr for ViewSyncTaskError {}

/// view sync task state
#[derive(Debug)]
pub struct ViewSyncTaskState {}
impl TS for ViewSyncTaskState {}

/// Types for view sync task
pub type ViewSyncTaskTypes =
    HSTWithEvent<ViewSyncTaskError, GlobalEvent, ChannelStream<GlobalEvent>, ViewSyncTaskState>;

/// add the networking task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_network_task<
    TYPES: NodeType<ConsensusType = SequencingConsensus, SignatureKey = EncodedSignature>,
    I: NodeImplementation<
        TYPES,
        Leaf = SequencingLeaf<TYPES>,
        ConsensusMessage = SequencingMessage<TYPES, I>,
    >,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    VOTE: VoteType<TYPES>,
    MEMBERSHIP: Membership<TYPES>,
    EXCHANGE: ConsensusExchange<
            TYPES,
            Message<TYPES, I>,
            Proposal = PROPOSAL,
            Vote = VOTE,
            Membership = MEMBERSHIP,
        > + Copy
        + 'static,
>(
    task_runner: TaskRunner,
    event_stream: ChannelStream<SequencingHotShotEvent<TYPES, I>>,
    exchange: EXCHANGE,
) -> TaskRunner
where
    EXCHANGE::Networking:
        CommunicationChannel<TYPES, Message<TYPES, I>, PROPOSAL, VOTE, MEMBERSHIP>,
{
    let channel = exchange.network().clone();
    let broadcast_stream = GeneratedStream::<Messages<TYPES, I>>::new(Arc::new(move || {
        let network = channel.clone();
        let closure = async move {
            let msgs = Messages(
                network
                    .recv_msgs(TransmitType::Broadcast)
                    .await
                    .expect("Failed to receive broadcast messages"),
            );
            async_sleep(Duration::new(0, 500)).await;
            network.shut_down().await;
            msgs
        };
        boxed_sync(closure)
    }));
    let channel = exchange.network().clone();
    let direct_stream = GeneratedStream::<Messages<TYPES, I>>::new(Arc::new(move || {
        let network = channel.clone();
        let closure = async move {
            let msgs = Messages(
                network
                    .recv_msgs(TransmitType::Direct)
                    .await
                    .expect("Failed to receive direct messages"),
            );
            async_sleep(Duration::new(0, 500)).await;
            network.shut_down().await;
            msgs
        };
        boxed_sync(closure)
    }));
    let message_stream = Merge::new(broadcast_stream, direct_stream);
    let channel = exchange.network().clone();
    let network_state: NetworkTaskState<_, _, _, _, _, _> = NetworkTaskState {
        channel,
        event_stream: event_stream.clone(),
        view: ViewNumber::genesis(),
        phantom: PhantomData,
    };
    let registry = task_runner.registry.clone();
    let network_message_handler = HandleMessage(Arc::new(
        move |messages: Messages<TYPES, I>,
              mut state: NetworkTaskState<
            TYPES,
            I,
            PROPOSAL,
            VOTE,
            MEMBERSHIP,
            EXCHANGE::Networking,
        >| {
            async move {
                for message in messages.0 {
                    state.handle_message(message).await;
                }
                (None, state)
            }
            .boxed()
        },
    ));
    let network_event_handler = HandleEvent(Arc::new(
        move |event, mut state: NetworkTaskState<_, _, _, _, MEMBERSHIP, _>| {
            async move {
                if let SequencingHotShotEvent::Shutdown = event {
                    (Some(HotShotTaskCompleted::ShutDown), state)
                } else {
                    state.handle_event(event, exchange.membership()).await;
                    (None, state)
                }
            }
            .boxed()
        },
    ));
    let networking_name = "Networking Task";
    let networking_event_filter = FilterEvent(Arc::new(
        NetworkTaskState::<TYPES, I, PROPOSAL, VOTE, MEMBERSHIP, EXCHANGE::Networking>::filter,
    ));

    let networking_task_builder =
        TaskBuilder::<NetworkTaskTypes<_, _, _, _, _, _>>::new(networking_name.to_string())
            .register_message_stream(message_stream)
            .register_event_stream(event_stream.clone(), networking_event_filter)
            .await
            .register_registry(&mut registry.clone())
            .await
            .register_state(network_state)
            .register_message_handler(network_message_handler)
            .register_event_handler(network_event_handler);

    // impossible for unwraps to fail
    // we *just* registered
    let networking_task_id = networking_task_builder.get_task_id().unwrap();
    let networking_task = NetworkTaskTypes::build(networking_task_builder).launch();

    let task_runner = task_runner.add_task(
        networking_task_id,
        networking_name.to_string(),
        networking_task,
    );

    task_runner
}

/// add the consensus task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_consensus_task(
    task_runner: TaskRunner,
    event_stream: ChannelStream<GlobalEvent>,
) -> TaskRunner {
    // build the consensus task
    let consensus_state = ConsensusTaskState {};
    let registry = task_runner.registry.clone();
    let consensus_event_handler = HandleEvent(Arc::new(move |event, state| {
        async move {
            if let GlobalEvent::Shutdown = event {
                (Some(HotShotTaskCompleted::ShutDown), state)
            } else {
                (None, state)
            }
        }
        .boxed()
    }));
    let consensus_name = "Consensus Task";
    let consensus_event_filter = FilterEvent::default();

    let consensus_task_builder = TaskBuilder::<ConsensusTaskTypes>::new(consensus_name.to_string())
        .register_event_stream(event_stream.clone(), consensus_event_filter)
        .await
        .register_registry(&mut registry.clone())
        .await
        .register_state(consensus_state)
        .register_event_handler(consensus_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let consensus_task_id = consensus_task_builder.get_task_id().unwrap();
    let consensus_task = ConsensusTaskTypes::build(consensus_task_builder).launch();

    task_runner.add_task(
        consensus_task_id,
        consensus_name.to_string(),
        consensus_task,
    )
}

/// add the Data Availability task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_da_task(
    task_runner: TaskRunner,
    event_stream: ChannelStream<GlobalEvent>,
) -> TaskRunner {
    // build the da task
    let da_state = DATaskState {};
    let registry = task_runner.registry.clone();
    let da_event_handler = HandleEvent(Arc::new(move |event, state| {
        async move {
            if let GlobalEvent::Shutdown = event {
                (Some(HotShotTaskCompleted::ShutDown), state)
            } else {
                (None, state)
            }
        }
        .boxed()
    }));
    let da_name = "DA Task";
    let da_event_filter = FilterEvent::default();

    let da_task_builder = TaskBuilder::<DATaskTypes>::new(da_name.to_string())
        .register_event_stream(event_stream.clone(), da_event_filter)
        .await
        .register_registry(&mut registry.clone())
        .await
        .register_state(da_state)
        .register_event_handler(da_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let da_task_id = da_task_builder.get_task_id().unwrap();

    let da_task = DATaskTypes::build(da_task_builder).launch();
    task_runner.add_task(da_task_id, da_name.to_string(), da_task)
}

/// add the view sync task
/// # Panics
/// Is unable to panic. This section here is just to satisfy clippy
pub async fn add_view_sync_task(
    task_runner: TaskRunner,
    event_stream: ChannelStream<GlobalEvent>,
) -> TaskRunner {
    // build the view sync task
    let view_sync_state = ViewSyncTaskState {};
    let registry = task_runner.registry.clone();
    let view_sync_event_handler = HandleEvent(Arc::new(move |event, state| {
        async move {
            if let GlobalEvent::Shutdown = event {
                (Some(HotShotTaskCompleted::ShutDown), state)
            } else {
                (None, state)
            }
        }
        .boxed()
    }));
    let view_sync_name = "ViewSync Task";
    let view_sync_event_filter = FilterEvent::default();

    let view_sync_task_builder = TaskBuilder::<ViewSyncTaskTypes>::new(view_sync_name.to_string())
        .register_event_stream(event_stream.clone(), view_sync_event_filter)
        .await
        .register_registry(&mut registry.clone())
        .await
        .register_state(view_sync_state)
        .register_event_handler(view_sync_event_handler);
    // impossible for unwrap to fail
    // we *just* registered
    let view_sync_task_id = view_sync_task_builder.get_task_id().unwrap();

    let view_sync_task = ViewSyncTaskTypes::build(view_sync_task_builder).launch();
    task_runner.add_task(
        view_sync_task_id,
        view_sync_name.to_string(),
        view_sync_task,
    )
}

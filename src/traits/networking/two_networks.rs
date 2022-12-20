use std::{marker::PhantomData, sync::{Arc, atomic::AtomicBool}};

use async_trait::async_trait;
use hotshot_types::{traits::{network::{NetworkingImplementation, NetworkMessage, NetworkError, NetworkChange, Passable}, node_implementation::NodeType}, message::Message, data::{LeafType, ProposalType}};
use nll::nll_todo::nll_todo;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Clone)]
pub struct TwoNetworks<
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    T: NetworkingImplementation<TYPES, LEAF, PROPOSAL>,
    U: NetworkingImplementation<TYPES, LEAF, PROPOSAL>,
    const N: usize,
    const M: usize>
{
    network_1: [T; N],
    network_2: [U; M],
    _pd_0: PhantomData<TYPES>,
    _pd_1: PhantomData<LEAF>,
    _pd_2: PhantomData<PROPOSAL>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = ""))]
pub struct TwoNetworksMessage<CONTENTS: Passable>{
    contents: CONTENTS,
    targets: Vec<TwoNetworksDirection>,
}

impl<CONTENTS: Passable> Passable for TwoNetworksMessage<CONTENTS> { }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TwoNetworksDirection {
    target: TwoNetworksResource,
    idx: usize,
    priority: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TwoNetworksResource {
    First,
    Second
}


impl<CONTENTS: Passable , const P: usize>
NetworkMessage<CONTENTS> for TwoNetworksMessage<CONTENTS> {
    type Resources = ();

    // const PRIORITIES : usize = P;

    type Targets = Vec<TwoNetworksDirection>;

    fn resources(&self) -> Self::Resources {
        ()
    }

    fn targets(&self) -> Self::Targets {
        self.targets
    }

    const PRIORITIES: usize = P;
}

#[async_trait]
impl<TYPES, LEAF, PROPOSAL, const N: usize, const M: usize, T, U> NetworkingImplementation<TYPES, LEAF, PROPOSAL> for TwoNetworks<TYPES, LEAF, PROPOSAL, T, U, N, M>
where
    TYPES: NodeType,
    LEAF: LeafType<NodeType = TYPES>,
    PROPOSAL: ProposalType<NodeType = TYPES>,
    T: NetworkingImplementation<TYPES, LEAF, PROPOSAL>,
    U: NetworkingImplementation<TYPES, LEAF, PROPOSAL>,
{
    type Message = TwoNetworksMessage<Contents>;
    type Contents = Message<TYPES, LEAF, PROPOSAL>;

    /// Returns true when node is successfully initialized
    /// into the network
    ///
    /// Blocks until node is ready
    async fn ready(&self) -> bool {
        nll_todo()
    }

    /// Broadcasts a message to the network
    ///
    /// Should provide that the message eventually reach all non-faulty nodes
    async fn broadcast_message(
        &self,
        message: Self::Message,
    ) -> Result<(), NetworkError> {
        nll_todo()
    }

    /// Sends a direct message to a specific node
    async fn message_node(
        &self,
        message: Self::Message,
        recipient: TYPES::SignatureKey,
    ) -> Result<(), NetworkError> {
        nll_todo()
    }

    /// Moves out the entire queue of received broadcast messages, should there be any
    ///
    /// Provided as a future to allow the backend to do async locking
    async fn broadcast_queue(&self) -> Result<Vec<Self::Message>, NetworkError> {
        nll_todo()
    }

    /// Provides a future for the next received broadcast
    ///
    /// Will unwrap the underlying `NetworkMessage`
    async fn next_broadcast(&self) -> Result<Self::Message, NetworkError> {
        nll_todo()
    }

    /// Moves out the entire queue of received direct messages to this node
    async fn direct_queue(&self) -> Result<Vec<Self::Message>, NetworkError> {
        nll_todo()
    }

    /// Provides a future for the next received direct message to this node
    ///
    /// Will unwrap the underlying `NetworkMessage`
    async fn next_direct(&self) -> Result<Self::Message, NetworkError> {
        nll_todo()
    }

    /// Node's currently known to the networking implementation
    ///
    /// Kludge function to work around leader election
    async fn known_nodes(&self) -> Vec<TYPES::SignatureKey> {
        nll_todo()
    }

    /// Returns a list of changes in the network that have been observed. Calling this function will clear the internal list.
    async fn network_changes(
        &self,
    ) -> Result<Vec<NetworkChange<TYPES::SignatureKey>>, NetworkError> {
        nll_todo()
    }

    /// Shut down this network. Afterwards this network should no longer be used.
    ///
    /// This should also cause other functions to immediately return with a [`NetworkError`]
    async fn shut_down(&self) -> () {
        nll_todo()
    }

    /// Insert `value` into the shared store under `key`.
    async fn put_record(
        &self,
        key: impl Serialize + Send + Sync + 'static,
        value: impl Serialize + Send + Sync + 'static,
    ) -> Result<(), NetworkError> {
        nll_todo()
    }

    /// Get value stored in shared store under `key`
    async fn get_record<V: for<'a> Deserialize<'a>>(
        &self,
        key: impl Serialize + Send + Sync + 'static,
    ) -> Result<V, NetworkError> {
        nll_todo()
    }

    /// notifies the network of the next leader
    /// so it can prepare. Does not block
    async fn notify_of_subsequent_leader(
        &self,
        pk: TYPES::SignatureKey,
        cancelled: Arc<AtomicBool>,
    ) {
        nll_todo()
    }
}

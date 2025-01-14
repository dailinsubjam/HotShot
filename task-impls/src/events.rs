use hotshot_types::certificate::{DACertificate, QuorumCertificate};
use hotshot_types::data::{DAProposal, ViewNumber};
use hotshot_types::message::Proposal;
use hotshot_types::traits::node_implementation::NodeImplementation;
use hotshot_types::traits::node_implementation::NodeType;
use hotshot_types::traits::node_implementation::QuorumProposalType;
use hotshot_types::traits::node_implementation::ViewSyncProposalType;
use hotshot_types::vote::ViewSyncVote;
use hotshot_types::vote::{DAVote, QuorumVote};

use crate::view_sync::ViewSyncPhase;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SequencingHotShotEvent<TYPES: NodeType, I: NodeImplementation<TYPES>> {
    /// Shutdown the task
    Shutdown,
    /// A quorum proposal has been received from the network; handled by the consensus task
    QuorumProposalRecv(Proposal<QuorumProposalType<TYPES, I>>, TYPES::SignatureKey),
    /// A quorum vote has been received from the network; handled by the consensus task
    QuorumVoteRecv(QuorumVote<TYPES, I::Leaf>),
    /// A DA proposal has been received from the network; handled by the DA task
    DAProposalRecv(Proposal<DAProposal<TYPES>>, TYPES::SignatureKey),
    /// A DA vote has been received by the network; handled by the DA task
    DAVoteRecv(DAVote<TYPES>),
    /// A Data Availability Certificate (DAC) has been recieved by the network; handled by the consensus task
    DACRecv(DACertificate<TYPES>),
    /// Send a quorum proposal to the network; emitted by the leader in the consensus task
    QuorumProposalSend(Proposal<QuorumProposalType<TYPES, I>>, TYPES::SignatureKey),
    /// Send a quorum vote to the next leader; emitted by a replica in the consensus task after seeing a valid quorum proposal
    QuorumVoteSend(QuorumVote<TYPES, I::Leaf>),
    /// Send a DA proposal to the DA committee; emitted by the DA leader (which is the same node as the leader of view v + 1) in the DA task
    DAProposalSend(Proposal<DAProposal<TYPES>>, TYPES::SignatureKey),
    /// Send a DA vote to the DA leader; emitted by DA committee members in the DA task after seeing a valid DA proposal
    DAVoteSend(DAVote<TYPES>),
    /// The next leader has collected enough votes to form a QC; emitted by the next leader in the consensus task; an internal event only
    QCFormed(QuorumCertificate<TYPES, I::Leaf>),
    /// The DA leader has collected enough votes to form a DAC; emitted by the DA leader in the DA task; sent to the entire network via the networking task
    DACSend(DACertificate<TYPES>, TYPES::SignatureKey),
    /// The current view has changed; emitted by the replica in the consensus task or replica in the view sync task; received by almost all other tasks
    ViewChange(ViewNumber),
    /// Timeout for the view sync protocol; emitted by a replica in the view sync task
    ViewSyncTimeout(ViewNumber, u64, ViewSyncPhase),
    /// Send a view sync vote to the network; emitted by a replica in the view sync task
    ViewSyncVoteSend(ViewSyncVote<TYPES>),
    /// Send a view sync certificate to the network; emitted by a relay in the view sync task
    ViewSyncCertificateSend(
        Proposal<ViewSyncProposalType<TYPES, I>>,
        TYPES::SignatureKey,
    ),
    /// Receive a view sync vote from the network; received by a relay in the view sync task
    ViewSyncVoteRecv(ViewSyncVote<TYPES>),
    /// Receive a view sync certificate from the network; received by a replica in the view sync task
    ViewSyncCertificateRecv(Proposal<ViewSyncProposalType<TYPES, I>>),
    /// Trigger the start of the view sync protocol; emitted by view sync task; internal trigger only
    ViewSyncTrigger(ViewNumber),
    /// A consensus view has timed out; emitted by a replica in the consensus task; received by the view sync task; internal event only
    Timeout(ViewNumber),
    /// Receive transactions from the network
    TransactionsRecv(Vec<TYPES::Transaction>),
    /// Send transactions to the network
    TransactionSend(TYPES::Transaction, TYPES::SignatureKey),
    // Event to send DA block data from DA leader to next quorum leader (which should always be the same node); internal event only
    SendDABlockData(TYPES::BlockType),
}

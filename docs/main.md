# PhaseLock: A linear time, committee electing, BFT Protocol.

## Table of contents
  1. [Background](#background)
  2. [Protocol Overview](#protocol-overview)
     1. [Basics](#basics)
        - [View Timeouts](#view-timeouts)
     2. [Sequential PhaseLock](#sequential)
        - [Leader](#sequential-leader)
        - [Replica](#sequential-replica)
     3. [Pipelined PhaseLock](#pipelined)
  3. [Appendices](#appendices) 
     1. [Definitions](#definitions)
        1. [Quorum Certificate](#quorum-certificate)
        2. [Safe Node Predicate](#safe-node-predicate)


# Background

PhaseLock is a hybrid, committee electing, Proof of Stake protocol for the partially synchronous model that exhibits
optimistic responsiveness and linear communication footprint.

PhaseLock's construction borrows heavily from the construction of [Hotstuff](https://arxiv.org/abs/1803.05069) and
[Algorand](https://people.csail.mit.edu/nickolai/papers/gilad-algorand-eprint.pdf), in many senses being a synthesis of
Hotstuff's protocol with Algorand's sortition.

# Protocol Overview

PhaseLock comes in two variants, [Pipelined Phaselock](#pipelined) and [Sequential Phaselock](#sequential).
Sequential PhaseLock is the simpler of the two variants, and is the basal form, from which Pipelined PhaseLock is
derived, so it will be discussed first.

## Basics

The operation of PhaseLock is divided in to a sequence of discrete epoch, referred to as
'views'. Each view is assigned an integer index (represented as a [`u64`]) starting with 0, which is
monotonically increasing.

In each view number, a singe node is designated the 'leader'[^1], and is responsible for driving
consensus for that round. The leader changes with each view number, and every caught-up node
independently calculates who they think the leader is, without communication.

For the sake of simplicity, the leaders are not described as voting, however, the leader in a given
view additionally performs all the actions that a replica would, in addition to its own, including
voting. 

All nodes maintain an internal reference to a Quorum Certificated called the LockedQC, which is
updated when the node commits a Leaf. Nodes will only accept leaves that extend from their LockedQC
_or_ leaves that extend from a QC with a view number higher than that of any QC the node has seen.

### View Timeouts

The particular method a node uses to determine if a round has timed out is not important for saftey,
but is critical for liveness.

Currently, nodes apply an approach where the timeout normally takes on a set value, but engages in
exponential back-off when a round fails, ensuring that _eventually_ enough nodes will been in the
same view number to complete a round.

At present, when a round does successfully complete, the timeout is immediately dropped all the way
back down to the set timeout value.

As the protocol exhibits optimistic responsiveness[^2], there is little detriment to setting the
base value for timeouts higher than is strictly necessary.


## Sequential

Sequential PhaseLock does not currently support committee election or dynamically updating the
membership list, instead using a predefined list of participant nodes with equal weights. Sequential
PhaseLock is essentially identical to [Basic HotStuff](https://arxiv.org/pdf/1803.05069.pdf).

Each view of Sequential PhaseLock is divided into 4 stages the specifics of which depend on if the
node is the leader or a replica. Upon either reviving a commit QC in a round, or the round timing
out, a node will calculate the next leader, and send a NewView message for the next view number to
it, tagged with the nodes current prepareQC.

### Sequential Leader

1. Prepare

   In this phase, the leader:
   * Waits to receive NewView messages from at least `2f + 1` nodes
   * Selects the highest view-numbered Quorum Certificate from among the NewView messages to build
     off of (HighQC),
   * Creates a new Leaf building off of the HighQC, containing the transactions in the leader's
     mempool, tagged with the HighQC as its justification QC
   * Broadcasts the new Leaf to the network as a proposal
2. Pre-Commit

   In this phase, the leader:
   * Waits to receive at least `2f + 1` Votes (partial signatures) on its proposal from the Prepare
     phase
   * Packages the votes into a Quorum Certificate marked as originating in the Prepare phase
   * Broadcasts the Quorum Certificate to the network
3. Commit

   In this phase, the leader:
   * Waits to receive at least `2f + 1` Votes on the Quorum Certificate from the previous phase
   * Packages the votes into a Quorum Certificate marked as originating in the Pre-Commit Phase
   * Broadcasts the Quorum Certificate to the network
4. Decide

   In this phase, the leader:
   * Waits to receive at least `2f + 1` Votes on the Quorum Certificate from the previous phase
   * Packages the votes into a Quorum Certificate marked as originating in the Commit Phase
   * Broadcasts the Quorum Certificate to the network
     
### Sequential Replica

1. Prepare

    In this phase, the replica:
    * Waits for a proposal from the leader for the current view
    * Verifies that the proposal contains a valid block
    * Verifies that the proposal extends from the proposal's justification QC
    * Verifies that the proposal satisfies the safe node predicate
    * Generates a vote (partial signature) for the proposal for the prepare phase
    * Sends the vote to the leader
2. Pre-Commit

    In this phase, the replica:
    * Waits for a Prepare QC from the leader
    * Generates a vote for the proposal for the pre-commit stage
    * Sends the vote to the leader
    
3. Commit

    In this phase, the replica:
  * Waits for a Pre-Commit QC from the leader
  * Generates a vote for the proposal for the commit stage
  * Sends the vote to the leader
  
4. Decide

    In this phase, the replica:
  * Waits for the Commit QC from the node
  * Executes the commands between the previously committed Leaf and the one in the proposal for this
    view
## Pipelined

# Appendices

## Definitions

### Quorum Certificate

A Quorum Certificate is a threshold signature of a [`Leaf`](crate::data::Leaf), composed of
signatures from at least `2f + 1` nodes.

In the case of sequential phaselock, or pipelined phaselock without committee election, `f` is
defined to be the maximum number of faulty nodes the network can tolerate.

In the case of pipelined phaselock with committee election, `f` is defined to be the maximum number
of faulty committee seats the network can tolerate.

### Safe Node Predicate

The safe node predicate can be defined using the following rust-like psuedo code

```ignore
fn safe_node(
    phase_lock: PhaseLock,
    proposal_node: Leaf,
    proposal_justifcation: QuorumCertificate,
) -> bool {
    let saftey_rule = proposal_node.extends_from(phase_lock.locked_qc);
    let liveness_rule = proposal_justifcation.view_number > phase_lock.locked_qc;
    saftey_rule || liveness_rule
}
```

In essence, a node is safe if it either extends from the nodes current locked_qc, _or_ contains a
justification quorum certificate with a view number higher than that of the node's current
locked_qc.

[^1]: Though, it should be noted, that it technically valid for the election process to specifiy
    arbitrarily many leaders for a round, but at most one of them will be able to make progress
    
[^2]: The network will complete a view and move on to the next one as fast as network conditions,
    irrespective of the timeout value for the round
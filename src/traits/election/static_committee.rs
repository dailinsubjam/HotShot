use super::vrf::JfPubKey;
// use ark_bls12_381::Parameters as Param381;
use commit::{Commitment, Committable, RawCommitmentBuilder};
use espresso_systems_common::hotshot::tag;
use hotshot_primitives::qc::bit_vector::StakeTableEntry;
use hotshot_types::{
    data::LeafType,
    traits::{
        election::{Checked, ElectionConfig, ElectionError, Membership, VoteToken},
        node_implementation::NodeType,
        signature_key::{EncodedSignature, SignatureKey},
    },
};
use jf_primitives::signatures::{bls_over_bn254::VerKey};
use jf_primitives::signatures::bls_over_bn254::{BLSOverBN254CurveSignatureScheme, KeyPair as QCKeyPair};
#[allow(deprecated)]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::num::NonZeroU64;
use tracing::error;

/// Dummy implementation of [`Membership`]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneralStaticCommittee<T, LEAF: LeafType<NodeType = T>, PUBKEY: SignatureKey> {
    /// All the nodes participating
    nodes: Vec<PUBKEY>,
    nodes_qc: Vec<StakeTableEntry<VerKey>>,
    /// The nodes on the static committee
    committee_nodes: Vec<PUBKEY>,
    committee_nodes_qc: Vec<StakeTableEntry<VerKey>>,
    /// Node type phantom
    _type_phantom: PhantomData<T>,
    /// Leaf phantom
    _leaf_phantom: PhantomData<LEAF>,
}

/// static committee using a vrf kp
pub type StaticCommittee<T, LEAF> = GeneralStaticCommittee<T, LEAF, JfPubKey<BLSOverBN254CurveSignatureScheme>>;

impl<T, LEAF: LeafType<NodeType = T>, PUBKEY: SignatureKey>
    GeneralStaticCommittee<T, LEAF, PUBKEY>
{
    /// Creates a new dummy elector
    #[must_use]
    pub fn new(nodes: Vec<PUBKEY>, nodes_qc: Vec<StakeTableEntry<VerKey>>) -> Self {
        Self {
            nodes: nodes.clone(),
            nodes_qc: nodes_qc.clone(),
            committee_nodes: nodes,
            committee_nodes_qc: nodes_qc,
            _type_phantom: PhantomData,
            _leaf_phantom: PhantomData,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(bound(deserialize = ""))]
/// Vote token for a static committee
pub struct StaticVoteToken<K: SignatureKey> {
    /// signature
    signature: EncodedSignature,
    /// public key
    pub_key: K,
}

impl<PUBKEY: SignatureKey> VoteToken for StaticVoteToken<PUBKEY> {
    fn vote_count(&self) -> NonZeroU64 {
        NonZeroU64::new(1).unwrap()
    }
}

impl<PUBKEY: SignatureKey> Committable for StaticVoteToken<PUBKEY> {
    fn commit(&self) -> Commitment<Self> {
        RawCommitmentBuilder::new("StaticVoteToken")
            .var_size_field("signature", &self.signature.0)
            .var_size_field("pub_key", &self.pub_key.to_bytes().0)
            .finalize()
    }

    fn tag() -> String {
        tag::STATIC_VOTE_TOKEN.to_string()
    }
}

/// configuration for static committee. stub for now
#[derive(Default, Clone, Serialize, Deserialize, core::fmt::Debug)]
pub struct StaticElectionConfig {
    /// Number of nodes on the committee
    num_nodes: u64,
}

impl ElectionConfig for StaticElectionConfig {}

impl<TYPES, LEAF: LeafType<NodeType = TYPES>, PUBKEY: SignatureKey + 'static> Membership<TYPES>
    for GeneralStaticCommittee<TYPES, LEAF, PUBKEY>
where
    TYPES: NodeType<
        SignatureKey = PUBKEY,
        VoteTokenType = StaticVoteToken<PUBKEY>,
        ElectionConfigType = StaticElectionConfig,
    >,
{
    /// Just use the vector of public keys for the stake table
    type StakeTable = Vec<PUBKEY>;

    /// Clone the static table
    fn get_stake_table(
        &self,
        _view_number: TYPES::Time,
        _state: &TYPES::StateType,
    ) -> Self::StakeTable {
        self.nodes.clone()
    }
    /// Clone the public key and corresponding stake table
    fn get_qc_stake_table (
            &self,
        ) -> Vec<StakeTableEntry<VerKey>> {
            self.nodes_qc.clone()
    }

    fn get_committee_qc_stake_table (
        &self,
    ) -> Vec<StakeTableEntry<VerKey>> {
        self.committee_nodes_qc.clone()
    }

    /// Index the vector of public keys with the current view number
    fn get_leader(&self, view_number: TYPES::Time) -> PUBKEY {
        let index = (*view_number % self.nodes.len() as u64) as usize;
        self.nodes[index].clone()
    }

    /// Simply make the partial signature
    fn make_vote_token(
        &self,
        view_number: TYPES::Time,
        private_key: &<PUBKEY as SignatureKey>::PrivateKey,
        key_pair: QCKeyPair,
    ) -> std::result::Result<Option<StaticVoteToken<PUBKEY>>, ElectionError> {
        // TODO ED Below
        let pub_key = PUBKEY::from_private(private_key);
        if !self.committee_nodes.contains(&pub_key) {
            return Ok(None);
        }
        let mut message: Vec<u8> = vec![];
        message.extend(view_number.to_le_bytes());
        // Change the length from 8 to 32 to make it consistent with other commitments, use defined constant? instead of 32.
        message.extend_from_slice(&[0u8; 32 - 8]);
        let signature = PUBKEY::sign(key_pair, &message);
        Ok(Some(StaticVoteToken { signature, pub_key }))
    }

    fn validate_vote_token(
        &self,
        _view_number: TYPES::Time,
        pub_key: PUBKEY,
        token: Checked<TYPES::VoteTokenType>,
    ) -> Result<Checked<TYPES::VoteTokenType>, ElectionError> {
        match token {
            Checked::Valid(t) | Checked::Unchecked(t) => {
                if !self.committee_nodes.contains(&pub_key) {
                    Ok(Checked::Inval(t))
                } else {
                    Ok(Checked::Valid(t))
                }
            }
            Checked::Inval(t) => Ok(Checked::Inval(t)),
        }
    }

    fn default_election_config(num_nodes: u64) -> TYPES::ElectionConfigType {
        StaticElectionConfig { num_nodes }
    }

    fn create_election(keys_qc: Vec<StakeTableEntry<VerKey>>, keys: Vec<PUBKEY>, config: TYPES::ElectionConfigType) -> Self {
        let mut committee_nodes = keys.clone();
        let mut committee_nodes_qc = keys_qc.clone();
        committee_nodes.truncate(config.num_nodes.try_into().unwrap());
        committee_nodes_qc.truncate(config.num_nodes.try_into().unwrap());
        error!("DA Committee Size: {}", config.num_nodes);
        Self {
            nodes_qc: keys_qc,
            nodes: keys,
            committee_nodes,
            committee_nodes_qc,
            _type_phantom: PhantomData,
            _leaf_phantom: PhantomData,
        }
    }

    fn total_nodes(&self) -> usize {
        self.committee_nodes.len()
    }

    fn success_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.committee_nodes.len() as u64 * 2) / 3) + 1).unwrap()
    }

    fn failure_threshold(&self) -> NonZeroU64 {
        NonZeroU64::new(((self.committee_nodes.len() as u64) / 3) + 1).unwrap()
    }

    fn get_committee(
        &self,
        _view_number: <TYPES as NodeType>::Time,
    ) -> std::collections::BTreeSet<<TYPES as NodeType>::SignatureKey> {
        self.committee_nodes.clone().into_iter().collect()
    }
}

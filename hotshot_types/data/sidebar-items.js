window.SIDEBAR_ITEMS = {"constant":[["_DERIVE_core_fmt_Debug_FOR_CommitmentProposal",""],["_DERIVE_core_fmt_Debug_FOR_DAProposal",""],["_DERIVE_core_fmt_Debug_FOR_ValidatingProposal",""]],"fn":[["fake_commitment","Fake the thing a genesis block points to. Needed to avoid infinite recursion"],["random_commitment","create a random commitment"]],"struct":[["CommitmentProposal","A proposal to append a new block commitment to the log."],["DAProposal","A proposal to start providing data availability for a block."],["InconsistentDeltasError","Error which occurs when [`DeltasType::fill`] is called with a block that does not match the deltas’ internal block commitment."],["InconsistentDeltasSnafu","SNAFU context selector for the `InconsistentDeltasError` error"],["SequencingLeaf","This is the consensus-internal analogous concept to a block, and it contains the block proper, as well as the hash of its parent `Leaf`. NOTE: `State` is constrained to implementing `BlockContents`, is `TypeMap::Block`"],["ValidatingLeaf","This is the consensus-internal analogous concept to a block, and it contains the block proper, as well as the hash of its parent `Leaf`. NOTE: `State` is constrained to implementing `BlockContents`, is `TypeMap::Block`"],["ValidatingProposal","subset of state that we stick into a leaf. original hotstuff proposal"],["ViewNumber","Type-safe wrapper around `u64` so we know the thing we’re talking about is a view number."]],"trait":[["DeltasType","A state change encoded in a leaf."],["LeafType","An item which is appended to a blockchain."],["ProposalType","A proposal to a network of voting nodes."],["TestableLeaf","Additional functions required to use a [`LeafType`] with hotshot-testing."]],"type":[["LeafBlock","The [`Block`] in a [`LeafType`]."],["LeafDeltas","The [`DeltasType`] in a [`LeafType`]."],["LeafDeltasError","Errors reported by the [`DeltasType`] in a [`LeafType`]."],["LeafNode","The [`NodeType`] in a [`LeafType`]."],["LeafState","The [`StateType`] in a [`LeafType`]."],["LeafTime","The [`ConsensusTime`] used by a [`LeafType`]."],["LeafTransaction","The [`Transaction`] in a [`LeafType`]."],["Transaction","The `Transaction` type associated with a `State`, as a syntactic shortcut"],["TxnCommitment","`Commitment` to the `Transaction` type associated with a `State`, as a syntactic shortcut"]]};
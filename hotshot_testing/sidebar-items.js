window.SIDEBAR_ITEMS = {"constant":[["N","For now we only support a size of [`H_256`]. This can be changed in the future."]],"enum":[["ConsensusRoundError","Overarchign errors encountered when trying to reach consensus"],["ConsensusTestError","An overarching consensus test failure"],["TransactionError","Error that is returned from [`TestRunner`] with methods related to transactions"]],"macro":[["cross_all_types","Macro to generate tests for all types based on a description Arguments:"],["cross_all_types_proptest","Macro to generate property-based tests for all types based on a description Arguments:"],["cross_test","Generate a test. Args:"],["cross_tests","Macro to generate tests for all types based on a description Arguments:"],["gen_inner_fn","Generate the inside of a test. Only for internal usage."],["gen_inner_fn_proptest","special casing for proptest. Need the inner thing to block"]],"mod":[["launcher","test launcher infrastructure"],["macros","macros to generate a cross product of tests"],["network_reliability","implementations of various networking models"],["test_description","structs and infra to describe the tests to be written"],["test_types","set of commonly used test types for our tests"]],"struct":[["InconsistentAfterTxnSnafu","SNAFU context selector for the `ConsensusRoundError::InconsistentAfterTxn` variant"],["InvalidNodeSnafu","SNAFU context selector for the `TransactionError::InvalidNode` variant"],["NoNodesSnafu","SNAFU context selector for the `TransactionError::NoNodes` variant"],["NoSuchNodeSnafu","SNAFU context selector for the `ConsensusRoundError::NoSuchNode` variant"],["NoValidBalanceSnafu","SNAFU context selector for the `TransactionError::NoValidBalance` variant"],["Node",""],["ReplicasTimedOutSnafu","SNAFU context selector for the `ConsensusRoundError::ReplicasTimedOut` variant"],["Round","functions to run a round of consensus the control flow is: (1) pre safety check, (2) setup round, (3) post safety check"],["RoundResult","Result of running a round of consensus"],["SafetyFailedSnafu","SNAFU context selector for the `ConsensusRoundError::SafetyFailed` variant"],["TestNodeImpl","An implementation of [`NodeImplementation`] for testing."],["TestRunner","The runner of a test network spin up and down nodes, execute rounds"],["TimedOutWithoutAnyLeaderSnafu","SNAFU context selector for the `ConsensusRoundError::TimedOutWithoutAnyLeader` variant"],["TooManyFailuresSnafu","SNAFU context selector for the `ConsensusTestError::TooManyFailures` variant"],["TransactionSnafu","SNAFU context selector for the `ConsensusRoundError::TransactionError` variant"]],"type":[["Generator","Wrapper for a function that takes a `node_id` and returns an instance of `T`."],["RoundPostSafetyCheck","Type of function used for checking results after running a view of consensus"],["RoundPreSafetyCheck","Type of function used for checking safety before beginnning consensus"],["RoundSetup","Type of function used for configuring a round of consensus"],["StateAndBlock","Alias for `(Vec<S>, Vec<B>)`. Used in [`RoundResult`]."]]};
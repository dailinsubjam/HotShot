window.SIDEBAR_ITEMS = {"enum":[["DHTError","Error enum for querying store because for some reason, [`libp2p::kad::GetRecordError`] does not derive `Error`"],["GetRecordWrapperError","Wrapper Error enum for [`libp2p::kad::GetRecordError`]. [`libp2p::kad::GetRecordError`] does not derive [`std::error::Error`] so in order to feed this into [`DHTError`] and snafu derive, we need a wrapper type"],["NetworkError","wrapper type for errors generated by the `Network`"]],"struct":[["CancelledRequestSnafu","SNAFU context selector for the `DHTError::CancelledRequest` variant"],["DialSnafu","SNAFU context selector for the `NetworkError::DialError` variant"],["DisagreementSnafu","SNAFU context selector for the `DHTError::Disagreement` variant"],["GetRecordSnafu","SNAFU context selector for the `DHTError::GetRecord` variant"],["GossipsubBuildSnafu","SNAFU context selector for the `NetworkError::GossipsubBuild` variant"],["GossipsubConfigSnafu","SNAFU context selector for the `NetworkError::GossipsubConfig` variant"],["NoKnownPeersSnafu","SNAFU context selector for the `NetworkError::NoKnownPeers` variant"],["NotFoundSnafu","SNAFU context selector for the `DHTError::NotFound` variant"],["PublishSnafu","SNAFU context selector for the `NetworkError::PublishError` variant"],["PutRecordSnafu","SNAFU context selector for the `DHTError::PutRecord` variant"],["StreamClosedSnafu","SNAFU context selector for the `NetworkError::StreamClosed` variant"],["TransportLaunchSnafu","SNAFU context selector for the `NetworkError::TransportLaunch` variant"],["TransportSnafu","SNAFU context selector for the `NetworkError::Transport` variant"]]};
initSidebarItems({"enum":[["ClientRequest","Actions to send from the client to the swarm"],["NetworkEvent","events generated by the swarm that we wish to relay to the client"],["NetworkNodeConfigBuilderError","Error type for NetworkNodeConfigBuilder"],["NetworkNodeHandleError","Error wrapper type for interacting with swarm handle"],["NetworkNodeType","this is mostly to estimate how many network connections a node should allow"]],"fn":[["deserialize_msg","deserialize an arbitrary message"],["gen_multiaddr","bind all interfaces on port `port` TODO something more general"],["gen_transport","Generate authenticated transport, copied from `development_transport` http://noiseprotocol.org/noise.html#payload-security-properties for definition of XX"],["get_random_handle","Given a slice of handles assumed to be larger than 0, chooses one"],["serialize_msg","serialize an arbitrary message"],["spawn_handler","Glue function that listens for events from the Swarm corresponding to `handle` and calls `event_handler` when an event is observed. The idea is that this function can be used independent of the actual state we use"],["spin_up_swarm","a single node, connects them to each other and waits for connections to propagate to all nodes."]],"mod":[["error","Contains the [`NetworkError`] snafu types"],["network_node_handle_error","Re-exports of the snafu errors that [`NetworkNodeHandleError`] can throw"]],"struct":[["ConnectionData","metadata about connections"],["NetworkDef","Overarching network behaviour performing:"],["NetworkNode","Network definition"],["NetworkNodeConfig","describe the configuration of the network"],["NetworkNodeConfigBuilder","Builder for `NetworkNodeConfig`."],["NetworkNodeHandle","A handle containing:"]]});
(function() {var implementors = {
"hotshot":[["impl&lt;M: NetworkMsg + 'static + ViewMessage&lt;TYPES&gt;, K: <a class=\"trait\" href=\"hotshot/types/trait.SignatureKey.html\" title=\"trait hotshot::types::SignatureKey\">SignatureKey</a> + 'static, E: ElectionConfig + 'static, TYPES: NodeType + 'static, PROPOSAL: ProposalType&lt;NodeType = TYPES&gt; + 'static, VOTE: <a class=\"trait\" href=\"hotshot/types/trait.VoteType.html\" title=\"trait hotshot::types::VoteType\">VoteType</a>&lt;TYPES&gt; + 'static&gt; ConnectedNetwork&lt;M, K&gt; for <a class=\"struct\" href=\"hotshot/traits/networking/web_server_network/struct.WebServerNetwork.html\" title=\"struct hotshot::traits::networking::web_server_network::WebServerNetwork\">WebServerNetwork</a>&lt;M, K, E, TYPES, PROPOSAL, VOTE&gt;"],["impl&lt;M: NetworkMsg, K: <a class=\"trait\" href=\"hotshot/types/trait.SignatureKey.html\" title=\"trait hotshot::types::SignatureKey\">SignatureKey</a> + 'static&gt; ConnectedNetwork&lt;M, K&gt; for <a class=\"struct\" href=\"hotshot/traits/networking/libp2p_network/struct.Libp2pNetwork.html\" title=\"struct hotshot::traits::networking::libp2p_network::Libp2pNetwork\">Libp2pNetwork</a>&lt;M, K&gt;"],["impl&lt;M: NetworkMsg, K: <a class=\"trait\" href=\"hotshot/types/trait.SignatureKey.html\" title=\"trait hotshot::types::SignatureKey\">SignatureKey</a> + 'static, E: ElectionConfig + 'static&gt; ConnectedNetwork&lt;M, K&gt; for <a class=\"struct\" href=\"hotshot/traits/networking/centralized_server_network/struct.CentralizedServerNetwork.html\" title=\"struct hotshot::traits::networking::centralized_server_network::CentralizedServerNetwork\">CentralizedServerNetwork</a>&lt;K, E&gt;"],["impl&lt;M: NetworkMsg, K: <a class=\"trait\" href=\"hotshot/types/trait.SignatureKey.html\" title=\"trait hotshot::types::SignatureKey\">SignatureKey</a> + 'static&gt; ConnectedNetwork&lt;M, K&gt; for <a class=\"struct\" href=\"hotshot/traits/networking/memory_network/struct.MemoryNetwork.html\" title=\"struct hotshot::traits::networking::memory_network::MemoryNetwork\">MemoryNetwork</a>&lt;M, K&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
(function() {var implementors = {};
implementors["networking_demo"] = [{"text":"impl NetworkBehaviour for <a class=\"struct\" href=\"networking_demo/network/struct.NetworkDef.html\" title=\"struct networking_demo::network::NetworkDef\">NetworkDef</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Gossipsub: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Gossipsub as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Kademlia&lt;MemoryStore&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Kademlia&lt;MemoryStore&gt; as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Identify: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Identify as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;RequestResponse&lt;<a class=\"struct\" href=\"networking_demo/direct_message/struct.DirectMessageCodec.html\" title=\"struct networking_demo::direct_message::DirectMessageCodec\">DirectMessageCodec</a>&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;RequestResponse&lt;<a class=\"struct\" href=\"networking_demo/direct_message/struct.DirectMessageCodec.html\" title=\"struct networking_demo::direct_message::DirectMessageCodec\">DirectMessageCodec</a>&gt; as NetworkBehaviour&gt;::OutEvent&gt;,&nbsp;</span>","synthetic":false,"types":["networking_demo::network::def::NetworkDef"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
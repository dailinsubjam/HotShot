(function() {var implementors = {
"hotshot":[["impl&lt;TYPES, LEAF:&nbsp;LeafType&lt;NodeType = TYPES&gt;, PUBKEY:&nbsp;<a class=\"trait\" href=\"hotshot/types/trait.SignatureKey.html\" title=\"trait hotshot::types::SignatureKey\">SignatureKey</a> + 'static&gt; Membership&lt;TYPES&gt; for <a class=\"struct\" href=\"hotshot/traits/election/static_committee/struct.GeneralStaticCommittee.html\" title=\"struct hotshot::traits::election::static_committee::GeneralStaticCommittee\">GeneralStaticCommittee</a>&lt;TYPES, LEAF, PUBKEY&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TYPES: NodeType&lt;SignatureKey = PUBKEY, VoteTokenType = <a class=\"struct\" href=\"hotshot/traits/election/static_committee/struct.StaticVoteToken.html\" title=\"struct hotshot::traits::election::static_committee::StaticVoteToken\">StaticVoteToken</a>&lt;PUBKEY&gt;, ElectionConfigType = <a class=\"struct\" href=\"hotshot/traits/election/static_committee/struct.StaticElectionConfig.html\" title=\"struct hotshot::traits::election::static_committee::StaticElectionConfig\">StaticElectionConfig</a>&gt;,</span>"],["impl&lt;VRFHASHER, VRFPARAMS, VRF, SIGSCHEME, TYPES, LEAF:&nbsp;LeafType&lt;NodeType = TYPES&gt;&gt; Membership&lt;TYPES&gt; for <a class=\"struct\" href=\"hotshot/traits/election/vrf/struct.VrfImpl.html\" title=\"struct hotshot::traits::election::vrf::VrfImpl\">VrfImpl</a>&lt;TYPES, LEAF, SIGSCHEME, VRF, VRFHASHER, VRFPARAMS&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;SIGSCHEME: SignatureScheme&lt;PublicParameter = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.unit.html\">()</a>, MessageUnit = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;SIGSCHEME::VerificationKey: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + for&lt;'a&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SIGSCHEME::SigningKey: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + for&lt;'a&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SIGSCHEME::Signature: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + for&lt;'a&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;VRF: Vrf&lt;VRFHASHER, VRFPARAMS, PublicParameter = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.unit.html\">()</a>, Input = [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.array.html\">32</a>], Output = [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.68.2/std/primitive.array.html\">32</a>], PublicKey = SIGSCHEME::VerificationKey, SecretKey = SIGSCHEME::SigningKey&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;VRF::Proof: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + for&lt;'a&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.159/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;VRF::PublicParameter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;VRFHASHER: Digest + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;VRFPARAMS: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.2/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + Bls12Parameters,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;VRFPARAMS as Bls12Parameters&gt;::G1Parameters: SWHashToGroup,<br>&nbsp;&nbsp;&nbsp;&nbsp;TYPES: NodeType&lt;VoteTokenType = <a class=\"struct\" href=\"hotshot/traits/election/vrf/struct.VRFVoteToken.html\" title=\"struct hotshot::traits::election::vrf::VRFVoteToken\">VRFVoteToken</a>&lt;VRF::PublicKey, VRF::Proof&gt;, ElectionConfigType = <a class=\"struct\" href=\"hotshot/traits/election/vrf/struct.VRFStakeTableConfig.html\" title=\"struct hotshot::traits::election::vrf::VRFStakeTableConfig\">VRFStakeTableConfig</a>, SignatureKey = <a class=\"struct\" href=\"hotshot/traits/election/vrf/struct.JfPubKey.html\" title=\"struct hotshot::traits::election::vrf::JfPubKey\">JfPubKey</a>&lt;SIGSCHEME&gt;&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
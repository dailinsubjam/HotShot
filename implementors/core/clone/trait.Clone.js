(function() {var implementors = {};
implementors["phaselock"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/data/struct.Leaf.html\" title=\"struct phaselock::data::Leaf\">Leaf</a>&lt;T, N&gt;","synthetic":false,"types":["phaselock::data::Leaf"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/data/struct.QuorumCertificate.html\" title=\"struct phaselock::data::QuorumCertificate\">QuorumCertificate</a>&lt;N&gt;","synthetic":false,"types":["phaselock::data::QuorumCertificate"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/data/struct.VecQuorumCertificate.html\" title=\"struct phaselock::data::VecQuorumCertificate\">VecQuorumCertificate</a>","synthetic":false,"types":["phaselock::data::VecQuorumCertificate"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"phaselock/data/enum.Stage.html\" title=\"enum phaselock::data::Stage\">Stage</a>","synthetic":false,"types":["phaselock::data::Stage"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/data/struct.BlockHash.html\" title=\"struct phaselock::data::BlockHash\">BlockHash</a>&lt;N&gt;","synthetic":false,"types":["phaselock::data::BlockHash"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/demos/dentry/struct.Subtraction.html\" title=\"struct phaselock::demos::dentry::Subtraction\">Subtraction</a>","synthetic":false,"types":["phaselock::demos::dentry::Subtraction"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/demos/dentry/struct.Addition.html\" title=\"struct phaselock::demos::dentry::Addition\">Addition</a>","synthetic":false,"types":["phaselock::demos::dentry::Addition"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/demos/dentry/struct.Transaction.html\" title=\"struct phaselock::demos::dentry::Transaction\">Transaction</a>","synthetic":false,"types":["phaselock::demos::dentry::Transaction"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/demos/dentry/struct.State.html\" title=\"struct phaselock::demos::dentry::State\">State</a>","synthetic":false,"types":["phaselock::demos::dentry::State"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/demos/dentry/struct.DEntryBlock.html\" title=\"struct phaselock::demos::dentry::DEntryBlock\">DEntryBlock</a>","synthetic":false,"types":["phaselock::demos::dentry::DEntryBlock"]},{"text":"impl&lt;NET:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/demos/dentry/struct.DEntryNode.html\" title=\"struct phaselock::demos::dentry::DEntryNode\">DEntryNode</a>&lt;NET&gt;","synthetic":false,"types":["phaselock::demos::dentry::DEntryNode"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/traits/implementations/struct.MemoryNetwork.html\" title=\"struct phaselock::traits::implementations::MemoryNetwork\">MemoryNetwork</a>&lt;T&gt;","synthetic":false,"types":["phaselock::traits::networking::memory_network::MemoryNetwork"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/traits/implementations/struct.WNetwork.html\" title=\"struct phaselock::traits::implementations::WNetwork\">WNetwork</a>&lt;T&gt;","synthetic":false,"types":["phaselock::traits::networking::w_network::WNetwork"]},{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, State:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/traits/implementations/struct.MemoryStorage.html\" title=\"struct phaselock::traits::implementations::MemoryStorage\">MemoryStorage</a>&lt;Block, State, N&gt;","synthetic":false,"types":["phaselock::traits::storage::memory_storage::MemoryStorage"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>, S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.Event.html\" title=\"struct phaselock::types::Event\">Event</a>&lt;B, S&gt;","synthetic":false,"types":["phaselock::types::event::Event"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>, S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"phaselock/types/enum.EventType.html\" title=\"enum phaselock::types::EventType\">EventType</a>&lt;B, S&gt;","synthetic":false,"types":["phaselock::types::event::EventType"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"phaselock/traits/trait.NodeImplementation.html\" title=\"trait phaselock::traits::NodeImplementation\">NodeImplementation</a>&lt;N&gt; + 'static, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.PhaseLockHandle.html\" title=\"struct phaselock::types::PhaseLockHandle\">PhaseLockHandle</a>&lt;B, N&gt;","synthetic":false,"types":["phaselock::types::handle::PhaseLockHandle"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"phaselock/types/enum.Message.html\" title=\"enum phaselock::types::Message\">Message</a>&lt;B, T, S, N&gt;","synthetic":false,"types":["phaselock::types::message::Message"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.NewView.html\" title=\"struct phaselock::types::NewView\">NewView</a>&lt;N&gt;","synthetic":false,"types":["phaselock::types::message::NewView"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.Prepare.html\" title=\"struct phaselock::types::Prepare\">Prepare</a>&lt;T, S, N&gt;","synthetic":false,"types":["phaselock::types::message::Prepare"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.Vote.html\" title=\"struct phaselock::types::Vote\">Vote</a>&lt;N&gt;","synthetic":false,"types":["phaselock::types::message::Vote"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.PreCommit.html\" title=\"struct phaselock::types::PreCommit\">PreCommit</a>&lt;N&gt;","synthetic":false,"types":["phaselock::types::message::PreCommit"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.Commit.html\" title=\"struct phaselock::types::Commit\">Commit</a>&lt;N&gt;","synthetic":false,"types":["phaselock::types::message::Commit"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/types/struct.Decide.html\" title=\"struct phaselock::types::Decide\">Decide</a>&lt;N&gt;","synthetic":false,"types":["phaselock::types::message::Decide"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/utility/broadcast/struct.BroadcastSender.html\" title=\"struct phaselock::utility::broadcast::BroadcastSender\">BroadcastSender</a>&lt;T&gt;","synthetic":false,"types":["phaselock::utility::broadcast::BroadcastSender"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/utility/broadcast/struct.BroadcastReceiver.html\" title=\"struct phaselock::utility::broadcast::BroadcastReceiver\">BroadcastReceiver</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["phaselock::utility::broadcast::BroadcastReceiver"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/struct.PubKey.html\" title=\"struct phaselock::PubKey\">PubKey</a>","synthetic":false,"types":["phaselock::PubKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/struct.PrivKey.html\" title=\"struct phaselock::PrivKey\">PrivKey</a>","synthetic":false,"types":["phaselock::PrivKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/struct.PhaseLockConfig.html\" title=\"struct phaselock::PhaseLockConfig\">PhaseLockConfig</a>","synthetic":false,"types":["phaselock::PhaseLockConfig"]},{"text":"impl&lt;I:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"phaselock/traits/trait.NodeImplementation.html\" title=\"trait phaselock::traits::NodeImplementation\">NodeImplementation</a>&lt;N&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"phaselock/struct.PhaseLock.html\" title=\"struct phaselock::PhaseLock\">PhaseLock</a>&lt;I, N&gt;","synthetic":false,"types":["phaselock::PhaseLock"]}];
implementors["threshold_crypto"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"threshold_crypto/error/enum.Error.html\" title=\"enum threshold_crypto::error::Error\">Error</a>","synthetic":false,"types":["threshold_crypto::error::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"threshold_crypto/error/enum.FromBytesError.html\" title=\"enum threshold_crypto::error::FromBytesError\">FromBytesError</a>","synthetic":false,"types":["threshold_crypto::error::FromBytesError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/poly/struct.Poly.html\" title=\"struct threshold_crypto::poly::Poly\">Poly</a>","synthetic":false,"types":["threshold_crypto::poly::Poly"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/poly/struct.Commitment.html\" title=\"struct threshold_crypto::poly::Commitment\">Commitment</a>","synthetic":false,"types":["threshold_crypto::poly::Commitment"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/poly/struct.BivarPoly.html\" title=\"struct threshold_crypto::poly::BivarPoly\">BivarPoly</a>","synthetic":false,"types":["threshold_crypto::poly::BivarPoly"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/poly/struct.BivarCommitment.html\" title=\"struct threshold_crypto::poly::BivarCommitment\">BivarCommitment</a>","synthetic":false,"types":["threshold_crypto::poly::BivarCommitment"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/serde_impl/struct.SerdeSecret.html\" title=\"struct threshold_crypto::serde_impl::SerdeSecret\">SerdeSecret</a>&lt;T&gt;","synthetic":false,"types":["threshold_crypto::serde_impl::SerdeSecret"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.Fr.html\" title=\"struct threshold_crypto::Fr\">Mersenne8</a>","synthetic":false,"types":["threshold_crypto::mock::ms8::Mersenne8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.PEngine.html\" title=\"struct threshold_crypto::PEngine\">Mocktography</a>","synthetic":false,"types":["threshold_crypto::mock::Mocktography"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.G1Affine.html\" title=\"struct threshold_crypto::G1Affine\">Ms8Affine</a>","synthetic":false,"types":["threshold_crypto::mock::Ms8Affine"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.G1.html\" title=\"struct threshold_crypto::G1\">Ms8Projective</a>","synthetic":false,"types":["threshold_crypto::mock::Ms8Projective"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.PublicKey.html\" title=\"struct threshold_crypto::PublicKey\">PublicKey</a>","synthetic":false,"types":["threshold_crypto::PublicKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.PublicKeyShare.html\" title=\"struct threshold_crypto::PublicKeyShare\">PublicKeyShare</a>","synthetic":false,"types":["threshold_crypto::PublicKeyShare"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.Signature.html\" title=\"struct threshold_crypto::Signature\">Signature</a>","synthetic":false,"types":["threshold_crypto::Signature"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.SignatureShare.html\" title=\"struct threshold_crypto::SignatureShare\">SignatureShare</a>","synthetic":false,"types":["threshold_crypto::SignatureShare"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.SecretKey.html\" title=\"struct threshold_crypto::SecretKey\">SecretKey</a>","synthetic":false,"types":["threshold_crypto::SecretKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.SecretKeyShare.html\" title=\"struct threshold_crypto::SecretKeyShare\">SecretKeyShare</a>","synthetic":false,"types":["threshold_crypto::SecretKeyShare"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.Ciphertext.html\" title=\"struct threshold_crypto::Ciphertext\">Ciphertext</a>","synthetic":false,"types":["threshold_crypto::Ciphertext"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.DecryptionShare.html\" title=\"struct threshold_crypto::DecryptionShare\">DecryptionShare</a>","synthetic":false,"types":["threshold_crypto::DecryptionShare"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.PublicKeySet.html\" title=\"struct threshold_crypto::PublicKeySet\">PublicKeySet</a>","synthetic":false,"types":["threshold_crypto::PublicKeySet"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"threshold_crypto/struct.SecretKeySet.html\" title=\"struct threshold_crypto::SecretKeySet\">SecretKeySet</a>","synthetic":false,"types":["threshold_crypto::SecretKeySet"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
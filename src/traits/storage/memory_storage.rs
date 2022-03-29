//! [`HashMap`](std::collections::HashMap) and [`Vec`] based implementation of the storage trait
//!
//! This module provides a non-persisting, dummy adapter for the [`Storage`] trait

use crate::{
    data::{BlockHash, Leaf, LeafHash},
    traits::{
        storage::{Storage, StorageResult},
        BlockContents, State,
    },
    QuorumCertificate,
};
use async_std::sync::RwLock;
use dashmap::DashMap;
use futures::{
    future::{BoxFuture, FutureExt},
    Future,
};
use phaselock_types::traits::storage::StorageUpdater;
use std::sync::Arc;
use tracing::{info_span, trace, Instrument};

/// Internal state for a [`MemoryStorage`]
struct MemoryStorageInternal<Block, State, const N: usize> {
    /// The Blocks stored by this [`MemoryStorage`]
    blocks: DashMap<BlockHash<N>, Block>,
    /// The [`QuorumCertificate`]s stored by this [`MemoryStorage`]
    ///
    /// In order to maintain the struct constraints, this list must be append only. Once a QC is
    /// inserted, it index _must not_ change
    qcs: RwLock<Vec<QuorumCertificate<N>>>,
    /// Index of the [`QuorumCertificate`]s by hash
    hash_to_qc: DashMap<BlockHash<N>, usize>,
    /// Index of the [`QuorumCertificate`]s by view number
    view_to_qc: DashMap<u64, usize>,
    /// The [`Leaf`s stored by this [`MemoryStorage`]
    ///
    /// In order to maintain the struct constraints, this list must be append only. Once a QC is
    /// inserted, it index _must not_ change
    leaves: RwLock<Vec<Leaf<Block, N>>>,
    /// Index of the [`Leaf`]s by their hashes
    hash_to_leaf: DashMap<LeafHash<N>, usize>,
    /// Index of the [`Leaf`]s by their block's hashes
    block_to_leaf: DashMap<BlockHash<N>, usize>,
    /// The store of states
    states: DashMap<LeafHash<N>, State>,
}

/// In memory, ephemeral, storage for a [`PhaseLock`](crate::PhaseLock) instance
#[derive(Clone)]
pub struct MemoryStorage<Block, State, const N: usize> {
    /// The inner state of this [`MemoryStorage`]
    inner: Arc<MemoryStorageInternal<Block, State, N>>,
}

impl<Block, State, const N: usize> Default for MemoryStorage<Block, State, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Block, State, const N: usize> MemoryStorage<Block, State, N> {
    /// Creates a new, empty [`MemoryStorage`]
    pub fn new() -> Self {
        let inner = MemoryStorageInternal {
            blocks: DashMap::new(),
            qcs: RwLock::new(Vec::new()),
            hash_to_qc: DashMap::new(),
            view_to_qc: DashMap::new(),
            leaves: RwLock::new(Vec::new()),
            hash_to_leaf: DashMap::new(),
            block_to_leaf: DashMap::new(),
            states: DashMap::new(),
        };
        MemoryStorage {
            inner: Arc::new(inner),
        }
    }
}

impl<B: BlockContents<N> + 'static, S: State<N, Block = B> + 'static, const N: usize>
    Storage<B, S, N> for MemoryStorage<B, S, N>
{
    fn get_block<'b, 'a: 'b>(
        &'a self,
        hash: &'b BlockHash<N>,
    ) -> BoxFuture<'b, StorageResult<Option<B>>> {
        async move {
            Ok(if let Some(r) = self.inner.blocks.get(hash) {
                trace!("Block found");
                let block = r.value().clone();
                Some(block)
            } else {
                trace!("Block not found");
                None
            })
        }
        .instrument(info_span!("MemoryStorage::get_block", ?hash))
        .boxed()
    }
    fn get_qc<'b, 'a: 'b>(
        &'a self,
        hash: &'b BlockHash<N>,
    ) -> BoxFuture<'b, StorageResult<Option<QuorumCertificate<N>>>> {
        async move {
            // Check to see if we have the qc
            let index = self.inner.hash_to_qc.get(hash);
            Ok(if let Some(index) = index {
                trace!("Found qc");
                let qcs = self.inner.qcs.read().await;
                let qc = qcs[*index.value()].clone();
                Some(qc)
            } else {
                trace!("Did not find qc");
                None
            })
        }
        .instrument(info_span!("MemoryStorage::get_qc", ?hash))
        .boxed()
    }

    fn get_qc_for_view(
        &self,
        view: u64,
    ) -> BoxFuture<'_, StorageResult<Option<QuorumCertificate<N>>>> {
        async move {
            // Check to see if we have the qc
            let index = self.inner.view_to_qc.get(&view);
            Ok(if let Some(index) = index {
                trace!("Found qc");
                let qcs = self.inner.qcs.read().await;
                let qc = qcs[*index.value()].clone();
                Some(qc)
            } else {
                trace!("Did not find qc");
                None
            })
        }
        .instrument(info_span!("MemoryStorage::get_qc_for_view", ?view))
        .boxed()
    }
    fn get_leaf<'b, 'a: 'b>(
        &'a self,
        hash: &'b LeafHash<N>,
    ) -> BoxFuture<'b, StorageResult<Option<Leaf<B, N>>>> {
        async move {
            trace!(?self.inner.hash_to_leaf, ?hash);
            // Check to see if we have the leaf
            let index = self.inner.hash_to_leaf.get(hash);
            Ok(if let Some(index) = index {
                trace!("Found leaf");
                let leaves = self.inner.leaves.read().await;
                Some(leaves[*index.value()].clone())
            } else {
                trace!("Did not find leaf");
                None
            })
        }
        .instrument(info_span!("MemoryStorage::get_leaf", ?hash))
        .boxed()
    }

    fn get_leaf_by_block<'b, 'a: 'b>(
        &'a self,
        hash: &'b BlockHash<N>,
    ) -> BoxFuture<'b, StorageResult<Option<Leaf<B, N>>>> {
        async move {
            // Check to see if we have the leaf
            let index = self.inner.block_to_leaf.get(hash);
            Ok(if let Some(index) = index {
                trace!("Found leaf");
                let leaves = self.inner.leaves.read().await;
                Some(leaves[*index.value()].clone())
            } else {
                trace!("Did not find leaf");
                None
            })
        }
        .instrument(info_span!("MemoryStorage::get_by_block", ?hash))
        .boxed()
    }

    fn get_state<'b, 'a: 'b>(
        &'a self,
        hash: &'b LeafHash<N>,
    ) -> BoxFuture<'b, StorageResult<Option<S>>> {
        let maybe_state = self.inner.states.get(hash);
        let x = Ok(if let Some(state) = maybe_state {
            let state = state.value().clone();
            Some(state)
        } else {
            None
        });
        async move { x }.boxed()
    }

    fn update<'a, F, FUT>(&'a self, update_fn: F) -> BoxFuture<'_, StorageResult>
    where
        F: FnOnce(Box<dyn StorageUpdater<'a, B, S, N> + 'a>) -> FUT + Send + 'a,
        FUT: Future<Output = StorageResult> + Send + 'a,
    {
        async move {
            let updater = Box::new(MemoryStorageUpdater { inner: &self.inner });
            update_fn(updater).await?;
            Ok(())
        }
        .boxed()
    }
}

/// An implementation of [`StorageUpdater`] for [`MemoryStorage`]
struct MemoryStorageUpdater<'a, B, S, const N: usize> {
    /// Reference to the internals of the memory storage
    inner: &'a MemoryStorageInternal<B, S, N>,
}

impl<'a, B, S, const N: usize> StorageUpdater<'a, B, S, N> for MemoryStorageUpdater<'a, B, S, N>
where
    B: BlockContents<N> + 'static,
    S: State<N, Block = B> + 'static,
{
    fn insert_block(&mut self, hash: BlockHash<N>, block: B) -> BoxFuture<'_, StorageResult> {
        async move {
            trace!(?block, "inserting block");
            self.inner.blocks.insert(hash, block);
            Ok(())
        }
        .instrument(info_span!("MemoryStorage::insert_block", ?hash))
        .boxed()
    }

    fn insert_qc(&mut self, qc: QuorumCertificate<N>) -> BoxFuture<'_, StorageResult> {
        async move {
            // Insert the qc into the main vec and the add the references
            let view = qc.view_number;
            let hash = qc.block_hash;
            let mut qcs = self.inner.qcs.write().await;
            let index = qcs.len();
            trace!(?qc, ?index, "Inserting qc");
            qcs.push(qc);
            self.inner.view_to_qc.insert(view, index);
            self.inner.hash_to_qc.insert(hash, index);
            Ok(())
        }
        .instrument(info_span!("MemoryStorage::insert_qc"))
        .boxed()
    }

    fn insert_leaf(&mut self, leaf: Leaf<B, N>) -> BoxFuture<'_, StorageResult> {
        async move {
            let hash = leaf.hash();
            trace!(?leaf, ?hash, "Inserting");
            let block_hash = BlockContents::hash(&leaf.item);
            let mut leaves = self.inner.leaves.write().await;
            let index = leaves.len();
            trace!(?leaf, ?index, "Inserting leaf");
            leaves.push(leaf);
            self.inner.hash_to_leaf.insert(hash, index);
            self.inner.block_to_leaf.insert(block_hash, index);
            Ok(())
        }
        .instrument(info_span!("MemoryStorage::insert_leaf"))
        .boxed()
    }

    fn insert_state(&mut self, state: S, hash: LeafHash<N>) -> BoxFuture<'_, StorageResult> {
        async move {
            trace!(?hash, "Inserting state");
            self.inner.states.insert(hash, state);
            Ok(())
        }
        .instrument(info_span!("MemoryStorage::insert_state"))
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::Stage;
    #[allow(clippy::wildcard_imports)]
    use phaselock_types::traits::block_contents::dummy::*;
    use phaselock_utils::test_util::setup_logging;
    use tracing::instrument;

    fn dummy_qc(
        hash_block: BlockHash<32>,
        hash_leaf: LeafHash<32>,
        view: u64,
        valid: bool,
    ) -> QuorumCertificate<32> {
        QuorumCertificate {
            block_hash: hash_block,
            leaf_hash: hash_leaf,
            view_number: view,
            stage: if valid { Stage::Decide } else { Stage::None },
            signature: None,
            genesis: true,
        }
    }

    #[async_std::test]
    #[instrument]
    async fn blocks() {
        setup_logging();
        // Get our storage and dummy block
        let storage = MemoryStorage::<DummyBlock, DummyState, 32>::default();
        let test_block_1 = DummyBlock::random();
        let hash_1 = <DummyBlock as BlockContents<32>>::hash(&test_block_1);
        let test_block_2 = DummyBlock::random();
        let hash_2 = <DummyBlock as BlockContents<32>>::hash(&test_block_2);
        // Attempt to insert the blocks
        let res = storage
            .update(|mut m| {
                let test_block_1 = test_block_1.clone();
                async move { m.insert_block(hash_1, test_block_1).await }
            })
            .await;
        assert!(res.is_ok());
        let res = storage
            .update(|mut m| {
                let test_block_2 = test_block_2.clone();
                async move {
                    m.insert_block(hash_2, test_block_2).await?;
                    Ok(())
                }
            })
            .await;
        assert!(res.is_ok());
        // Then attempt to get the blocks
        let block_1 = storage.get_block(&hash_1).await.unwrap().unwrap();
        let block_2 = storage.get_block(&hash_2).await.unwrap().unwrap();
        // Make sure we got the right blocks
        assert_eq!(block_1, test_block_1);
        assert_eq!(block_2, test_block_2);
        // Try to get an invalid block and make sure it is nothing
        let bad_hash = BlockHash::<32>::random();
        let res = storage.get_block(&bad_hash).await.unwrap();
        assert!(res.is_none());
    }

    #[async_std::test]
    #[instrument]
    async fn qcs() {
        setup_logging();
        let storage = MemoryStorage::<DummyBlock, DummyState, 32>::default();
        // Create a few dummy qcs
        let qc_1_hash_block = BlockHash::<32>::random();
        let qc_1_hash_leaf = LeafHash::<32>::random();
        let qc_1 = dummy_qc(qc_1_hash_block, qc_1_hash_leaf, 1, true);
        let qc_2_hash_block = BlockHash::<32>::random();
        let qc_2_hash_leaf = LeafHash::<32>::random();
        let qc_2 = dummy_qc(qc_2_hash_block, qc_2_hash_leaf, 2, true);
        // Attempt to insert them
        storage
            .update(|mut m| {
                let qc_1 = qc_1.clone();
                let qc_2 = qc_2.clone();
                async move {
                    m.insert_qc(qc_1).await?;
                    m.insert_qc(qc_2).await?;
                    Ok(())
                }
            })
            .await
            .unwrap();
        // Attempt to get them back by hash
        let h_qc_1 = storage.get_qc(&qc_1_hash_block).await.unwrap().unwrap();
        let h_qc_2 = storage.get_qc(&qc_2_hash_block).await.unwrap().unwrap();
        // Check to make sure we got the right QCs back
        assert_eq!(h_qc_1, qc_1);
        assert_eq!(h_qc_2, qc_2);
        // Attempt to get them back by view number
        let v_qc_1 = storage.get_qc_for_view(1).await.unwrap().unwrap();
        let v_qc_2 = storage.get_qc_for_view(2).await.unwrap().unwrap();
        // Check to make sure we got the right QCs back
        assert_eq!(v_qc_1, qc_1);
        assert_eq!(v_qc_2, qc_2);
        // Make sure trying to get bunk QCs fails
        let bunk_hash = BlockHash::<32>::random();
        assert!(storage.get_qc(&bunk_hash).await.unwrap().is_none());
        assert!(storage.get_qc_for_view(3).await.unwrap().is_none());
        // Make sure inserting a bunk QC fails
        //let bad_qc = dummy_qc(bunk_hash, 3, false);
        //assert!(!storage.insert_qc(bad_qc).await.is_some());
    }

    #[async_std::test]
    #[instrument]
    async fn leaves() {
        setup_logging();
        let storage = MemoryStorage::<DummyBlock, DummyState, 32>::default();
        // Create a few dummy leaves
        let block_1 = DummyBlock::random();
        let block_2 = DummyBlock::random();
        let parent_1 = LeafHash::<32>::random();
        let parent_2 = LeafHash::<32>::random();
        let leaf_1 = Leaf {
            parent: parent_1,
            item: block_1.clone(),
        };
        let hash_1 = leaf_1.hash();
        let leaf_2 = Leaf {
            parent: parent_2,
            item: block_2.clone(),
        };
        let hash_2 = leaf_2.hash();
        // Attempt to insert them
        storage
            .update(|mut m| {
                let leaf_1 = leaf_1.clone();
                let leaf_2 = leaf_2.clone();
                async move {
                    m.insert_leaf(leaf_1).await?;
                    m.insert_leaf(leaf_2).await?;
                    Ok(())
                }
            })
            .await
            .unwrap();
        // Attempt to get them back by hash
        let h_leaf_1 = storage.get_leaf(&hash_1).await.unwrap().unwrap();
        let h_leaf_2 = storage.get_leaf(&hash_2).await.unwrap().unwrap();
        // Make sure they are the right leaves
        assert_eq!(h_leaf_1.parent, leaf_1.parent);
        assert_eq!(h_leaf_1.item, leaf_1.item);
        assert_eq!(h_leaf_2.parent, leaf_2.parent);
        assert_eq!(h_leaf_2.item, leaf_2.item);
        // Attempt to get them back by block hash
        let b_leaf_1 = storage
            .get_leaf_by_block(&<DummyBlock as BlockContents<32>>::hash(&block_1))
            .await
            .unwrap()
            .unwrap();
        let b_leaf_2 = storage
            .get_leaf_by_block(&<DummyBlock as BlockContents<32>>::hash(&block_2))
            .await
            .unwrap()
            .unwrap();
        // Make sure they are the right leaves
        assert_eq!(b_leaf_1.parent, leaf_1.parent);
        assert_eq!(b_leaf_1.item, leaf_1.item);
        assert_eq!(b_leaf_2.parent, leaf_2.parent);
        assert_eq!(b_leaf_2.item, leaf_2.item);
        // Getting a bunk leaf by hash fails
        assert!(storage
            .get_leaf(&LeafHash::<32>::random())
            .await
            .unwrap()
            .is_none());
        // Getting a bunk leaf by block hash fails
        assert!(storage
            .get_leaf_by_block(&BlockHash::<32>::random())
            .await
            .unwrap()
            .is_none());
    }
}
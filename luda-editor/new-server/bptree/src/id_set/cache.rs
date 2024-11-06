use super::*;
use arc_swap::ArcSwapAny;
use std::{collections::HashMap, sync::Arc};

pub(crate) type Pages = HashMap<PageOffset, Page>;
pub(crate) type CachedPages = Arc<Pages>;

#[derive(Debug, Clone, Default)]
pub(crate) struct PageCache {
    inner: Arc<ArcSwapAny<CachedPages>>,
}

impl PageCache {
    pub fn clone_inner(&self) -> Pages {
        self.inner.load().as_ref().clone()
    }

    pub(crate) fn load(&self) -> Arc<Pages> {
        self.inner.load_full()
    }

    pub(crate) fn store(&self, new_cache: Pages) {
        self.inner.store(Arc::new(new_cache));
    }

    pub(crate) fn contains_id(&self, id: u128) -> Option<bool> {
        let guard = self.inner.load();
        let header = guard.get(&PageOffset::HEADER)?.as_header();
        let mut node = guard.get(&header.root_node_offset)?.as_node();

        loop {
            if node.is_leaf() {
                let leaf_node = node.as_leaf_node();

                return Some(leaf_node.contains(id));
            }
            let internal_node = node.as_internal_node();
            let child_offset = internal_node.find_child_offset_for(id);
            node = guard.get(&child_offset)?.as_node();
        }
    }
}

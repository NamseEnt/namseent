use super::*;
use arc_swap::ArcSwapAny;
use std::{
    collections::BTreeMap,
    ops::Deref,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::Instant,
};

pub(crate) type Pages = BTreeMap<PageOffset, WithLastRefTime<Page>>;
pub(crate) type CachedPages = Arc<Pages>;

#[derive(Debug, Clone)]
pub(crate) struct PageCache {
    inner: Arc<ArcSwapAny<CachedPages>>,
    limit: usize,
}

impl PageCache {
    pub fn new(limit: usize) -> Self {
        Self {
            inner: Default::default(),
            limit,
        }
    }
    /// Returns the stale pages that were evicted from the cache.
    /// This function assume that `tuples` is not in the cache.
    pub fn push(&self, new_pages: BTreeMap<PageOffset, Page>) -> Vec<(PageOffset, Page)> {
        if new_pages.is_empty() || self.limit < 1 {
            return new_pages.into_iter().collect();
        }

        let mut cache = self.inner.load_full().as_ref().clone();

        let Some(evict_count) = (cache.len() + new_pages.len()).checked_sub(self.limit) else {
            cache.append(
                &mut new_pages
                    .into_iter()
                    .map(|(offset, page)| (offset, WithLastRefTime::new(page)))
                    .collect::<BTreeMap<_, _>>(),
            );
            self.inner.store(Arc::new(cache));

            return Vec::new();
        };

        let mut evicted: Vec<(PageOffset, Page)> = Vec::with_capacity(evict_count);

        if cache.len() < evict_count {
            let evict_from_new_pages = evict_count - cache.len();

            evicted.extend(
                &mut std::mem::take(&mut cache)
                    .into_iter()
                    .map(|(offset, page)| (offset, page.inner)),
            );

            if evict_from_new_pages > 0 {
                let mut new_pages_vec = new_pages.into_iter().collect::<Vec<_>>();
                let mut evicts =
                    new_pages_vec.split_off(new_pages_vec.len() - evict_from_new_pages);
                evicted.append(&mut evicts);
                cache = new_pages_vec
                    .into_iter()
                    .map(|(offset, page)| (offset, WithLastRefTime::new(page)))
                    .collect();
            }
        } else {
            let mut cache_vec: Vec<_> = cache.into_iter().collect();
            cache_vec.sort_by(|(_, a), (_, b)| {
                a.last_ref
                    .load(Ordering::Relaxed)
                    .cmp(&b.last_ref.load(Ordering::Relaxed))
            });
            let evicts = cache_vec.split_off(cache_vec.len() - evict_count);
            evicted.extend(
                evicts
                    .into_iter()
                    .map(|(offset, page)| (offset, page.inner)),
            );

            cache = cache_vec.into_iter().collect();
            cache.append(
                &mut new_pages
                    .into_iter()
                    .map(|(offset, page)| (offset, WithLastRefTime::new(page)))
                    .collect(),
            );
        }

        self.inner.store(Arc::new(cache));

        evicted
    }
    pub fn contains_id(&self, id: u128) -> Option<bool> {
        let guard = self.inner.load();
        let header = guard.get(&PageOffset::HEADER)?.as_header();
        let mut node = guard.get(&header.root_node_offset)?.as_node();

        loop {
            match node.as_one_of() {
                NodeMatchRef::Internal { internal_node } => {
                    let child_offset = internal_node.find_child_offset_for(id);
                    node = guard.get(&child_offset)?.as_node();
                }
                NodeMatchRef::Leaf { leaf_node } => return Some(leaf_node.contains(id)),
            }
        }
    }

    /// # Return
    ///
    ///   - `None` for cache miss.
    ///   - `Some(None)` if no more ids are available.
    pub(crate) fn next(&self, exclusive_start_id: Option<u128>) -> Option<Option<Vec<u128>>> {
        let id = exclusive_start_id.unwrap_or_default();
        let guard = self.inner.load();
        let header = guard.get(&PageOffset::HEADER)?.as_header();
        let mut node = guard.get(&header.root_node_offset)?.as_node();

        loop {
            match node.as_one_of() {
                NodeMatchRef::Internal { internal_node } => {
                    let child_offset = internal_node.find_child_offset_for(id);
                    assert_ne!(child_offset, PageOffset::NULL);

                    node = guard.get(&child_offset)?.as_node();
                }
                NodeMatchRef::Leaf { leaf_node } => match leaf_node.next(exclusive_start_id) {
                    NextResult::Found { ids } => {
                        return Some(Some(ids));
                    }
                    NextResult::NoMoreIds => {
                        return Some(None);
                    }
                    NextResult::CheckRightNode { right_node_offset } => {
                        assert_ne!(right_node_offset, PageOffset::NULL);
                        node = guard.get(&right_node_offset)?.as_node();
                        assert!(node.is_leaf());
                        continue;
                    }
                },
            }
        }
    }

    pub(crate) fn load_full(&self) -> CachedPages {
        self.inner.load_full()
    }
}

#[derive(Debug)]
pub(crate) struct WithLastRefTime<T> {
    inner: T,
    last_ref: AtomicU64,
}
static START_INSTANT: OnceLock<Instant> = OnceLock::new();
fn start_elapsed() -> u64 {
    START_INSTANT.get_or_init(Instant::now).elapsed().as_secs()
}
impl<T> WithLastRefTime<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            last_ref: AtomicU64::new(start_elapsed()),
        }
    }
}
impl<T: Clone> Clone for WithLastRefTime<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            last_ref: AtomicU64::new(self.last_ref.load(Ordering::Relaxed)),
        }
    }
}
impl<T> AsRef<T> for WithLastRefTime<T> {
    fn as_ref(&self) -> &T {
        self.last_ref.store(start_elapsed(), Ordering::Relaxed);
        &self.inner
    }
}
impl<T> Deref for WithLastRefTime<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

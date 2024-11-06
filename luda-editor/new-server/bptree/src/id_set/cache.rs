use super::*;
use arc_swap::ArcSwapAny;
use std::{
    collections::BTreeMap,
    ops::Deref,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock,
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
    pub fn load(&self) -> Arc<Pages> {
        self.inner.load_full()
    }
    /// Returns the stale pages that were evicted from the cache.
    /// This function assume that `tuples` is not in the cache.
    pub fn push(&self, new_pages: BTreeMap<PageOffset, Page>) -> Vec<(PageOffset, Page)> {
        if new_pages.is_empty() || self.limit < 1 {
            return Vec::new();
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

        let mut evicted = Vec::with_capacity(evict_count);

        if cache.len() >= evict_count {
            let overflowed = cache.len() - evict_count;

            evicted.extend(
                &mut std::mem::take(&mut cache)
                    .into_iter()
                    .map(|(offset, page)| (offset, page.inner)),
            );

            if overflowed > 0 {
                let mut new_pages_vec = new_pages.into_iter().collect::<Vec<_>>();
                let mut evicts = new_pages_vec.split_off(new_pages_vec.len() - overflowed);
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

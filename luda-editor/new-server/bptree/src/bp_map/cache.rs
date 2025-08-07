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

pub(crate) type PagesMap = BTreeMap<PageRange, WithLastRefTime<PageBlock>>;
pub(crate) type CachedPages = Arc<PagesMap>;

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
    pub fn push(&self, new_pages: BTreeMap<PageRange, PageBlock>) -> Vec<(PageRange, PageBlock)> {
        if new_pages.is_empty() || self.limit < 1 {
            return new_pages.into_iter().collect();
        }

        let mut cache: BTreeMap<_, _> = self.inner.load_full().as_ref().clone();

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
    pub fn contains_key(&self, key: Key) -> Option<bool> {
        let guard = self.inner.load();
        let header = guard.get(&PageRange::HEADER)?.as_page().as_header();
        let mut node = guard
            .get(&PageRange::page(header.root_node_offset))?
            .as_page();

        loop {
            match node {
                Page::InternalNode(internal_node) => {
                    let child_offset = internal_node.find_child_offset_for(key);
                    node = guard.get(&PageRange::page(child_offset))?.as_page();
                }
                Page::LeafNode(leaf_node) => return Some(leaf_node.contains(key)),
                x => panic!("Unexpected page type: {x:?}"),
            }
        }
    }
    pub fn get(&self, key: Key) -> Option<Option<Bytes>> {
        let guard = self.inner.load();
        let header = guard.get(&PageRange::HEADER)?.as_page().as_header();
        let mut node = guard
            .get(&PageRange::page(header.root_node_offset))?
            .as_page();

        loop {
            match node {
                Page::InternalNode(internal_node) => {
                    let child_offset = internal_node.find_child_offset_for(key);
                    node = guard.get(&PageRange::page(child_offset))?.as_page();
                }
                Page::LeafNode(leaf_node) => {
                    let Some(record_page_range) = leaf_node.get_record_page_range(key) else {
                        return Some(None);
                    };
                    let bytes = guard.get(&record_page_range)?.as_record().content();
                    return Some(Some(bytes));
                }
                x => panic!("Unexpected page type: {x:?}"),
            }
        }
    }

    /// # Return
    ///
    ///   - `None` for cache miss.
    ///   - `Some(None)` if no more keys are available.
    pub(crate) fn next(&self, exclusive_start_key: Option<Key>) -> Option<Option<Vec<Entry>>> {
        let key = exclusive_start_key.unwrap_or_default();
        let guard = self.inner.load();
        let header = guard
            .get(&PageRange::page(PageOffset::HEADER))?
            .as_page()
            .as_header();
        let mut node = guard
            .get(&PageRange::page(header.root_node_offset))?
            .as_page();

        loop {
            match node {
                Page::InternalNode(internal_node) => {
                    let child_offset = internal_node.find_child_offset_for(key);
                    assert_ne!(child_offset, PageOffset::NULL);

                    node = guard.get(&PageRange::page(child_offset))?.as_page();
                }
                Page::LeafNode(leaf_node) => match leaf_node.next(exclusive_start_key) {
                    NextResult::Found { key_ranges } => {
                        let entries = key_ranges
                            .into_iter()
                            .map(|(key, record_page_page_range)| {
                                guard
                                    .get(&record_page_page_range)
                                    .map(|x| x.as_record().content())
                                    .map(|bytes| Entry { key, value: bytes })
                            })
                            .collect::<Option<Vec<_>>>()?;
                        return Some(Some(entries));
                    }
                    NextResult::NoMoreEntries => {
                        return Some(None);
                    }
                    NextResult::CheckRightNode { right_node_offset } => {
                        assert_ne!(right_node_offset, PageOffset::NULL);
                        node = guard.get(&PageRange::page(right_node_offset))?.as_page();
                        assert!(matches!(node, Page::LeafNode(_)));
                        continue;
                    }
                },
                x => panic!("Unexpected page type: {x:?}"),
            }
        }
    }

    pub(crate) fn load_full(&self) -> CachedPages {
        self.inner.load_full()
    }

    pub(crate) fn header(&self) -> Option<Header> {
        let guard = self.inner.load();
        Some(*guard.get(&PageRange::HEADER)?.as_page().as_header())
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

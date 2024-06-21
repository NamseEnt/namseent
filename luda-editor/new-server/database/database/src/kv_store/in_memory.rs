use super::KvStore;
use anyhow::Result;
use quick_cache::sync::Cache;
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant, SystemTime},
};

#[derive(Clone)]
pub(crate) struct InMemoryCachedKsStore<Store: KvStore + Clone> {
    store: Store,
    cache: Arc<Cache<String, Option<Cached>>>,
    enabled: Arc<AtomicBool>,
}
#[derive(Clone)]
struct Cached {
    value: Arc<Vec<u8>>,
    expired_at: Option<SystemTime>,
}
impl<Store: KvStore + Clone> InMemoryCachedKsStore<Store> {
    pub fn new(store: Store, enabled: bool) -> Self {
        Self {
            store,
            cache: Arc::new(Cache::new(8196)),
            enabled: Arc::new(AtomicBool::new(enabled)),
        }
    }
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }
    fn enabled(&self) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::Relaxed)
    }
}

// All of below methods are **not strongly consistent**.
// If you want strong consistency, create new one and put lock on that method.
impl<Store: KvStore + Clone> KvStore for InMemoryCachedKsStore<Store> {
    async fn get(&self, key: impl AsRef<str>) -> Result<Option<super::ValueBuffer>> {
        let key = key.as_ref().to_string();
        if !self.enabled() {
            return self.store.get(key).await;
        }

        if let Some(cached) = self.cache.get(&key) {
            let Some(cached) = cached else {
                return Ok(None);
            };

            if let Some(expired_at) = cached.expired_at {
                if expired_at < SystemTime::now() {
                    self.cache.insert(key, None);
                    return Ok(None);
                }
            }
            return Ok(Some(super::ValueBuffer::Arc(cached.value.clone())));
        }

        let stored = self.store.get_with_expiration(&key).await?;
        self.cache.insert(
            key,
            stored.as_ref().map(|(buffer, expired_at)| Cached {
                value: buffer.get_arc_vec(),
                expired_at: *expired_at,
            }),
        );

        Ok(stored.map(|(buffer, _)| buffer))
    }

    async fn get_with_expiration(
        &self,
        _key: impl AsRef<str>,
    ) -> Result<Option<(super::ValueBuffer, Option<SystemTime>)>> {
        todo!("Not implemented yet.")
    }

    async fn put(
        &self,
        key: impl AsRef<str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        self.store.put(key.as_ref(), value, ttl).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.insert(
            key.as_ref().to_string(),
            Some(Cached {
                value: value.as_ref().to_vec().into(),
                // expired_at would have mismatch between the cache and the store
                // because the timing of calling below `SystemTime::now()` is different to the store's `now` time.
                expired_at: ttl.map(|ttl| SystemTime::now() + ttl),
            }),
        );
        Ok(())
    }

    async fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        self.store.delete(key.as_ref()).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.remove(key.as_ref());

        Ok(())
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        key: impl AsRef<str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        self.store.create(key.as_ref(), value_fn, ttl).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.remove(key.as_ref());

        Ok(())
    }

    // async fn update<T, Fut>(
    //     &self,
    //     key: impl AsRef<str>,
    //     update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    // ) -> Result<bool>
    // where
    //     T: rkyv::Archive + rkyv::Serialize<AllocSerializer<64>>,
    //     Fut: Future<Output = Option<Option<T>>>,
    // {
    //     if !self.enabled() {
    //         return self.store.update(key, update).await;
    //     }

    //     let value = self.get(key.as_ref())?;
    //     let Some(update_result) = update(value).await else {
    //         return Ok(true);
    //     };

    //     if !self
    //         .store
    //         .update(key.as_ref(), |_| async move { Some(update_result) })
    //         .await?
    //     {
    //         return Ok(false);
    //     }
    //     self.cache.invalidate(key.as_ref());

    //     Ok(true)
    // }
}

use super::{HeapArchived, KvStore};
use anyhow::Result;
use moka::sync::Cache;
use rkyv::ser::serializers::AllocSerializer;
use std::{
    future::Future,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Clone)]
pub struct InMemoryCachedKsStore<Store: KvStore + Clone> {
    store: Store,
    cache: Cache<String, Option<Arc<Vec<u8>>>>,
    enabled: Arc<AtomicBool>,
}
impl<Store: KvStore + Clone> InMemoryCachedKsStore<Store> {
    pub fn new_as_disabled(store: Store) -> Self {
        Self {
            store,
            cache: Cache::new(8196),
            enabled: Arc::new(AtomicBool::new(false)),
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
    fn get<T: rkyv::Archive>(&self, key: impl AsRef<str>) -> Result<Option<HeapArchived<T>>> {
        if !self.enabled() {
            return self.store.get(key);
        }

        if let Some(buffer) = self.cache.get(key.as_ref()) {
            return Ok(buffer
                .as_ref()
                .map(|buffer| HeapArchived::new(buffer.clone())));
        }

        let stored = self.store.get::<T>(key.as_ref())?;
        self.cache.insert(
            key.as_ref().to_string(),
            stored.as_ref().map(|stored| stored.get_arc_vec()),
        );

        Ok(stored)
    }

    fn put<T: rkyv::Serialize<AllocSerializer<0>>>(
        &self,
        key: impl AsRef<str>,
        value: &T,
    ) -> Result<()> {
        if !self.enabled() {
            return self.store.put(key, value);
        }
        self.store.put(key.as_ref(), value)?;
        let buffer = rkyv::to_bytes(value).unwrap();
        self.cache
            .insert(key.as_ref().to_string(), Some(Arc::new(buffer.to_vec())));
        Ok(())
    }

    fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        if !self.enabled() {
            return self.store.delete(key);
        }
        self.store.delete(key.as_ref())?;
        self.cache.invalidate(key.as_ref());

        Ok(())
    }

    async fn update<T, Fut>(
        &self,
        key: impl AsRef<str>,
        update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    ) -> Result<bool>
    where
        T: rkyv::Archive + rkyv::Serialize<AllocSerializer<0>>,
        Fut: Future<Output = Option<Option<T>>>,
    {
        if !self.enabled() {
            return self.store.update(key, update).await;
        }

        let value = self.get(key.as_ref())?;
        let Some(update_result) = update(value).await else {
            return Ok(true);
        };

        if !self
            .store
            .update(key.as_ref(), |_| async move { Some(update_result) })
            .await?
        {
            return Ok(false);
        }
        self.cache.invalidate(key.as_ref());

        Ok(true)
    }
}

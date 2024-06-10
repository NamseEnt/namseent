use super::KvStore;
use anyhow::Result;
use quick_cache::sync::Cache;
use std::sync::{atomic::AtomicBool, Arc};

#[derive(Clone)]
pub(crate) struct InMemoryCachedKsStore<Store: KvStore + Clone> {
    store: Store,
    cache: Arc<Cache<String, Option<Arc<Vec<u8>>>>>,
    enabled: Arc<AtomicBool>,
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
        if !self.enabled() {
            return self.store.get(key).await;
        }

        if let Some(buffer) = self.cache.get(key.as_ref()) {
            return Ok(buffer
                .as_ref()
                .map(|buffer| super::ValueBuffer::Arc(buffer.clone())));
        }

        let stored = self.store.get(key.as_ref()).await?;
        self.cache.insert(
            key.as_ref().to_string(),
            stored.as_ref().map(|stored| stored.get_arc_vec()),
        );

        Ok(stored)
    }

    async fn put(&self, key: impl AsRef<str>, value: &impl AsRef<[u8]>) -> Result<()> {
        self.store.put(key.as_ref(), value).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.insert(
            key.as_ref().to_string(),
            Some(Arc::new(value.as_ref().to_vec())),
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
    ) -> Result<()> {
        self.store.create(key.as_ref(), value_fn).await?;
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

use super::*;
use crate::Result;
use document::{TransactItems, ValueBuffer};
use quick_cache::{sync::Cache, Equivalent};
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, SystemTime},
};

#[derive(Clone)]
pub(crate) struct InMemoryCachedKsStore<Store: DocumentStore + Clone> {
    store: Store,
    cache: Arc<Cache<CacheKey, Option<Cached>>>,
    enabled: Arc<AtomicBool>,
}
#[derive(Clone)]
struct Cached {
    value: Arc<Vec<u8>>,
    expired_at: Option<SystemTime>,
}
impl<Store: DocumentStore + Clone> InMemoryCachedKsStore<Store> {
    pub fn new(store: Store, enabled: bool) -> Self {
        Self {
            store,
            cache: Arc::new(Cache::new(8196)),
            enabled: Arc::new(AtomicBool::new(enabled)),
        }
    }
    pub fn set_cache_enabled(&self, enabled: bool) {
        self.enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }
    fn enabled(&self) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey {
    name: String,
    pk: Vec<u8>,
    sk: Option<Vec<u8>>,
}

impl Equivalent<CacheKey> for (&str, &[u8], Option<&[u8]>) {
    fn equivalent(&self, key: &CacheKey) -> bool {
        self.0 == key.name && self.1 == key.pk && self.2 == key.sk.as_deref()
    }
}

// All of below methods are **not strongly consistent**.
// If you want strong consistency, create new one and put lock on that method.
impl<Store: DocumentStore + Clone> DocumentStore for InMemoryCachedKsStore<Store> {
    async fn get(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<ValueBuffer>> {
        if !self.enabled() {
            return self.store.get(name, pk, sk).await;
        }

        if let Some(cached) = self.cache.get(&(name, pk, sk)) {
            let Some(cached) = cached else {
                return Ok(None);
            };

            if let Some(expired_at) = cached.expired_at {
                if expired_at < SystemTime::now() {
                    self.cache.insert(
                        CacheKey {
                            name: name.to_string(),
                            pk: pk.to_vec(),
                            sk: sk.map(|sk| sk.to_vec()),
                        },
                        None,
                    );
                    return Ok(None);
                }
            }
            return Ok(Some(ValueBuffer::Arc(cached.value.clone())));
        }

        let stored = self.store.get_with_expiration(name, pk, sk).await?;
        self.cache.insert(
            CacheKey {
                name: name.to_string(),
                pk: pk.to_vec(),
                sk: sk.map(|sk| sk.to_vec()),
            },
            stored.as_ref().map(|(buffer, expired_at)| Cached {
                value: buffer.get_arc_vec(),
                expired_at: *expired_at,
            }),
        );

        Ok(stored.map(|(buffer, _)| buffer))
    }

    async fn get_with_expiration(
        &self,
        _name: &'static str,
        _pk: &[u8],
        _sk: Option<&[u8]>,
    ) -> Result<Option<(ValueBuffer, Option<SystemTime>)>> {
        todo!("Not implemented yet.")
    }

    /// Query is not cached.
    async fn query(&self, name: &'static str, pk: &[u8]) -> Result<Vec<ValueBuffer>> {
        self.store.query(name, pk).await
    }

    async fn put(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        self.store.put(name, pk, sk, value, ttl).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.insert(
            CacheKey {
                name: name.to_string(),
                pk: pk.to_vec(),
                sk: sk.map(|sk| sk.to_vec()),
            },
            Some(Cached {
                value: value.as_ref().to_vec().into(),
                // expired_at would have mismatch between the cache and the store
                // because the timing of calling below `SystemTime::now()` is different to the store's `now` time.
                expired_at: ttl.map(|ttl| SystemTime::now() + ttl),
            }),
        );
        Ok(())
    }

    async fn delete(&self, name: &'static str, pk: &[u8], sk: Option<&[u8]>) -> Result<()> {
        self.store.delete(name, pk, sk).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.remove(&(name, pk, sk));

        Ok(())
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        self.store.create(name, pk, sk, value_fn, ttl).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.remove(&(name, pk, sk));

        Ok(())
    }

    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>> {
        let result = self.store.transact(transact_items).await;
        if !self.enabled() {
            return result;
        }

        transact_items.iter().for_each(|item| match item {
            TransactItem::Put { name, pk, sk, .. }
            | TransactItem::Create { name, pk, sk, .. }
            | TransactItem::Update { name, pk, sk, .. }
            | TransactItem::Delete { name, pk, sk } => {
                self.cache.remove(&(*name, pk.as_ref(), sk.as_deref()));
            }
        });

        result
    }

    async fn wait_backup(&self) -> Result<()> {
        self.store.wait_backup().await
    }

    // async fn update<T, Fut>(
    //     &self,
    //     key: &str,
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

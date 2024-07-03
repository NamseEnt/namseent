use super::DocumentStore;
use crate::Result;
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
    pk: String,
    sk: Option<String>,
}

impl Equivalent<CacheKey> for (&str, &str, Option<&str>) {
    fn equivalent(&self, key: &CacheKey) -> bool {
        self.0 == key.name && self.1 == key.pk && self.2 == key.sk.as_deref()
    }
}

// All of below methods are **not strongly consistent**.
// If you want strong consistency, create new one and put lock on that method.
impl<Store: DocumentStore + Clone> DocumentStore for InMemoryCachedKsStore<Store> {
    async fn get(
        &self,
        name: &str,
        pk: &str,
        sk: Option<&str>,
    ) -> Result<Option<super::ValueBuffer>> {
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
                            pk: pk.to_string(),
                            sk: sk.map(|sk| sk.to_string()),
                        },
                        None,
                    );
                    return Ok(None);
                }
            }
            return Ok(Some(super::ValueBuffer::Arc(cached.value.clone())));
        }

        let stored = self.store.get_with_expiration(name, pk, sk).await?;
        self.cache.insert(
            CacheKey {
                name: name.to_string(),
                pk: pk.to_string(),
                sk: sk.map(|sk| sk.to_string()),
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
        _name: &str,
        _pk: &str,
        _sk: Option<&str>,
    ) -> Result<Option<(super::ValueBuffer, Option<SystemTime>)>> {
        todo!("Not implemented yet.")
    }

    async fn put(
        &self,
        name: &str,
        pk: &str,
        sk: Option<&str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let pk = pk.as_ref();
        let sk = sk.map(|sk| sk.as_ref());
        self.store.put(name, pk, sk, value, ttl).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.insert(
            CacheKey {
                name: name.to_string(),
                pk: pk.to_string(),
                sk: sk.map(|sk| sk.to_string()),
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

    async fn delete(&self, name: &str, pk: &str, sk: Option<&str>) -> Result<()> {
        let pk = pk.as_ref();
        let sk = sk.map(|sk| sk.as_ref());
        self.store.delete(name, pk, sk).await?;
        if !self.enabled() {
            return Ok(());
        }
        self.cache.remove(&(name, pk, sk));

        Ok(())
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        name: &str,
        pk: &str,
        sk: Option<&str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()> {
        self.store.create(name, pk, sk, value_fn, ttl).await?;
        self.cache.remove(&(name, pk, sk));

        Ok(())
    }

    async fn transact(
        &self,
        transact_items: impl IntoIterator<Item = document::TransactItem>,
    ) -> Result<()> {
        let transact_items = transact_items.into_iter().collect::<Vec<_>>();
        let keys = transact_items
            .iter()
            .map(|item| match item {
                document::TransactItem::Put { name, pk, sk, .. } => {
                    (name.clone(), pk.clone(), sk.clone())
                }
                document::TransactItem::Create { name, pk, sk, .. } => {
                    (name.clone(), pk.clone(), sk.clone())
                }
                document::TransactItem::Delete { name, pk, sk } => {
                    (name.clone(), pk.clone(), sk.clone())
                }
            })
            .collect::<Vec<_>>();

        self.store.transact(transact_items).await?;
        for (name, pk, sk) in keys {
            self.cache
                .remove(&(name.as_str(), pk.as_str(), sk.as_deref()));
        }

        Ok(())
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

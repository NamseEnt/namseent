use crate::*;
use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex, OnceLock},
};

pub struct SerdeLruCache<Key: serde::Serialize, Value, const CAPACITY: usize = 1024> {
    map: OnceLock<Mutex<lru::LruCache<SerdeHash<Key>, Arc<Value>>>>,
}

impl<Key: serde::Serialize, Value, const CAPACITY: usize> SerdeLruCache<Key, Value, CAPACITY> {
    pub const fn new() -> Self {
        Self {
            map: OnceLock::new(),
        }
    }

    pub fn get_or_create(&self, key: &Key, create: impl FnOnce(&Key) -> Value) -> Arc<Value> {
        let map = self
            .map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())));

        let mut map = map.lock().unwrap();
        let hash_key = SerdeHash::new(key);

        map.get_or_insert(hash_key, || Arc::new(create(key)))
            .clone()
    }

    pub fn get_or_try_create(
        &self,
        key: &Key,
        try_create: impl FnOnce(&Key) -> Option<Value>,
    ) -> Option<Arc<Value>> {
        let map = self
            .map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())));

        let mut map = map.lock().unwrap();
        let hash_key = SerdeHash::new(key);

        match map.try_get_or_insert(hash_key, || {
            let value = try_create(key);
            match value {
                Some(value) => Ok(Arc::new(value)),
                None => Err(()),
            }
        }) {
            Ok(value) => Some(value.clone()),
            Err(_) => None,
        }
    }
}

impl<Key: serde::Serialize, Value, const CAPACITY: usize> Default
    for SerdeLruCache<Key, Value, CAPACITY>
{
    fn default() -> Self {
        Self::new()
    }
}

use std::{
    hash::Hash,
    num::NonZeroUsize,
    sync::{Arc, Mutex, OnceLock},
};

pub struct LruCache<Key: Hash + Eq + Clone, Value, const CAPACITY: usize = 1024> {
    map: OnceLock<Mutex<lru::LruCache<Key, Arc<Value>>>>,
}

impl<Key: Hash + Eq + Clone, Value, const CAPACITY: usize> LruCache<Key, Value, CAPACITY> {
    pub const fn new() -> Self {
        Self {
            map: OnceLock::new(),
        }
    }

    pub fn get_or_create(&self, key: &Key, create: impl FnOnce(&Key) -> Value) -> Arc<Value> {
        let map = self
            .map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())));

        {
            let mut map = map.lock().unwrap();
            if let Some(value) = map.get(key) {
                return value.clone();
            }
        }

        let created = Arc::new(create(key));

        let mut map = map.lock().unwrap();
        if let Some(value) = map.get(key) {
            value.clone()
        } else {
            map.put(key.clone(), created.clone());
            created
        }
    }

    pub fn get_or_try_create(
        &self,
        key: &Key,
        try_create: impl FnOnce(&Key) -> Option<Value>,
    ) -> Option<Arc<Value>> {
        let map = self
            .map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())));

        {
            let mut map = map.lock().unwrap();
            if let Some(value) = map.get(key) {
                return Some(value.clone());
            }
        }

        let created = Arc::new(try_create(key)?);

        let mut map = map.lock().unwrap();
        if let Some(value) = map.get(key) {
            Some(value.clone())
        } else {
            map.put(key.clone(), created.clone());
            Some(created)
        }
    }

    pub fn get(&self, key: &Key) -> Option<Arc<Value>> {
        let map = self
            .map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())));

        let mut map = map.lock().unwrap();

        map.get(key).cloned()
    }

    pub fn put(&self, key: Key, value: Value) {
        let map = self
            .map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())));

        let mut map = map.lock().unwrap();

        map.put(key, Arc::new(value));
    }
}

impl<Key: Hash + Eq + Clone, Value, const CAPACITY: usize> Default
    for LruCache<Key, Value, CAPACITY>
{
    fn default() -> Self {
        Self::new()
    }
}

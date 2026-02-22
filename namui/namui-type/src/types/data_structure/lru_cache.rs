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

    fn map_mutex(&self) -> &Mutex<lru::LruCache<Key, Arc<Value>>> {
        self.map
            .get_or_init(|| Mutex::new(lru::LruCache::new(NonZeroUsize::new(CAPACITY).unwrap())))
    }

    fn with_map<R>(&self, f: impl FnOnce(&mut lru::LruCache<Key, Arc<Value>>) -> R) -> R {
        let mut map = self.map_mutex().lock().unwrap();
        f(&mut map)
    }

    pub fn get_or_create(&self, key: &Key, create: impl FnOnce(&Key) -> Value) -> Arc<Value> {
        self.get_or_try_create(key, |key| Some(create(key)))
            .expect("get_or_create closure must always return a value")
    }

    pub fn get_or_try_create(
        &self,
        key: &Key,
        try_create: impl FnOnce(&Key) -> Option<Value>,
    ) -> Option<Arc<Value>> {
        if let Some(value) = self.with_map(|map| map.get(key).cloned()) {
            return Some(value);
        }

        let created = Arc::new(try_create(key)?);

        Some(self.with_map(|map| {
            if let Some(value) = map.get(key) {
                value.clone()
            } else {
                map.put(key.clone(), created.clone());
                created
            }
        }))
    }

    pub fn get(&self, key: &Key) -> Option<Arc<Value>> {
        self.with_map(|map| map.get(key).cloned())
    }

    pub fn put(&self, key: Key, value: Value) {
        self.with_map(|map| {
            map.put(key, Arc::new(value));
        });
    }
}

impl<Key: Hash + Eq + Clone, Value, const CAPACITY: usize> Default
    for LruCache<Key, Value, CAPACITY>
{
    fn default() -> Self {
        Self::new()
    }
}

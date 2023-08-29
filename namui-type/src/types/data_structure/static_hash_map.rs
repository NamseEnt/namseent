use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex, OnceLock},
};

pub struct StaticHashMap<Key: Hash + Eq + PartialEq, Value> {
    map: OnceLock<Mutex<HashMap<Key, Arc<Value>>>>,
}

impl<Key: Hash + Eq + PartialEq, Value> StaticHashMap<Key, Value> {
    pub const fn new() -> Self {
        Self {
            map: OnceLock::new(),
        }
    }

    pub fn get_or_create(&self, key: Key, create: impl FnOnce(&Key) -> Value) -> Arc<Value> {
        let map = self.map.get_or_init(|| Default::default());

        let mut map = map.lock().unwrap();
        map.entry(key)
            .or_insert_with_key(|key| Arc::new(create(key)))
            .clone()
    }

    pub fn get_or_try_create(
        &self,
        key: Key,
        try_crate: impl Fn(&Key) -> Option<Value>,
    ) -> Option<Arc<Value>> {
        let map = self.map.get_or_init(|| Default::default());
        let mut map = map.lock().unwrap();

        match map.get(&key) {
            Some(value) => Some(value.clone()),
            None => match try_crate(&key) {
                Some(value) => {
                    let value = Arc::new(value);
                    map.insert(key, value.clone());
                    Some(value)
                }
                None => None,
            },
        }
    }

    pub fn insert(&self, key: Key, value: Value) {
        let map = self.map.get_or_init(|| Default::default());
        let mut map = map.lock().unwrap();
        map.insert(key, Arc::new(value));
    }

    pub fn get(&self, key: &Key) -> Option<Arc<Value>> {
        let map = self.map.get_or_init(|| Default::default());
        let map = map.lock().unwrap();
        map.get(key).cloned()
    }
}

impl<Key: Hash + Eq + PartialEq, Value> Default for StaticHashMap<Key, Value> {
    fn default() -> Self {
        Self::new()
    }
}

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

pub struct SerdeMap<Key: serde::Serialize, Value> {
    _key: std::marker::PhantomData<Key>,
    map: OnceLock<Mutex<HashMap<Vec<u8>, Arc<Value>>>>,
}

impl<Key: serde::Serialize, Value> SerdeMap<Key, Value> {
    pub const fn new() -> Self {
        Self {
            _key: std::marker::PhantomData,
            map: OnceLock::new(),
        }
    }

    pub fn get_or_create(&self, key: &Key, create: impl FnOnce(&Key) -> Value) -> Arc<Value> {
        let map = self.map.get_or_init(|| Default::default());

        let mut map = map.lock().unwrap();
        let serde_key = postcard::to_allocvec(&key).unwrap();
        map.entry(serde_key)
            .or_insert_with_key(|_| Arc::new(create(key)))
            .clone()
    }

    pub fn get_or_try_create(
        &self,
        key: &Key,
        try_crate: impl Fn(&Key) -> Option<Value>,
    ) -> Option<Arc<Value>> {
        let map = self.map.get_or_init(|| Default::default());
        let mut map = map.lock().unwrap();
        let serde_key = postcard::to_allocvec(&key).unwrap();

        match map.get(&serde_key) {
            Some(value) => Some(value.clone()),
            None => match try_crate(key) {
                Some(value) => {
                    let value = Arc::new(value);
                    map.insert(serde_key, value.clone());
                    Some(value)
                }
                None => None,
            },
        }
    }

    pub fn insert(&self, key: &Key, value: Value) {
        let map = self.map.get_or_init(|| Default::default());
        let mut map = map.lock().unwrap();
        let serde_key = postcard::to_allocvec(&key).unwrap();
        map.insert(serde_key, Arc::new(value));
    }

    pub fn get(&self, key: &Key) -> Option<Arc<Value>> {
        let map = self.map.get_or_init(|| Default::default());
        let map = map.lock().unwrap();
        let serde_key = postcard::to_allocvec(&key).unwrap();
        map.get(&serde_key).cloned()
    }
}

impl<Key: serde::Serialize, Value> Default for SerdeMap<Key, Value> {
    fn default() -> Self {
        Self::new()
    }
}

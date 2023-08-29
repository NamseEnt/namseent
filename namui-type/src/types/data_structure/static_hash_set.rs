use std::{
    collections::HashSet,
    hash::Hash,
    sync::{Mutex, OnceLock},
};

pub struct StaticHashSet<Key: Hash + Eq + PartialEq> {
    map: OnceLock<Mutex<HashSet<Key>>>,
}

impl<Key: Hash + Eq + PartialEq> StaticHashSet<Key> {
    pub const fn new() -> Self {
        Self {
            map: OnceLock::new(),
        }
    }
    pub fn contains(&self, key: &Key) -> bool {
        self.map
            .get_or_init(|| Default::default())
            .lock()
            .unwrap()
            .contains(key)
    }
    pub fn insert(&self, key: Key) -> bool {
        self.map
            .get_or_init(|| Default::default())
            .lock()
            .unwrap()
            .insert(key)
    }
    pub fn remove(&self, key: &Key) -> bool {
        self.map
            .get_or_init(|| Default::default())
            .lock()
            .unwrap()
            .remove(key)
    }
}

impl<Key: Hash + Eq + PartialEq> Default for StaticHashSet<Key> {
    fn default() -> Self {
        Self::new()
    }
}

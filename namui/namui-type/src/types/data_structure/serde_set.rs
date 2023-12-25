use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

pub struct SerdeSet<Key: serde::Serialize + PartialEq> {
    _key: std::marker::PhantomData<Key>,
    set: OnceLock<Mutex<HashSet<Vec<u8>>>>,
}

impl<Key: serde::Serialize + PartialEq> SerdeSet<Key> {
    pub const fn new() -> Self {
        Self {
            _key: std::marker::PhantomData,
            set: OnceLock::new(),
        }
    }
    pub fn contains(&self, key: &Key) -> bool {
        self.set
            .get_or_init(Default::default)
            .lock()
            .unwrap()
            .contains(&postcard::to_allocvec(key).unwrap())
    }
    pub fn insert(&self, key: &Key) -> bool {
        self.set
            .get_or_init(Default::default)
            .lock()
            .unwrap()
            .insert(postcard::to_allocvec(key).unwrap())
    }
    pub fn remove(&self, key: &Key) -> bool {
        self.set
            .get_or_init(Default::default)
            .lock()
            .unwrap()
            .remove(&postcard::to_allocvec(key).unwrap())
    }
}

impl<Key: serde::Serialize + PartialEq> Default for SerdeSet<Key> {
    fn default() -> Self {
        Self::new()
    }
}

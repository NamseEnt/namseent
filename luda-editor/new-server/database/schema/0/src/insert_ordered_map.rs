use crate::*;
use rkyv::Archive;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[doc_part]
struct InsertOrderedMap<K, V>
where
    K: Debug + Archive + Eq + Hash + Clone,
    <K as Archive>::Archived: Debug + Eq + Hash,
    V: Debug + Archive,
    <V as Archive>::Archived: Debug,
    <HashMap<K, V> as Archive>::Archived: Debug,
    <Vec<K> as Archive>::Archived: Debug,
{
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> InsertOrderedMap<K, V>
where
    K: Debug + Archive + Eq + Hash + Clone,
    <K as Archive>::Archived: Debug + Eq + Hash,
    V: Debug + Archive,
    <V as Archive>::Archived: Debug,
    <HashMap<K, V> as Archive>::Archived: Debug,
    <Vec<K> as Archive>::Archived: Debug,
{
    pub fn into_values(self) -> impl Iterator<Item = V> {
        let mut map = self.map;
        self.order
            .into_iter()
            .map(move |key| map.remove(&key).unwrap())
    }
    pub fn len(&self) -> usize {
        self.order.len()
    }
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
    pub fn insert(&mut self, index: usize, key: K, value: V) -> Option<V> {
        self.order.insert(index, key.clone());
        self.map.insert(key, value)
    }
    pub fn update_by_key(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }
    pub fn remove_by_key(&mut self, key: &K) -> Option<V> {
        self.order.retain(|k| k != key);
        self.map.remove(key)
    }
    pub fn get_mut_by_key(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }
}

impl<K, V> ArchivedInsertOrderedMap<K, V>
where
    K: Debug + Archive + Eq + Hash + Clone,
    <K as Archive>::Archived: Debug + Eq + Hash,
    V: Debug + Archive,
    <V as Archive>::Archived: Debug,
    <HashMap<K, V> as Archive>::Archived: Debug,
    <Vec<K> as Archive>::Archived: Debug,
{
    pub fn len(&self) -> usize {
        self.order.len()
    }
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
    pub fn contains_key(&self, key: &K::Archived) -> bool {
        self.map.contains_key(key)
    }
}

impl<K, V> Default for InsertOrderedMap<K, V>
where
    K: Debug + Archive + Eq + Hash + Clone,
    <K as Archive>::Archived: Debug + Eq + Hash,
    V: Debug + Archive,
    <V as Archive>::Archived: Debug,
    <HashMap<K, V> as Archive>::Archived: Debug,
    <Vec<K> as Archive>::Archived: Debug,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            order: Default::default(),
        }
    }
}

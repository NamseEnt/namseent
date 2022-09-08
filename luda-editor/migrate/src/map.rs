use crate::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Map<T: History> {
    #[serde(flatten)]
    items: BTreeMap<String, T>,
    #[serde(skip, default = "Vec::new")]
    commands: Vec<Command<T>>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
enum Command<T: History> {
    Insert(String, T),
}

impl<T: History> Map<T> {
    pub fn new(items: impl IntoIterator<Item = (String, T)>) -> Self {
        Map {
            items: BTreeMap::from_iter(items.into_iter().map(|(key, value)| (key, value))),
            commands: Vec::new(),
        }
    }
    pub fn set(&mut self, key: impl AsRef<str>, value: T) {
        self.commands
            .push(Command::Insert(key.as_ref().to_string(), value));
    }
    pub fn get(&self, key: impl AsRef<str>) -> Option<&T> {
        self.items.get(key.as_ref())
    }
    pub fn values(&self) -> impl ExactSizeIterator<Item = &T> {
        self.items.values()
    }
}

impl<T: History> History for Map<T> {
    fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
        array.insert(txn, index, yrs::PrelimMap::<bool>::new());
        let mut map = array.get(index).unwrap().to_ymap().unwrap();
        for (key, item) in self.items.into_iter() {
            item.insert_to_map(txn, &mut map, key);
        }
    }
    fn insert_to_map(
        self,
        txn: &mut yrs::Transaction,
        map: &yrs::Map,
        key: impl Into<std::rc::Rc<str>>,
    ) {
        let key: std::rc::Rc<str> = key.into();
        map.insert(txn, key.clone(), yrs::PrelimMap::<bool>::new());
        let mut map = map.get(key.as_ref()).unwrap().to_ymap().unwrap();
        for (key, item) in self.items.into_iter() {
            item.insert_to_map(txn, &mut map, key);
        }
    }
    fn insert_to_root(self, txn: &mut yrs::Transaction) {
        let mut root = txn.get_map("root");
        for (key, item) in self.items.into_iter() {
            item.insert_to_map(txn, &mut root, key);
        }
    }
    fn from_map(_root: &yrs::Map) -> Self {
        unreachable!()
    }
    fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
        unreachable!()
    }
    fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map) {
        for command in self.commands {
            match command {
                Command::Insert(key, value) => value.insert_to_map(txn, head, key),
            };
        }
    }
    fn from_value(_value: crate::Value) -> Self {
        unreachable!()
    }
    fn get_version() -> Option<u32> {
        unreachable!()
    }
    fn migrate(_version_of_doc: u32, _doc: yrs::Doc) -> Self {
        unreachable!()
    }
    fn as_value(&self) -> crate::Value {
        unreachable!()
    }
}

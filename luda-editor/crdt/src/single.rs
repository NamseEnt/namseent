use crate::*;

/// The reason why I made this is I cannot determine that any struct implemented History
/// So I couldn't branch them when insert to array or map.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Single<T: History> {
    #[serde(flatten)]
    item: T,
}

impl<T: History> Single<T> {
    pub fn new(item: T) -> Self {
        Single { item }
    }
}

impl<T: History> std::ops::Deref for Single<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: History> std::ops::DerefMut for Single<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<T: History> History for Single<T> {
    fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
        self.item.insert_to_array(txn, array, index);
    }
    fn insert_to_map(
        self,
        txn: &mut yrs::Transaction,
        map: &yrs::Map,
        key: impl Into<std::rc::Rc<str>>,
    ) {
        self.item.insert_to_map(txn, map, key);
    }
    fn insert_to_root(self, txn: &mut yrs::Transaction) {
        self.item.insert_to_root(txn)
    }
    fn from_map(root: &yrs::Map) -> Self {
        let item = T::from_map(root);
        Self::new(item)
    }
    fn update_to_array(self, txn: &mut yrs::Transaction, head: &yrs::Array) {
        self.item.update_to_array(txn, head)
    }
    fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map) {
        self.item.update_to_map(txn, head)
    }
    fn from_value(value: crate::Value) -> Self {
        let item = T::from_value(value);
        Self::new(item)
    }
    fn get_version() -> Option<u32> {
        unreachable!()
    }
    fn migrate(_version_of_doc: u32, _doc: yrs::Doc) -> Self {
        unreachable!()
    }
    fn as_value(&self) -> crate::Value {
        self.item.as_value()
    }
}

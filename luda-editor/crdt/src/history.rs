pub trait History: Send + Sync + Clone {
    fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32);
    fn insert_to_map(
        self,
        txn: &mut yrs::Transaction,
        map: &yrs::Map,
        key: impl Into<std::rc::Rc<str>>,
    );
    fn insert_to_root(self, txn: &mut yrs::Transaction);
    fn from_map(root: &yrs::Map) -> Self;
    fn update_to_array(self, txn: &mut yrs::Transaction, head: &yrs::Array);
    fn update_to_map(self, txn: &mut yrs::Transaction, head: &yrs::Map);
    fn from_value(value: crate::Value) -> Self;
    fn as_value(&self) -> crate::Value;
    fn get_version() -> Option<u32>;
    fn migrate(version_of_doc: u32, doc: yrs::Doc) -> Self;
}

macro_rules! impl_history_for_primitive {
    ($type: ty) => {
        impl History for $type {
            fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
                array.insert(txn, index, self);
            }

            fn insert_to_map(
                self,
                txn: &mut yrs::Transaction,
                map: &yrs::Map,
                key: impl Into<std::rc::Rc<str>>,
            ) {
                map.insert(txn, key, self);
            }

            fn insert_to_root(self, _txn: &mut yrs::Transaction) {
                unreachable!()
            }

            fn from_map(_root: &yrs::Map) -> Self {
                unreachable!()
            }
            fn update_to_array(self, _txn: &mut yrs::Transaction, _head: &yrs::Array) {
                unreachable!()
            }
            fn update_to_map(self, _txn: &mut yrs::Transaction, _head: &yrs::Map) {
                unreachable!()
            }
            fn from_value(value: crate::Value) -> Self {
                value.into()
            }
            fn as_value(&self) -> crate::Value {
                self.into()
            }
            fn get_version() -> Option<u32> {
                None
            }
            fn migrate(_version_of_doc: u32, _doc: yrs::Doc) -> Self {
                unreachable!()
            }
        }
    };
}

impl_history_for_primitive!(i32);
impl_history_for_primitive!(String);

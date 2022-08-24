use crate::*;
use lib0::any::Any;

#[derive(Debug, Clone, PartialEq)]
pub struct List<T: History> {
    items: Vec<T>,
    commands: Vec<Command<T>>,
}

#[derive(Debug, Clone, PartialEq)]
enum Command<T: History> {
    Push(T),
    Remove(u32),
    Update(u32, T),
}

impl<T: History> List<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        List {
            items: items.into_iter().collect(),
            commands: Vec::new(),
        }
    }
    pub fn push(&mut self, value: T) {
        self.commands.push(Command::Push(value));
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> {
        self.items.iter()
    }
    pub fn remove(&mut self, index: usize) {
        self.commands.push(Command::Remove(index as u32));
    }
    pub fn update(&mut self, index: usize, callback: impl FnOnce(&mut T)) {
        let item = self.items.get_mut(index).unwrap();
        callback(item);
        self.commands
            .push(Command::Update(index as u32, item.clone()));
    }
}

impl<T: History> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        List {
            items: iter.into_iter().collect(),
            commands: Vec::new(),
        }
    }
}

impl<T: History> History for List<T> {
    fn insert_to_array(self, txn: &mut yrs::Transaction, array: &yrs::Array, index: u32) {
        array.insert(txn, index, yrs::PrelimArray::<_, Any>::from([]));
        let mut array = array.get(index).unwrap().to_yarray().unwrap();
        for (index, item) in self.items.into_iter().enumerate() {
            item.insert_to_array(txn, &mut array, index as u32);
        }
    }
    fn insert_to_map(
        self,
        txn: &mut yrs::Transaction,
        map: &yrs::Map,
        key: impl Into<std::rc::Rc<str>>,
    ) {
        let key: std::rc::Rc<str> = key.into();
        map.insert(txn, key.clone(), yrs::PrelimArray::<_, Any>::from([]));
        let mut array = map.get(&key).unwrap().to_yarray().unwrap();
        for (index, item) in self.items.into_iter().enumerate() {
            item.insert_to_array(txn, &mut array, index as u32);
        }
    }
    fn insert_to_root(self, _txn: &mut yrs::Transaction) {
        unreachable!()
    }
    fn from_map(_root: &yrs::Map) -> Self {
        unreachable!()
    }
    fn update_to_array(self, txn: &mut yrs::Transaction, head: &yrs::Array) {
        for command in self.commands {
            match command {
                Command::Push(item) => item.insert_to_array(txn, head, head.len()),
                Command::Remove(index) => head.remove(txn, index),
                Command::Update(index, item) => {
                    let prev_value = head.get(index).unwrap();
                    match &prev_value {
                        yrs::types::Value::Any(_) => {
                            let item_value = item.as_value();
                            if prev_value == item_value.yvalue {
                                continue;
                            }

                            head.remove(txn, index);
                            item.insert_to_array(txn, head, index);
                        }
                        yrs::types::Value::YArray(array) => item.update_to_array(txn, array),
                        yrs::types::Value::YMap(map) => item.update_to_map(txn, map),
                        yrs::types::Value::YText(_)
                        | yrs::types::Value::YXmlElement(_)
                        | yrs::types::Value::YXmlText(_) => unreachable!(),
                    }
                }
            }
        }
    }
    fn update_to_map(self, _txn: &mut yrs::Transaction, _head: &yrs::Map) {
        unreachable!()
    }
    fn from_value(_value: crate::Value) -> Self {
        unreachable!()
    }
    fn as_value(&self) -> crate::Value {
        unreachable!()
    }
    fn get_version() -> Option<u32> {
        unreachable!()
    }
    fn migrate(_version_of_doc: u32, _doc: yrs::Doc) -> Self {
        unreachable!()
    }
}

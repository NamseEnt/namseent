use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct SerdeHash<T: serde::Serialize> {
    _value: std::marker::PhantomData<T>,
    hash: u64,
}

impl<T: serde::Serialize> SerdeHash<T> {
    pub fn new<'a>(value: &'a T) -> Self {
        let vec = postcard::to_stdvec(&value).unwrap();
        let mut hasher = DefaultHasher::new();
        hasher.write(&vec);
        hasher.finish();

        Self {
            _value: std::marker::PhantomData,
            hash: hasher.finish(),
        }
    }
}

impl<T: serde::Serialize> Hash for SerdeHash<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl<T: serde::Serialize> PartialEq for SerdeHash<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<T: serde::Serialize> Eq for SerdeHash<T> {}

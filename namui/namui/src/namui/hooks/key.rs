use namui_type::Uuid;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyVec {
    front: Option<Arc<KeyVec>>,
    chunk: KeyChunk,
}
impl KeyVec {
    pub(crate) fn new_child(key: impl Into<Key>) -> Self {
        KeyVec {
            front: None,
            chunk: KeyChunk::Child(key.into()),
        }
    }
    pub(crate) fn child(&self, key: impl Into<Key>) -> KeyVec {
        KeyVec {
            front: Some(Arc::new(self.clone())),
            chunk: KeyChunk::Child(key.into()),
        }
    }
    pub(crate) fn custom_key(&self, key: impl Into<Key>) -> KeyVec {
        KeyVec {
            front: Some(Arc::new(self.clone())),
            chunk: KeyChunk::Custom(key.into()),
        }
    }
}

impl Debug for KeyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_vec = self;
        let mut keys = vec![];
        while let Some(front) = &key_vec.front {
            keys.push(&key_vec.chunk);
            key_vec = front;
        }
        keys.push(&key_vec.chunk);
        write!(f, "{:?}", keys)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum KeyChunk {
    Child(Key),
    Custom(Key),
}

impl Debug for KeyChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyChunk::Child(key) => write!(f, "Child->{:?}", key),
            KeyChunk::Custom(key) => write!(f, "Custom->{:?}", key),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Key {
    Usize(usize),
    String(String),
    Uuid(Uuid),
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Usize(value) => write!(f, "Usize({})", value),
            Key::String(value) => write!(f, "String({})", value),
            Key::Uuid(value) => write!(f, "Uuid({})", value),
        }
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Key::String(value)
    }
}

impl<'a> From<&'a String> for Key {
    fn from(value: &'a String) -> Self {
        Key::String(value.clone())
    }
}

impl<'a> From<&'a str> for Key {
    fn from(value: &'a str) -> Self {
        Key::String(value.to_string())
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        Key::Usize(value)
    }
}

impl From<Uuid> for Key {
    fn from(value: Uuid) -> Self {
        Key::Uuid(value)
    }
}

impl From<&Uuid> for Key {
    fn from(value: &Uuid) -> Self {
        Key::Uuid(*value)
    }
}

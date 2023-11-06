use namui_type::Uuid;
use std::fmt::Debug;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyVec {
    chunks: Vec<KeyChunk>,
}
impl KeyVec {
    pub(crate) fn new_child(key: impl Into<Key>) -> Self {
        KeyVec {
            chunks: vec![KeyChunk::Child(key.into())],
        }
    }
    pub(crate) fn child(&self, key: impl Into<Key>) -> KeyVec {
        let mut key_vec = self.clone();
        key_vec.chunks.push(KeyChunk::Child(key.into()));
        key_vec
    }
    pub(crate) fn custom_key(&self, key: impl Into<Key>) -> KeyVec {
        let mut key_vec = self.clone();
        key_vec.chunks.push(KeyChunk::Custom(key.into()));
        key_vec
    }
}

impl Debug for KeyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.chunks.iter()).finish()
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

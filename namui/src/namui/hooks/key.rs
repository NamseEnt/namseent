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
    pub(crate) fn group(&self, key: impl Into<Key>) -> KeyVec {
        let mut key_vec = self.clone();
        key_vec.chunks.push(KeyChunk::Group(key.into()));
        key_vec
    }
    pub(crate) fn custom_key(&self, key: impl Into<Key>) -> KeyVec {
        let mut key_vec = self.clone();
        key_vec.chunks.push(KeyChunk::Custom(key.into()));
        key_vec
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum KeyChunk {
    Child(Key),
    Group(Key),
    Custom(Key),
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Key {
    Usize(usize),
    String(String),
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Key::String(value)
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

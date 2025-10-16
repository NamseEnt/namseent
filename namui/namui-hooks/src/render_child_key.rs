use crate::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, OurSerde)]
pub(crate) enum ChildKey {
    String(String),
    U128(u128),
    IncrementalComponent { index: usize, type_name: String },
    IncrementalCompose { index: usize },
}

impl From<String> for ChildKey {
    fn from(value: String) -> Self {
        ChildKey::String(value)
    }
}

impl<'a> From<&'a String> for ChildKey {
    fn from(value: &'a String) -> Self {
        ChildKey::String(value.clone())
    }
}

impl<'a> From<&'a str> for ChildKey {
    fn from(value: &'a str) -> Self {
        ChildKey::String(value.to_string())
    }
}

impl From<usize> for ChildKey {
    fn from(value: usize) -> Self {
        ChildKey::U128(value as u128)
    }
}

impl From<u128> for ChildKey {
    fn from(value: u128) -> Self {
        ChildKey::U128(value)
    }
}

impl From<&u128> for ChildKey {
    fn from(value: &u128) -> Self {
        ChildKey::U128(*value)
    }
}

pub enum AddKey {
    String(String),
    U128(u128),
    Incremental,
}

impl From<Option<AddKey>> for AddKey {
    fn from(key: Option<AddKey>) -> Self {
        key.unwrap_or(AddKey::Incremental)
    }
}

impl From<String> for AddKey {
    fn from(key: String) -> Self {
        AddKey::String(key)
    }
}

impl From<&str> for AddKey {
    fn from(key: &str) -> Self {
        AddKey::String(key.to_string())
    }
}

impl From<usize> for AddKey {
    fn from(key: usize) -> Self {
        AddKey::U128(key as u128)
    }
}

impl From<u128> for AddKey {
    fn from(key: u128) -> Self {
        AddKey::U128(key)
    }
}

#[derive(Clone, OurSerde, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ChildKeyChain {
    keys: Vec<ChildKey>,
}

impl ChildKeyChain {
    pub const ROOT: Self = Self { keys: Vec::new() };

    pub fn append(&self, key: ChildKey) -> Self {
        let mut keys = self.keys.clone();
        keys.push(key);
        Self { keys }
    }
}

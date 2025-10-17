use crate::*;
use crc32fast::Hasher;
use std::hash::Hash;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, OurSerde)]
pub(crate) struct ChildKey {
    value: u32,
}

impl ChildKey {
    fn hash(&self) -> u32 {
        self.value
    }

    pub(crate) fn string(key: String) -> ChildKey {
        let mut hasher = Hasher::new();
        hasher.update(key.as_bytes());
        ChildKey {
            value: hasher.finalize(),
        }
    }

    pub(crate) fn u128(uuid: u128) -> ChildKey {
        let mut hasher = Hasher::new();
        hasher.update(&uuid.to_le_bytes());
        ChildKey {
            value: hasher.finalize(),
        }
    }

    pub(crate) fn incremental_compose(index: usize) -> ChildKey {
        let mut hasher = Hasher::new();
        hasher.update(&index.to_le_bytes());
        ChildKey {
            value: hasher.finalize(),
        }
    }

    pub(crate) fn incremental_component(index: usize, type_name: &str) -> ChildKey {
        let mut hasher = Hasher::new();
        hasher.update(&index.to_le_bytes());
        hasher.update(type_name.as_bytes());
        ChildKey {
            value: hasher.finalize(),
        }
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
    hashed: u32,
}

impl ChildKeyChain {
    pub const ROOT: Self = Self { hashed: 0 };

    pub fn append(&self, key: ChildKey) -> Self {
        let hashed = self.hashed ^ key.hash();
        Self { hashed }
    }
}

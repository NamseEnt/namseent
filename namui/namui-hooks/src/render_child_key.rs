use crate::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, OurSerde)]
pub(crate) struct ChildKey {
    value: u32,
}

impl ChildKey {
    fn hash(&self) -> u32 {
        self.value
    }

    pub(crate) fn string(key: String) -> ChildKey {
        ChildKey {
            value: fxhash::hash32(&key),
        }
    }

    pub(crate) fn u128(uuid: u128) -> ChildKey {
        ChildKey {
            value: fxhash::hash32(&uuid),
        }
    }

    pub(crate) fn incremental_compose(index: usize) -> ChildKey {
        ChildKey {
            value: fxhash::hash32(&index),
        }
    }

    pub(crate) fn incremental_component(index: usize, type_name: &str) -> ChildKey {
        let mut hasher = fxhash::FxHasher32::default();
        hasher.write_usize(index);
        hasher.write(type_name.as_bytes());
        ChildKey {
            value: hasher.finish() as u32,
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

#[derive(Debug, Clone, OurSerde, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ChildKeyChain {
    hashed: u32,
}

impl ChildKeyChain {
    pub const ROOT: Self = Self { hashed: 0 };

    pub fn append(&self, key: ChildKey) -> Self {
        let mut hasher = fxhash::FxHasher32::default();
        hasher.write_u32(self.hashed);
        hasher.write_u32(key.hash());
        let hashed = hasher.finish() as u32;
        Self { hashed }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_commutative_property() {
        let key_a = ChildKey::string("first".to_string());
        let key_b = ChildKey::string("second".to_string());

        let chain_ab = ChildKeyChain::ROOT
            .append(key_a.clone())
            .append(key_b.clone());
        let chain_ba = ChildKeyChain::ROOT
            .append(key_b.clone())
            .append(key_a.clone());

        assert_ne!(chain_ab, chain_ba,);
    }

    #[test]
    fn test_collision_self_inverse() {
        let key = ChildKey::string("test".to_string());

        let chain = ChildKeyChain::ROOT.append(key.clone()).append(key.clone());

        assert_ne!(chain, ChildKeyChain::ROOT);
    }

    #[test]
    fn test_collision_complex_cancellation() {
        let key_a = ChildKey::string("alpha".to_string());
        let key_b = ChildKey::string("beta".to_string());

        let chain_aba = ChildKeyChain::ROOT
            .append(key_a.clone())
            .append(key_b.clone())
            .append(key_a.clone());

        let chain_b = ChildKeyChain::ROOT.append(key_b.clone());

        assert_ne!(chain_aba, chain_b);
    }

    #[test]
    fn test_collision_multiple_paths_to_same_hash() {
        let key_a = ChildKey::string("x".to_string());
        let key_b = ChildKey::string("y".to_string());
        let key_c = ChildKey::string("z".to_string());

        let chain_abc = ChildKeyChain::ROOT
            .append(key_a.clone())
            .append(key_b.clone())
            .append(key_c.clone());

        let chain_cba = ChildKeyChain::ROOT
            .append(key_c.clone())
            .append(key_b.clone())
            .append(key_a.clone());

        assert_ne!(chain_abc, chain_cba);
    }

    #[test]
    fn test_collision_with_component_keys() {
        let key_1 = ChildKey::incremental_component(0, "ComponentA");
        let key_2 = ChildKey::incremental_component(1, "ComponentB");

        let chain_12 = ChildKeyChain::ROOT
            .append(key_1.clone())
            .append(key_2.clone());
        let chain_21 = ChildKeyChain::ROOT
            .append(key_2.clone())
            .append(key_1.clone());

        assert_ne!(chain_12, chain_21);
    }

    #[test]
    fn test_collision_empty_and_paired_keys() {
        let key_1 = ChildKey::u128(12345);
        let key_2 = ChildKey::u128(67890);

        let chain = ChildKeyChain::ROOT
            .append(key_1.clone())
            .append(key_2.clone())
            .append(key_2.clone())
            .append(key_1.clone());

        assert_ne!(chain, ChildKeyChain::ROOT,);
    }

    #[test]
    fn test_collision_associative_property() {
        let key_a = ChildKey::string("aaa".to_string());
        let key_b = ChildKey::string("bbb".to_string());
        let key_c = ChildKey::string("ccc".to_string());

        let chain_1 = ChildKeyChain::ROOT
            .append(key_a.clone())
            .append(key_b.clone())
            .append(key_c.clone());

        let middle_chain = ChildKeyChain::ROOT
            .append(key_b.clone())
            .append(key_c.clone());
        let chain_2 = ChildKeyChain::ROOT.append(key_a.clone()).append(ChildKey {
            value: middle_chain.hashed,
        });

        assert_ne!(chain_1, chain_2);
    }

    #[test]
    fn test_no_collision_different_strings() {
        let strings = vec!["apple", "banana", "cherry", "date", "elderberry"];
        let mut chains = Vec::new();

        for s in &strings {
            let key = ChildKey::string(s.to_string());
            let chain = ChildKeyChain::ROOT.append(key);
            chains.push(chain);
        }

        for i in 0..chains.len() {
            for j in i + 1..chains.len() {
                assert_ne!(chains[i], chains[j]);
            }
        }
    }
}

use namui::*;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, State)]
pub struct EntityId(u64);

impl EntityId {
    pub(crate) const fn from_raw(raw: u64) -> Self {
        assert!(raw != 0, "entity ID zero is reserved");
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

macro_rules! define_entity_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, State)]
        pub struct $name(EntityId);

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn from_raw(raw: u64) -> Self {
                Self(EntityId::from_raw(raw))
            }

            pub const fn raw(self) -> u64 {
                self.0.raw()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.raw().fmt(formatter)
            }
        }
    };
}

define_entity_id!(MonsterId);
define_entity_id!(TowerId);
define_entity_id!(AttackId);

#[cfg(test)]
impl TowerId {
    pub(crate) const fn from_entity_id(id: EntityId) -> Self {
        Self(id)
    }
}

#[cfg(test)]
impl AttackId {
    pub(crate) const fn from_entity_id(id: EntityId) -> Self {
        Self(id)
    }
}

pub(crate) use td_core::EntityIdAllocator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_issues_one_shared_monotonic_sequence() {
        let mut allocator = EntityIdAllocator::default();

        assert_eq!(MonsterId::from_raw(allocator.allocate_raw()).raw(), 1);
        assert_eq!(TowerId::from_raw(allocator.allocate_raw()).raw(), 2);
        assert_eq!(AttackId::from_raw(allocator.allocate_raw()).raw(), 3);
        assert_eq!(AttackId::from_raw(9).raw(), 9);
    }

    #[test]
    #[should_panic(expected = "entity ID space exhausted")]
    fn allocator_does_not_wrap_at_the_maximum() {
        let mut allocator = EntityIdAllocator::from_next_id(u64::MAX);
        assert_eq!(allocator.allocate_raw(), u64::MAX);
        allocator.allocate_raw();
    }
}

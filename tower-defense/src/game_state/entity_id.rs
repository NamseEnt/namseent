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

            pub(crate) const fn from_entity_id(id: EntityId) -> Self {
                Self(id)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
pub(crate) struct EntityIdAllocator {
    next: u64,
}

impl Default for EntityIdAllocator {
    fn default() -> Self {
        Self { next: 1 }
    }
}

impl EntityIdAllocator {
    pub(crate) fn next_id(&self) -> u64 {
        self.next
    }

    pub(crate) fn allocate(&mut self) -> EntityId {
        let id = EntityId::from_raw(self.next);
        self.next = self.next.checked_add(1).expect("entity ID space exhausted");
        id
    }

    pub(crate) fn allocate_monster_id(&mut self) -> MonsterId {
        MonsterId::from_entity_id(self.allocate())
    }

    pub(crate) fn allocate_tower_id(&mut self) -> TowerId {
        TowerId::from_entity_id(self.allocate())
    }

    pub(crate) fn allocate_attack_id(&mut self) -> AttackId {
        AttackId::from_entity_id(self.allocate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_issues_one_shared_monotonic_sequence() {
        let mut allocator = EntityIdAllocator::default();

        assert_eq!(allocator.allocate_monster_id().raw(), 1);
        assert_eq!(allocator.allocate_tower_id().raw(), 2);
        assert_eq!(allocator.allocate_attack_id().raw(), 3);
        assert_eq!(AttackId::from_raw(9).raw(), 9);
    }

    #[test]
    #[should_panic(expected = "entity ID space exhausted")]
    fn allocator_does_not_wrap_at_the_maximum() {
        let mut allocator = EntityIdAllocator { next: u64::MAX };
        assert_eq!(allocator.allocate().raw(), u64::MAX);
        allocator.allocate();
    }
}

use crate::{CardState, TowerTemplateState};
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HandItemState {
    Card(CardState),
    Tower(TowerTemplateState),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HandSlotState {
    pub id: usize,
    pub item: HandItemState,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HandState {
    pub slots: Vec<HandSlotState>,
    /// Monotonic allocator for presentation-stable hand slot identities.
    ///
    /// This is deliberately separate from the slot order. Older snapshots do
    /// not contain the field and deserialize it as zero; `migrate_slot_ids`
    /// repairs that value before a snapshot is accepted.
    #[serde(default)]
    pub next_hand_slot_id: usize,
}

impl HandState {
    pub fn migrate_slot_ids(&mut self) {
        let mut used = HashSet::with_capacity(self.slots.len());
        let mut next = self
            .slots
            .iter()
            .map(|slot| slot.id)
            .filter(|id| *id != 0)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
            .max(1);

        for slot in &mut self.slots {
            if slot.id == 0 || !used.insert(slot.id) {
                while used.contains(&next) || next == 0 {
                    next = next.saturating_add(1).max(1);
                }
                slot.id = next;
                used.insert(next);
                next = next.saturating_add(1).max(1);
            }
        }

        let maximum = used.iter().copied().max().unwrap_or(0);
        self.next_hand_slot_id = self
            .next_hand_slot_id
            .max(maximum.saturating_add(1).max(1))
            .max(1);
    }

    pub fn allocate_slot_id(&mut self) -> usize {
        self.migrate_slot_ids();
        let id = self.next_hand_slot_id.max(1);
        self.next_hand_slot_id = id.saturating_add(1).max(1);
        id
    }

    pub(crate) fn sort_slots(&mut self) {
        self.slots
            .sort_by(|left, right| compare_items(&left.item, &right.item).reverse());
    }
}

fn compare_items(left: &HandItemState, right: &HandItemState) -> Ordering {
    match (left, right) {
        (HandItemState::Card(left), HandItemState::Card(right)) => left
            .rank
            .cmp(&right.rank)
            .then_with(|| left.suit.cmp(&right.suit))
            .then_with(|| left.id.cmp(&right.id)),
        (HandItemState::Tower(left), HandItemState::Tower(right)) => left
            .kind
            .cmp(&right.kind)
            .then_with(|| left.suit.cmp(&right.suit))
            .then_with(|| left.rank.cmp(&right.rank)),
        (HandItemState::Card(_), HandItemState::Tower(_)) => Ordering::Less,
        (HandItemState::Tower(_), HandItemState::Card(_)) => Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(id: usize, rank: u8) -> HandSlotState {
        HandSlotState {
            id,
            item: HandItemState::Card(CardState {
                id,
                suit: 0,
                rank,
                polish_pct_raw: 0,
                engraving: None,
            }),
            selected: false,
        }
    }

    #[test]
    fn sorting_preserves_slot_identity_and_allocator() {
        let mut hand = HandState {
            slots: vec![card(9, 1), card(4, 12)],
            next_hand_slot_id: 10,
        };

        hand.sort_slots();

        assert_eq!(
            hand.slots.iter().map(|slot| slot.id).collect::<Vec<_>>(),
            vec![4, 9]
        );
        assert_eq!(hand.allocate_slot_id(), 10);
        assert_eq!(hand.next_hand_slot_id, 11);
    }

    #[test]
    fn migration_repairs_missing_and_duplicate_ids_without_reusing_valid_ids() {
        let mut hand = HandState {
            slots: vec![card(3, 1), card(3, 2), card(0, 3)],
            next_hand_slot_id: 0,
        };

        hand.migrate_slot_ids();

        assert_eq!(
            hand.slots.iter().map(|slot| slot.id).collect::<Vec<_>>(),
            vec![3, 4, 5]
        );
        assert_eq!(hand.next_hand_slot_id, 6);
    }
}

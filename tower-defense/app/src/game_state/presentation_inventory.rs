use super::{
    item::ItemWithId,
    presentation_transition::{PresentationTransition, diff_projection},
};
use crate::PresentationInstant;
use namui::*;

#[derive(Clone, Debug, State)]
pub(crate) struct PresentationInventoryEntry {
    pub item: ItemWithId,
    pub order: usize,
    pub exit_started_at: Option<PresentationInstant>,
}

impl PresentationInventoryEntry {
    pub fn is_exiting(&self) -> bool {
        self.exit_started_at.is_some()
    }
}

#[derive(Clone, Debug, Default, State)]
pub(crate) struct PresentationInventory {
    pub entries: Vec<PresentationInventoryEntry>,
}

impl PresentationInventory {
    pub fn sync(
        &mut self,
        items: &[ItemWithId],
        presentation_instant: PresentationInstant,
        restore: bool,
    ) -> PresentationTransition<super::item::ItemId> {
        let before = self
            .entries
            .iter()
            .filter(|entry| !entry.is_exiting())
            .map(|entry| (entry.item.id, entry.item.clone()))
            .collect::<Vec<_>>();
        let after = items
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>();
        let transition = diff_projection(&before, &after);

        if restore {
            self.entries = items
                .iter()
                .enumerate()
                .map(|(order, item)| PresentationInventoryEntry {
                    item: item.clone(),
                    order,
                    exit_started_at: None,
                })
                .collect();
            return transition;
        }

        for entry in &mut self.entries {
            if let Some((order, item)) = items
                .iter()
                .enumerate()
                .find(|(_, item)| item.id == entry.item.id)
            {
                entry.item = item.clone();
                entry.order = order;
                entry.exit_started_at = None;
            } else if entry.exit_started_at.is_none() {
                entry.exit_started_at = Some(presentation_instant);
            }
        }
        for (order, item) in items.iter().enumerate() {
            if !self.entries.iter().any(|entry| entry.item.id == item.id) {
                self.entries.push(PresentationInventoryEntry {
                    item: item.clone(),
                    order,
                    exit_started_at: None,
                });
            }
        }
        self.entries
            .sort_by_key(|entry| (entry.is_exiting(), entry.order));
        transition
    }

    pub fn update(&mut self, presentation_instant: PresentationInstant) {
        self.entries.retain(|entry| {
            entry
                .exit_started_at
                .is_none_or(|started| (presentation_instant - started).as_secs_f32() < 0.5)
        });
    }

    pub fn active_item_index(&self, id: super::item::ItemId) -> Option<usize> {
        self.entries
            .iter()
            .filter(|entry| !entry.is_exiting())
            .position(|entry| entry.item.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::item::Item;

    #[test]
    fn removed_items_exit_but_are_not_interactable() {
        let mut inventory = PresentationInventory::default();
        let first = ItemWithId::new(Item::Bread(crate::game_state::item::BreadItem::standard()));
        let second = ItemWithId::new(Item::Bread(crate::game_state::item::BreadItem::standard()));
        let instant = PresentationInstant::zero();
        inventory.sync(&[first.clone(), second.clone()], instant, true);
        inventory.sync(std::slice::from_ref(&second), instant, false);

        assert_eq!(inventory.active_item_index(second.id), Some(0));
        assert!(
            inventory.entries.iter().any(|entry| {
                entry.item.id == first.id && entry.exit_started_at == Some(instant)
            })
        );
        assert_eq!(inventory.active_item_index(first.id), None);
    }
}

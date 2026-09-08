use crate::PresentationInstant;
mod shop_slot;

pub use shop_slot::*;

use crate::game_state::card_service::CardServiceDiscriminants;
use crate::*;

#[derive(Clone, Debug, State)]
pub struct Shop {
    pub slots: Vec<ShopSlotData>,
}

impl Shop {
    pub(crate) fn to_core_state(&self) -> td_core::ShopState {
        td_core::ShopState {
            slots: self
                .slots
                .iter()
                .filter(|slot| slot.exit_animation.is_none())
                .map(ShopSlotData::to_core_state)
                .collect(),
        }
    }

    pub(crate) fn from_core_state(
        state: td_core::ShopState,
        presentation_source: Option<&Self>,
    ) -> Option<Self> {
        let mut ids = Vec::with_capacity(state.slots.len());
        let slots = state
            .slots
            .into_iter()
            .filter(|slot| {
                !slot.purchased
                    || presentation_source.is_some_and(|source| {
                        source
                            .slots
                            .iter()
                            .any(|candidate| candidate.id.raw() == slot.id)
                    })
            })
            .map(|slot| {
                if ids.contains(&slot.id) {
                    return None;
                }
                ids.push(slot.id);
                let presentation = presentation_source.and_then(|source| {
                    source
                        .slots
                        .iter()
                        .find(|candidate| candidate.id.raw() == slot.id)
                });
                Some(ShopSlotData {
                    id: ShopSlotId::from_raw(slot.id),
                    slot: ShopSlot::from_core_state(slot.slot)?,
                    purchased: slot.purchased,
                    exit_animation: presentation.and_then(|slot| slot.exit_animation),
                })
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self { slots })
    }

    pub fn get_slot_by_id(&self, id: ShopSlotId) -> Option<&ShopSlotData> {
        self.slots.iter().find(|slot| slot.id == id)
    }

    pub fn get_slot_by_id_mut(&mut self, id: ShopSlotId) -> Option<&mut ShopSlotData> {
        self.slots.iter_mut().find(|slot| slot.id == id)
    }

    pub fn push(&mut self, slot: ShopSlot) {
        self.slots.push(ShopSlotData::new(slot));
    }

    pub fn remove_completed_exit_animations(&mut self, presentation_instant: PresentationInstant) {
        self.slots
            .retain(|slot| !slot.is_exit_animation_complete(presentation_instant));
    }

    pub fn update(&mut self, presentation_instant: PresentationInstant) {
        self.remove_completed_exit_animations(presentation_instant);
    }
}

impl ShopSlotData {
    pub(crate) fn to_core_state(&self) -> td_core::ShopSlotDataState {
        td_core::ShopSlotDataState {
            id: self.id.raw(),
            slot: self.slot.to_core_state(),
            purchased: self.purchased,
        }
    }
}

impl ShopSlot {
    pub(crate) fn to_core_state(&self) -> td_core::ShopSlotState {
        match self {
            Self::Item { item, cost } => td_core::ShopSlotState::Item {
                item: crate::game_state::item::ItemWithId {
                    id: crate::game_state::item::ItemId(0),
                    item: item.clone(),
                }
                .to_core_state(),
                cost: *cost,
            },
            Self::Upgrade { upgrade, cost } => td_core::ShopSlotState::Upgrade {
                upgrade: crate::game_state::upgrade::UpgradeWithId {
                    id: crate::game_state::upgrade::UpgradeId(0),
                    upgrade: *upgrade,
                }
                .to_core_state(),
                cost: *cost,
            },
            Self::CardService { card_service, cost } => td_core::ShopSlotState::CardService {
                kind: CardServiceDiscriminants::from(card_service)
                    .to_core_kind()
                    .raw(),
                cost: *cost,
            },
        }
    }

    pub(crate) fn from_core_state(state: td_core::ShopSlotState) -> Option<Self> {
        Some(match state {
            td_core::ShopSlotState::Item { item, cost } => Self::Item {
                item: crate::game_state::item::ItemWithId::from_core_state(item)?.item,
                cost,
            },
            td_core::ShopSlotState::Upgrade { upgrade, cost } => Self::Upgrade {
                upgrade: crate::game_state::upgrade::UpgradeWithId::from_core_state(upgrade)?
                    .upgrade,
                cost,
            },
            td_core::ShopSlotState::CardService { kind, cost } => Self::CardService {
                card_service: CardServiceDiscriminants::from_core_kind(
                    td_core::CardServiceKind::from_raw(kind)?,
                )
                .generate(),
                cost,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn purchased_slots_without_an_active_presentation_entry_are_pruned() {
        let item = td_core::generated_item(td_core::ItemKind::Candy).expect("candy item");
        let purchased_item = td_core::ShopSlotDataState {
            id: 7,
            slot: td_core::ShopSlotState::Item { item, cost: 20 },
            purchased: true,
        };
        let purchased_card_service = td_core::ShopSlotDataState {
            id: 8,
            slot: td_core::ShopSlotState::CardService {
                kind: td_core::CardServiceKind::Eraser.raw(),
                cost: 45,
            },
            purchased: true,
        };

        let shop = Shop::from_core_state(
            td_core::ShopState {
                slots: vec![purchased_item, purchased_card_service],
            },
            None,
        )
        .expect("valid purchased shop state");

        assert!(shop.slots.is_empty());
    }
}

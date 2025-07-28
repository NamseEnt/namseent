mod hand_slot;
mod render_card;
mod xy_with_spring;

use crate::{
    card::Card,
    game_state::hand::hand_slot::{HandSlot, HandSlotKind},
};
pub use hand_slot::HandSlotId;
use namui::*;

pub const HAND_SLOT_WH: Wh<Px> = Wh::new(px(112.), px(152.));
pub const HAND_WH: Wh<Px> = Wh::new(px(600.), px(160.));

#[derive(Default)]
pub struct Hand {
    slots: Vec<HandSlot>,
}
impl Hand {
    pub fn clear(&mut self) {
        self.slots.clear();
    }

    pub fn add_random_cards(&mut self, amount: usize) {
        let slots = (0..amount).map(|_| HandSlot::from_card(Card::new_random()));
        self.slots.extend(slots);
        self.calculate_slot_xy();
    }

    pub fn delete_slots(&mut self, ids: &[HandSlotId]) {
        self.slots.retain(|slot| !ids.contains(&slot.id));
        self.calculate_slot_xy();
    }

    pub fn selected_slot_ids(&self) -> Vec<HandSlotId> {
        self.slots
            .iter()
            .filter_map(|slot| match slot.selected {
                true => Some(slot.id),
                false => None,
            })
            .collect()
    }

    pub fn select_slot(&mut self, id: HandSlotId) {
        if let Some(slot) = self.slots.iter_mut().find(|s| s.id == id) {
            slot.selected = true;
        }
    }

    pub fn deselect_slot(&mut self, id: HandSlotId) {
        if let Some(slot) = self.slots.iter_mut().find(|s| s.id == id) {
            slot.selected = false;
        }
    }

    pub fn selected_cards(&self) -> Vec<Card> {
        self.slots
            .iter()
            .filter_map(|slot| match slot.slot_kind {
                HandSlotKind::Card { card } if slot.selected => Some(card),
                _ => None,
            })
            .collect()
    }

    pub fn all_cards(&self) -> Vec<Card> {
        self.slots
            .iter()
            .filter_map(|slot| match slot.slot_kind {
                HandSlotKind::Card { card } => Some(card),
                _ => None,
            })
            .collect()
    }

    fn calculate_slot_xy(&mut self) {
        let slot_count = self.slots.len();
        if slot_count == 0 {
            return;
        }
        let slot_count = slot_count as f32;

        let default_gap = px(8.0);
        let slot_width = HAND_SLOT_WH.width;
        let hand_width = HAND_WH.width;

        let total_width_with_default_gap =
            slot_width * slot_count + default_gap * (slot_count - 1.0);

        // 갭 계산: hand 너비를 넘으면 음수 갭 적용
        let gap = if total_width_with_default_gap > hand_width {
            (hand_width - slot_width * slot_count) / (slot_count - 1.0)
        } else {
            default_gap
        };

        let total_width = slot_width * slot_count + gap * (slot_count - 1.0);
        let start_x = (hand_width - total_width) / 2.0;

        // 각 슬롯의 xy 위치 계산 및 업데이트
        for (index, slot) in self.slots.iter_mut().enumerate() {
            let x = start_x + (slot_width + gap) * index as f32;
            let y = (HAND_WH.height - HAND_SLOT_WH.height) / 2.0;
            slot.set_xy(Xy { x, y });
        }
    }
}

pub struct HandComponent<'a> {
    pub hand: &'a Hand,
    pub on_click: &'a dyn Fn(HandSlotId),
}
impl Component for HandComponent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let HandComponent { hand, on_click } = self;
        for slot in &hand.slots {
            ctx.add_with_key(slot.id, slot).attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                event.stop_propagation();
                (on_click)(slot.id);
            });
        }
    }
}

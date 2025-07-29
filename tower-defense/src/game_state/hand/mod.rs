mod hand_slot;
mod render_card;
mod render_tower;
mod shared;
mod xy_with_spring;

use crate::{
    card::{Card, Rank, Suit},
    game_state::{
        hand::hand_slot::{HandSlot, HandSlotKind},
        tower::{TowerKind, TowerTemplate},
    },
};
pub use hand_slot::HandSlotId;
use namui::*;

pub const HAND_SLOT_WH: Wh<Px> = Wh::new(px(112.), px(152.));
pub const HAND_WH: Wh<Px> = Wh::new(px(600.), px(160.));

#[derive(Default, Clone)]
pub struct Hand {
    slots: Vec<HandSlot>,
}
impl Hand {
    pub fn clear(&mut self) {
        // exit 애니메이션 중이지 않은 모든 슬롯들의 ID 수집
        let slot_ids_to_delete: Vec<HandSlotId> = self
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .map(|slot| slot.id)
            .collect();

        // 수집된 슬롯들에 대해 delete_slots 호출
        if !slot_ids_to_delete.is_empty() {
            self.delete_slots(&slot_ids_to_delete);
        }
    }

    pub fn add_random_cards(&mut self, amount: usize) {
        let slots = (0..amount).map(|_| HandSlot::from_card(Card::new_random()));
        self.slots.extend(slots);
        self.calculate_slot_xy();
    }

    pub fn add_tower_template_with_barricades(
        &mut self,
        tower_template: TowerTemplate,
        barricade_amount: usize,
    ) {
        let barricade_template = TowerTemplate::new(TowerKind::Barricade, Suit::Spades, Rank::Ace);
        let mut tower_templates = vec![tower_template];

        // barricade_amount만큼 바리케이드 추가
        for _ in 0..barricade_amount {
            tower_templates.push(barricade_template.clone());
        }

        let slots = tower_templates
            .into_iter()
            .map(HandSlot::from_tower_template);

        self.slots.extend(slots);
        self.calculate_slot_xy();
    }

    pub fn delete_slots(&mut self, ids: &[HandSlotId]) {
        let now = Instant::now();
        // 삭제할 슬롯들에 exit 애니메이션 시작
        for slot in self.slots.iter_mut() {
            if ids.contains(&slot.id) {
                slot.start_exit_animation(now);
                slot.selected = false; // 선택 해제
            }
        }
        self.calculate_slot_xy();
    }

    pub fn remove_completed_exit_animations(&mut self) {
        let now = Instant::now();
        // 완료된 exit 애니메이션이 있는지 먼저 확인
        let has_completed_animations = self
            .slots
            .iter()
            .any(|slot| slot.is_exit_animation_complete(now));

        // 완료된 애니메이션이 있을 때만 retain 실행
        if has_completed_animations {
            self.slots
                .retain(|slot| !slot.is_exit_animation_complete(now));
        }
    }

    pub fn update(&mut self) {
        self.remove_completed_exit_animations();
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
            if slot.exit_animation.is_some() {
                return; // exit 애니메이션 중인 슬롯은 선택 불가
            }
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
                HandSlotKind::Card { .. } => None,
                HandSlotKind::Tower { .. } => None,
            })
            .collect()
    }

    pub fn all_cards(&self) -> Vec<Card> {
        self.slots
            .iter()
            .filter_map(|slot| match slot.slot_kind {
                HandSlotKind::Card { card } => Some(card),
                HandSlotKind::Tower { .. } => None,
            })
            .collect()
    }

    pub fn width(&self) -> Px {
        HAND_WH.width
    }

    pub fn get_slot_id_by_index(&self, index: usize) -> Option<HandSlotId> {
        self.slots.get(index).map(|slot| slot.id)
    }

    pub fn get_tower_template_by_id(&self, id: HandSlotId) -> Option<&TowerTemplate> {
        self.slots
            .iter()
            .find(|slot| slot.id == id)
            .and_then(|slot| slot.get_tower_template())
    }

    pub fn has_tower_slots(&self) -> bool {
        self.slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none()) // exit 애니메이션 중이지 않은 슬롯만
            .any(|slot| slot.get_tower_template().is_some())
    }

    fn calculate_slot_xy(&mut self) {
        // exit 애니메이션이 진행 중이지 않은 슬롯들만 필터링
        let active_slots: Vec<(usize, &mut HandSlot)> = self
            .slots
            .iter_mut()
            .enumerate()
            .filter(|(_, slot)| slot.exit_animation.is_none())
            .collect();

        let slot_count = active_slots.len();
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

        // 각 활성 슬롯의 xy 위치 계산 및 업데이트
        for (active_index, (_, slot)) in active_slots.into_iter().enumerate() {
            let x = start_x + (slot_width + gap) * active_index as f32;
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

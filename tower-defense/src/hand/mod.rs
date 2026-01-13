mod hand_slot;
mod render_card;
mod render_tower;
pub mod shared;

use hand_slot::HandSlot;
pub use hand_slot::HandSlotId;
use namui::*;
use render_card::RenderCard;
use render_tower::RenderTower;
use shared::*;
use std::{any::Any, cmp::Ordering, fmt::Debug};
pub use crate::animation::xy_with_spring;

pub const HAND_SLOT_WH: Wh<Px> = Wh::new(px(112.), px(152.));
pub const HAND_WH: Wh<Px> = Wh::new(px(600.), px(160.));

// 레이아웃 관련 상수들
const DEFAULT_SLOT_GAP: Px = px(8.0);

#[derive(Default, Clone, Debug, State)]
pub struct Hand<Item: State + Debug> {
    slots: Vec<HandSlot<Item>>,
}
impl<Item: State + PartialOrd + Debug> Hand<Item> {
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        let slots = items.into_iter().map(|item| HandSlot::new(item)).collect();
        let mut hand = Self { slots };
        hand.calculate_slot_xy();
        hand
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

    pub fn active_slot_ids(&self) -> Vec<HandSlotId> {
        self.active_slots().map(|slot| slot.id).collect()
    }
    pub fn selected_slot_ids(&self) -> Vec<HandSlotId> {
        self.active_slots()
            .filter_map(|slot| match slot.selected {
                true => Some(slot.id),
                false => None,
            })
            .collect()
    }

    pub fn select_slot(&mut self, id: HandSlotId) {
        if let Some(slot) = self.find_slot_by_id_mut(id) {
            if slot.exit_animation.is_some() {
                return; // exit 애니메이션 중인 슬롯은 선택 불가
            }
            slot.selected = true;
        }
    }

    pub fn deselect_slot(&mut self, id: HandSlotId) {
        if let Some(slot) = self.find_slot_by_id_mut(id) {
            slot.selected = false;
        }
    }

    pub fn get_slot_id_by_index(&self, index: usize) -> Option<HandSlotId> {
        self.slots.get(index).map(|slot| slot.id)
    }

    pub fn is_empty(&self) -> bool {
        self.active_slots().next().is_none()
    }

    pub fn get_items(&self, slot_ids: &[HandSlotId]) -> impl Iterator<Item = &Item> {
        slot_ids
            .iter()
            .map(|id| &self.slots.iter().find(|slot| slot.id == *id).unwrap().item)
    }

    pub fn push(&mut self, item: Item) {
        self.slots.push(HandSlot::new(item));
        self.calculate_slot_xy();
    }

    fn sort_slots(&mut self) {
        self.slots.sort_by(|a, b| {
            // exit 애니메이션 중인 슬롯은 뒤로 정렬
            match (a.exit_animation.is_some(), b.exit_animation.is_some()) {
                (true, false) => return Ordering::Greater,
                (false, true) => return Ordering::Less,
                _ => {}
            }

            a.item.partial_cmp(&b.item).unwrap().reverse()
        });
    }

    fn calculate_layout(slot_count: f32) -> (Px, Px) {
        let slot_width = HAND_SLOT_WH.width;
        let hand_width = HAND_WH.width;

        let total_width_with_default_gap =
            slot_width * slot_count + DEFAULT_SLOT_GAP * (slot_count - 1.0);

        // 갭 계산: hand 너비를 넘으면 음수 갭 적용
        let gap = if total_width_with_default_gap > hand_width {
            (hand_width - slot_width * slot_count) / (slot_count - 1.0)
        } else {
            DEFAULT_SLOT_GAP
        };

        let total_width = slot_width * slot_count + gap * (slot_count - 1.0);
        let start_x = (hand_width - total_width) / 2.0;

        (start_x, gap)
    }

    fn calculate_slot_xy(&mut self) {
        // 먼저 슬롯들을 정렬
        self.sort_slots();

        // exit 애니메이션이 진행 중이지 않은 슬롯들만 필터링
        let active_slots: Vec<(usize, &mut HandSlot<Item>)> = self
            .slots
            .iter_mut()
            .enumerate()
            .filter(|(_, slot)| slot.exit_animation.is_none())
            .collect();

        let slot_count = active_slots.len();
        if slot_count == 0 {
            return;
        }

        let (start_x, gap) = Self::calculate_layout(slot_count as f32);
        let slot_width = HAND_SLOT_WH.width;

        // 각 활성 슬롯의 xy 위치 계산 및 업데이트
        for (active_index, (_, slot)) in active_slots.into_iter().enumerate() {
            let x = start_x + (slot_width + gap) * active_index as f32;
            let y = (HAND_WH.height - HAND_SLOT_WH.height) / 2.0;
            slot.set_xy(Xy { x, y });
        }
    }
    fn find_slot_by_id_mut(&mut self, id: HandSlotId) -> Option<&mut HandSlot<Item>> {
        self.slots.iter_mut().find(|slot| slot.id == id)
    }

    fn active_slots(&self) -> impl Iterator<Item = &HandSlot<Item>> {
        self.slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
    }

    pub fn get_item(&self, slot_id: HandSlotId) -> Option<&Item> {
        self.slots
            .iter()
            .find(|slot| slot.id == slot_id)
            .map(|slot| &slot.item)
    }
}

pub struct HandComponent<'a, Item: State + Debug> {
    pub hand: &'a Hand<Item>,
    pub on_click: &'a dyn Fn(HandSlotId),
}
impl<'a, Item> Component for HandComponent<'a, Item>
where
    Item: State + Debug + Any,
{
    fn render(self, ctx: &RenderCtx) {
        let HandComponent { hand, on_click } = self;
        for slot in &hand.slots {
            ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                .add_with_key(slot.id, slot)
                .attach_event(|event| {
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

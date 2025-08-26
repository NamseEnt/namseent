mod hand_slot;
mod render_card;
mod render_tower;
pub mod shared;
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

// 레이아웃 관련 상수들
const DEFAULT_SLOT_GAP: Px = px(8.0);

// 바리케이드 기본 템플릿
const DEFAULT_BARRICADE_TEMPLATE: fn() -> TowerTemplate =
    || TowerTemplate::new(TowerKind::Barricade, Suit::Spades, Rank::Ace);

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
        let barricade_template = Self::create_barricade_template();
        let mut tower_templates = vec![tower_template];

        // barricade_amount만큼 바리케이드 추가
        tower_templates.extend((0..barricade_amount).map(|_| barricade_template.clone()));

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

    pub fn selected_cards(&self) -> Vec<Card> {
        self.active_slots()
            .filter_map(|slot| match slot.slot_kind {
                HandSlotKind::Card { card } if slot.selected => Some(card),
                HandSlotKind::Card { .. } => None,
                HandSlotKind::Tower { .. } => None,
            })
            .collect()
    }

    pub fn all_cards(&self) -> Vec<Card> {
        self.active_slots()
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
        self.find_slot_by_id(id)
            .and_then(|slot| slot.get_tower_template())
    }

    pub fn has_tower_slots(&self) -> bool {
        self.active_slots()
            .any(|slot| slot.get_tower_template().is_some())
    }

    pub fn get_first_tower_slot_id(&self) -> Option<HandSlotId> {
        self.active_slots()
            .find(|slot| slot.get_tower_template().is_some())
            .map(|slot| slot.id)
    }

    fn sort_slots(&mut self) {
        self.slots.sort_by(Self::compare_slots);
    }

    fn compare_slots(a: &HandSlot, b: &HandSlot) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        // exit 애니메이션 중인 슬롯은 뒤로 정렬
        match (a.exit_animation.is_some(), b.exit_animation.is_some()) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }

        // 카드와 타워 타입별 정렬
        match (&a.slot_kind, &b.slot_kind) {
            (HandSlotKind::Card { .. }, HandSlotKind::Tower { .. }) => Ordering::Less,
            (HandSlotKind::Tower { .. }, HandSlotKind::Card { .. }) => Ordering::Greater,
            (HandSlotKind::Card { card: card_a }, HandSlotKind::Card { card: card_b }) => {
                // 카드끼리는 rank -> suit -> id 순으로 정렬
                card_a
                    .rank
                    .cmp(&card_b.rank)
                    .then_with(|| card_a.suit.cmp(&card_b.suit))
                    .then_with(|| a.id.cmp(&b.id))
            }
            (
                HandSlotKind::Tower {
                    tower_template: tower_a,
                },
                HandSlotKind::Tower {
                    tower_template: tower_b,
                },
            ) => {
                // 타워끼리는 kind(역순) -> suit -> rank -> id 순으로 정렬
                tower_b
                    .kind
                    .cmp(&tower_a.kind)
                    .then_with(|| tower_a.suit.cmp(&tower_b.suit))
                    .then_with(|| tower_a.rank.cmp(&tower_b.rank))
                    .then_with(|| a.id.cmp(&b.id))
            }
        }
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

        let (start_x, gap) = Self::calculate_layout(slot_count as f32);
        let slot_width = HAND_SLOT_WH.width;

        // 각 활성 슬롯의 xy 위치 계산 및 업데이트
        for (active_index, (_, slot)) in active_slots.into_iter().enumerate() {
            let x = start_x + (slot_width + gap) * active_index as f32;
            let y = (HAND_WH.height - HAND_SLOT_WH.height) / 2.0;
            slot.set_xy(Xy { x, y });
        }
    }
    fn find_slot_by_id_mut(&mut self, id: HandSlotId) -> Option<&mut HandSlot> {
        self.slots.iter_mut().find(|slot| slot.id == id)
    }

    fn find_slot_by_id(&self, id: HandSlotId) -> Option<&HandSlot> {
        self.slots.iter().find(|slot| slot.id == id)
    }

    fn active_slots(&self) -> impl Iterator<Item = &HandSlot> {
        self.slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
    }
    fn create_barricade_template() -> TowerTemplate {
        DEFAULT_BARRICADE_TEMPLATE()
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

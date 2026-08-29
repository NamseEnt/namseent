use crate::PresentationInstant;
mod hand_slot;

pub use crate::animation::xy_with_spring;
use crate::{card::Card, game_state::tower::TowerTemplate};
use hand_slot::HandSlot;
pub use hand_slot::HandSlotId;
use namui::*;
use std::{any::Any, cmp::Ordering, fmt::Debug};

pub const HAND_SLOT_WH: Wh<Px> = Wh::new(px(112.), px(152.));
pub const HAND_WH: Wh<Px> = Wh::new(px(600.), px(160.));

// 레이아웃 관련 상수들
const DEFAULT_SLOT_GAP: Px = px(8.0);

#[derive(Clone, Debug, PartialEq, PartialOrd, State)]
pub enum HandItem {
    Card(Card),
    Tower(TowerTemplate),
}

impl HandItem {
    pub(crate) fn to_core_state(&self) -> td_core::HandItemState {
        match self {
            Self::Card(card) => td_core::HandItemState::Card(card.to_core_state()),
            Self::Tower(tower) => td_core::HandItemState::Tower(tower.to_core_state()),
        }
    }

    pub(crate) fn from_core_state(state: td_core::HandItemState) -> Option<Self> {
        Some(match state {
            td_core::HandItemState::Card(card) => Self::Card(Card::from_core_state(card)?),
            td_core::HandItemState::Tower(tower) => {
                Self::Tower(TowerTemplate::from_core_state(tower)?)
            }
        })
    }

    pub fn as_card(&self) -> Option<&Card> {
        match self {
            HandItem::Card(card) => Some(card),
            HandItem::Tower(_) => None,
        }
    }

    pub fn as_tower(&self) -> Option<&TowerTemplate> {
        match self {
            HandItem::Card(_) => None,
            HandItem::Tower(tower) => Some(tower),
        }
    }
}

#[derive(Default, Clone, Debug, State)]
pub struct Hand<Item: State + Debug> {
    slots: Vec<HandSlot<Item>>,
}
impl<Item: State + PartialOrd + Debug> Hand<Item> {
    #[cfg(test)]
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        let slots = items
            .into_iter()
            .enumerate()
            .map(|(index, item)| HandSlot::new(HandSlotId::from_raw(index + 1), item))
            .collect();
        let mut hand = Self { slots };
        hand.calculate_slot_xy();
        hand
    }
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn delete_slots(&mut self, ids: &[HandSlotId]) {
        let presentation_instant = PresentationInstant::capture();
        // 삭제할 슬롯들에 exit 애니메이션 시작
        for slot in self.slots.iter_mut() {
            if ids.contains(&slot.id) {
                slot.start_exit_animation(presentation_instant);
                slot.selected = false; // 선택 해제
            }
        }
        self.calculate_slot_xy();
    }

    pub fn remove_completed_exit_animations(&mut self, presentation_instant: PresentationInstant) {
        // 완료된 exit 애니메이션이 있는지 먼저 확인
        let has_completed_animations = self
            .slots
            .iter()
            .any(|slot| slot.is_exit_animation_complete(presentation_instant));

        // 완료된 애니메이션이 있을 때만 retain 실행
        if has_completed_animations {
            self.slots
                .retain(|slot| !slot.is_exit_animation_complete(presentation_instant));
        }
    }

    pub fn update(&mut self, presentation_instant: PresentationInstant) {
        self.remove_completed_exit_animations(presentation_instant);
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

    #[cfg(any(test, feature = "debug-tools"))]
    pub fn select_slot(&mut self, id: HandSlotId) {
        if let Some(slot) = self.find_slot_by_id_mut(id) {
            if slot.exit_animation.is_some() {
                return; // exit 애니메이션 중인 슬롯은 선택 불가
            }
            slot.selected = true;
        }
    }

    #[cfg(any(test, feature = "debug-tools"))]
    pub fn get_slot_id_by_index(&self, index: usize) -> Option<HandSlotId> {
        self.slots.get(index).map(|slot| slot.id)
    }

    #[cfg(feature = "debug-tools")]
    pub fn is_empty(&self) -> bool {
        self.active_slots().next().is_none()
    }

    pub fn get_items(&self, slot_ids: &[HandSlotId]) -> impl Iterator<Item = &Item> {
        slot_ids
            .iter()
            .map(|id| &self.slots.iter().find(|slot| slot.id == *id).unwrap().item)
    }

    #[cfg(feature = "debug-tools")]
    pub fn push(&mut self, item: Item) {
        let id = self.allocate_slot_id();
        self.slots
            .push(HandSlot::new(HandSlotId::from_raw(id), item));
        self.calculate_slot_xy();
    }

    #[cfg(feature = "debug-tools")]
    fn allocate_slot_id(&mut self) -> usize {
        self.slots
            .iter()
            .map(|slot| slot.id.raw())
            .max()
            .unwrap_or(0)
            .saturating_add(1)
            .max(1)
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
    #[cfg(any(test, feature = "debug-tools"))]
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

impl Hand<HandItem> {
    pub(crate) fn to_core_state(&self) -> td_core::HandState {
        td_core::HandState {
            slots: self
                .slots
                .iter()
                .filter(|slot| slot.exit_animation.is_none())
                .map(|slot| td_core::HandSlotState {
                    id: slot.id.raw(),
                    item: slot.item.to_core_state(),
                    selected: slot.selected,
                })
                .collect(),
            next_hand_slot_id: self
                .slots
                .iter()
                .map(|slot| slot.id.raw())
                .max()
                .unwrap_or(0)
                .saturating_add(1)
                .max(1),
        }
    }

    pub(crate) fn from_core_state(
        state: td_core::HandState,
        presentation_source: Option<&Self>,
    ) -> Option<Self> {
        Self::from_core_state_at(state, presentation_source, PresentationInstant::capture())
    }

    pub(crate) fn from_core_state_at(
        state: td_core::HandState,
        presentation_source: Option<&Self>,
        presentation_instant: PresentationInstant,
    ) -> Option<Self> {
        if state.slots.iter().any(|slot| slot.id == 0) {
            return None;
        }
        let mut ids = Vec::with_capacity(state.slots.len());
        let mut slots = state
            .slots
            .into_iter()
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
                let mut presentation_slot = hand_slot::HandSlot::from_raw(
                    presentation
                        .map(|slot| slot.id)
                        .unwrap_or_else(|| HandSlotId::from_raw(slot.id)),
                    HandItem::from_core_state(slot.item)?,
                    slot.selected,
                    presentation,
                );
                if presentation.is_some() {
                    presentation_slot.exit_animation = None;
                }
                Some(presentation_slot)
            })
            .collect::<Option<Vec<_>>>()?;
        if let Some(source) = presentation_source {
            for slot in &source.slots {
                if !ids.contains(&slot.id.raw()) {
                    let mut exiting = slot.clone();
                    exiting.selected = false;
                    if exiting.exit_animation.is_none() {
                        exiting.start_exit_animation(presentation_instant);
                    }
                    slots.push(exiting);
                }
            }
        }
        let mut hand = Self { slots };
        hand.calculate_slot_xy();
        Some(hand)
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
        for slot in hand.slots.iter().rev() {
            ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                .add_with_key(slot.id, slot)
                .attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };
                    if !event.is_local_xy_in() {
                        return;
                    }
                    if slot.exit_animation.is_some() {
                        return;
                    }
                    event.stop_propagation();
                    (on_click)(slot.id);
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn raw_state_round_trip_preserves_hand_items_and_selection() {
        let mut hand = Hand::new([
            HandItem::Card(Card::new(Rank::Ace, Suit::Spades)),
            HandItem::Card(Card::new(Rank::King, Suit::Hearts)),
        ]);
        let selected = hand.get_slot_id_by_index(0).expect("first hand slot");
        hand.select_slot(selected);

        let raw = hand.to_core_state();
        let restored = Hand::from_core_state(raw.clone(), Some(&hand)).expect("valid hand state");

        assert_eq!(restored.to_core_state(), raw);
    }

    #[test]
    fn raw_state_rejects_duplicate_slot_ids() {
        let hand = Hand::new([HandItem::Card(Card::new(Rank::Ace, Suit::Spades))]);
        let mut raw = hand.to_core_state();
        raw.slots.push(raw.slots[0].clone());

        assert!(Hand::from_core_state(raw, None).is_none());
    }

    #[test]
    fn missing_core_slot_is_retained_as_non_interactive_exit_entry() {
        let hand = Hand::new([
            HandItem::Card(Card::new(Rank::Ace, Suit::Spades)),
            HandItem::Card(Card::new(Rank::King, Suit::Hearts)),
        ]);
        let removed_id = hand.get_slot_id_by_index(0).expect("first slot");
        let retained_id = hand.get_slot_id_by_index(1).expect("second slot");
        let mut raw = hand.to_core_state();
        raw.slots.retain(|slot| slot.id != removed_id.raw());

        let reconciled = Hand::from_core_state_at(raw, Some(&hand), PresentationInstant::zero())
            .expect("valid hand state");

        assert_eq!(reconciled.active_slot_ids(), vec![retained_id]);
        assert_eq!(reconciled.slots.len(), 2);
        assert!(
            reconciled
                .slots
                .iter()
                .any(|slot| { slot.id == removed_id && slot.exit_animation.is_some() })
        );
    }
}

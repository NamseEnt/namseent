use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

mod behaviors;
mod definition;

use definition::card_service_definition;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardSelectionFilterState {
    Any,
    Face,
    Number,
    Rank(u8),
    Engraved,
    NotEngraved,
    And(Vec<CardSelectionFilterState>),
    Or(Vec<CardSelectionFilterState>),
}

impl CardSelectionFilterState {
    pub fn matches(&self, card: &CardState) -> bool {
        match self {
            Self::Any => true,
            Self::Face => (9..=11).contains(&card.rank),
            Self::Number => card.rank <= 8,
            Self::Rank(rank) => card.rank == *rank,
            Self::Engraved => card.engraving.is_some(),
            Self::NotEngraved => card.engraving.is_none(),
            Self::And(filters) => filters.iter().all(|filter| filter.matches(card)),
            Self::Or(filters) => filters.iter().any(|filter| filter.matches(card)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardServiceSelectionStepState {
    pub count: usize,
    pub filter: CardSelectionFilterState,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardServiceSelectionState {
    pub service_kind: u8,
    pub steps: Vec<CardServiceSelectionStepState>,
    pub current_step: usize,
    pub selected_card_ids: Vec<Vec<usize>>,
}

impl CardServiceSelectionState {
    pub fn service_key(service_kind: crate::CardServiceKind) -> &'static str {
        service_kind.key()
    }

    pub fn service_key_raw(service_kind: u8) -> Option<&'static str> {
        crate::CardServiceKind::from_raw(service_kind).map(Self::service_key)
    }

    pub fn service_kind(&self) -> Option<crate::CardServiceKind> {
        crate::CardServiceKind::from_raw(self.service_kind)
    }

    pub fn new(service_kind: crate::CardServiceKind) -> Option<Self> {
        let definition = card_service_definition(service_kind)?;
        Some(Self::from_definition(service_kind, definition))
    }

    fn from_definition(
        service_kind: crate::CardServiceKind,
        definition: &'static definition::CardServiceDefinition,
    ) -> Self {
        let steps = (definition.selection_steps)();
        let selected_card_ids = steps.iter().map(|_| Vec::new()).collect();
        Self {
            service_kind: service_kind.raw(),
            steps,
            current_step: 0,
            selected_card_ids,
        }
    }

    pub fn new_raw(service_kind: u8) -> Option<Self> {
        let kind = crate::CardServiceKind::from_raw(service_kind)?;
        let definition = definition::card_service_definition_raw(service_kind)?;
        Some(Self::from_definition(kind, definition))
    }

    pub fn current_step(&self) -> Option<&CardServiceSelectionStepState> {
        self.steps.get(self.current_step)
    }

    pub fn validate(
        &self,
        service_kind: crate::CardServiceKind,
        deck: &DeckState,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError> {
        if self.service_kind != service_kind.raw() {
            return Err(crate::CommandError::InvalidSelection);
        }
        let definition =
            card_service_definition(service_kind).ok_or(crate::CommandError::InvalidSelection)?;
        (definition.validate)(self, deck, selected_card_ids)
    }
}

impl crate::CoreState {
    pub fn apply_card_service_selection_mutation(
        &mut self,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError> {
        let service_kind = self
            .pending_card_service_kind
            .ok_or(crate::CommandError::InvalidFlow)?;
        let service_kind = crate::CardServiceKind::from_raw(service_kind)
            .ok_or(crate::CommandError::InvalidCardServiceKind { raw: service_kind })?;
        let selection =
            CardServiceSelectionState::new(service_kind).ok_or(crate::CommandError::InvalidFlow)?;
        selection.validate(service_kind, &self.deck, selected_card_ids)?;
        let definition =
            card_service_definition(service_kind).ok_or(crate::CommandError::Rejected)?;
        (definition.apply)(self, selected_card_ids)?;
        self.clear_card_service_selection();
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardState {
    pub id: usize,
    pub suit: u8,
    pub rank: u8,
    pub polish_pct_raw: i64,
    pub engraving: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeckState {
    pub revision: usize,
    pub next_card_id: usize,
    pub all_cards: Vec<CardState>,
    pub draw_pile: Vec<CardState>,
    pub discard_pile: Vec<CardState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardServicePurchaseBlockReason {
    NoEngravedCard,
    NotEnoughUnengravedCards { required: usize, available: usize },
}

impl DeckState {
    pub(crate) fn prepare_draw_pile(&mut self, rng: &mut ChaCha8Rng) {
        self.revision = self.revision.wrapping_add(1);
        self.draw_pile = self.all_cards.clone();
        self.discard_pile.clear();
        self.draw_pile.shuffle(rng);
    }

    pub(crate) fn draw(&mut self, rng: &mut ChaCha8Rng, count: usize) -> Vec<CardState> {
        self.revision = self.revision.wrapping_add(1);
        let mut cards = Vec::new();
        let mut draw_count = count;
        while cards.len() < draw_count {
            let Some(card) = self.draw_pile.pop() else {
                if self.discard_pile.is_empty() {
                    break;
                }
                self.draw_pile = std::mem::take(&mut self.discard_pile);
                self.draw_pile.shuffle(rng);
                continue;
            };
            if card.engraving == Some(3) {
                draw_count = draw_count.saturating_add(1);
            }
            cards.push(card);
        }

        if cards.iter().any(|card| card.engraving == Some(0)) {
            let mut pulled = Vec::new();
            self.draw_pile.retain(|card| {
                if card.engraving == Some(0) {
                    pulled.push(card.clone());
                    false
                } else {
                    true
                }
            });
            cards.extend(pulled);
        }
        cards
    }

    pub(crate) fn discard(&mut self, cards: impl IntoIterator<Item = CardState>) {
        self.revision = self.revision.wrapping_add(1);
        self.discard_pile.extend(cards);
    }

    pub fn get_card(&self, card_id: usize) -> Option<CardState> {
        self.all_cards
            .iter()
            .find(|card| card.id == card_id)
            .cloned()
    }

    pub fn add_card(&mut self, mut card: CardState) {
        self.revision = self.revision.wrapping_add(1);
        card.id = self.next_card_id;
        self.next_card_id = self.next_card_id.wrapping_add(1);
        self.all_cards.push(card.clone());
        if self.draw_pile.is_empty() {
            self.discard_pile.push(card);
        } else {
            self.draw_pile.push(card);
        }
    }

    pub fn remove_card(&mut self, card_id: usize) -> Option<CardState> {
        let index = self.all_cards.iter().position(|card| card.id == card_id)?;
        self.revision = self.revision.wrapping_add(1);
        let removed = self.all_cards.remove(index);
        self.draw_pile.retain(|card| card.id != card_id);
        self.discard_pile.retain(|card| card.id != card_id);
        Some(removed)
    }

    pub fn modify_card<F>(&mut self, card_id: usize, mut f: F) -> Option<CardState>
    where
        F: FnMut(&mut CardState),
    {
        let index = self.all_cards.iter().position(|card| card.id == card_id)?;
        {
            let card = &mut self.all_cards[index];
            f(card);
        }
        self.revision = self.revision.wrapping_add(1);
        Some(self.all_cards[index].clone())
    }
}

pub fn purchase_block_reasons(
    kind: crate::CardServiceKind,
    deck: &DeckState,
) -> Vec<CardServicePurchaseBlockReason> {
    card_service_definition(kind).map_or_else(Vec::new, |definition| {
        (definition.purchase_block_reasons)(deck)
    })
}

pub fn purchase_block_reasons_raw(
    raw: u8,
    deck: &DeckState,
) -> Result<Vec<CardServicePurchaseBlockReason>, crate::CommandError> {
    let kind = crate::CardServiceKind::from_raw(raw)
        .ok_or(crate::CommandError::InvalidCardServiceKind { raw })?;
    Ok(purchase_block_reasons(kind, deck))
}

pub fn purchase_is_available(kind: crate::CardServiceKind, deck: &DeckState) -> bool {
    purchase_block_reasons(kind, deck).is_empty()
}

pub fn purchase_is_available_raw(raw: u8, deck: &DeckState) -> Result<bool, crate::CommandError> {
    Ok(purchase_block_reasons_raw(raw, deck)?.is_empty())
}

#[cfg(test)]
mod purchase_tests {
    use super::*;

    fn core_state() -> crate::CoreState {
        crate::CoreState::new_initial(
            crate::GameConfigState {
                player: crate::PlayerConfigState {
                    max_hp_raw: 60_000,
                    starting_gold: 100,
                    starting_hp_raw: 60_000,
                    base_dice_chance: 3,
                    max_stages: 5,
                    base_hand_slots: 5,
                },
                towers: crate::TowerConfigState {
                    entries: vec![crate::TowerConfigEntryState {
                        kind: 0,
                        damage_raw: 1_000,
                        range_raw: 1_000,
                        cooldown_ms: 1_000,
                    }],
                },
                monsters: crate::MonsterConfigState {
                    stats: Vec::new(),
                    stage_waves: Vec::new(),
                },
            },
            7,
        )
    }

    fn deck(engraved_card_count: usize, unengraved_card_count: usize) -> DeckState {
        let all_cards = (0..engraved_card_count)
            .map(|id| CardState {
                id,
                suit: 0,
                rank: 0,
                polish_pct_raw: 0,
                engraving: Some(0),
            })
            .chain(
                (engraved_card_count..engraved_card_count + unengraved_card_count).map(|id| {
                    CardState {
                        id,
                        suit: 0,
                        rank: 0,
                        polish_pct_raw: 0,
                        engraving: None,
                    }
                }),
            )
            .collect();
        DeckState {
            revision: 0,
            next_card_id: engraved_card_count + unengraved_card_count,
            all_cards,
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    #[test]
    fn purchase_block_reasons_are_stable_for_each_restricted_kind() {
        assert_eq!(
            purchase_block_reasons(crate::CardServiceKind::MagicWand, &deck(0, 0)),
            vec![
                CardServicePurchaseBlockReason::NoEngravedCard,
                CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                    required: 1,
                    available: 0,
                },
            ]
        );
        assert_eq!(
            purchase_block_reasons(crate::CardServiceKind::Pliers, &deck(0, 2)),
            vec![CardServicePurchaseBlockReason::NoEngravedCard]
        );
        assert_eq!(
            purchase_block_reasons(crate::CardServiceKind::Magnet, &deck(1, 1)),
            vec![CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                required: 2,
                available: 1,
            }]
        );
        assert_eq!(
            purchase_block_reasons(crate::CardServiceKind::Cactus, &deck(1, 0)),
            vec![CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                required: 1,
                available: 0,
            }]
        );
        assert!(purchase_block_reasons(crate::CardServiceKind::LongSword, &deck(0, 0)).is_empty());
    }

    #[test]
    fn purchase_availability_matches_block_reasons() {
        for &kind in crate::CardServiceKind::ALL {
            let deck = deck(1, 2);
            assert_eq!(
                purchase_is_available(kind, &deck),
                purchase_block_reasons(kind, &deck).is_empty()
            );
        }
    }

    #[test]
    fn every_card_service_has_a_stable_selection_contract() {
        let expected = [
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 2),
            (9, 1),
            (10, 1),
            (11, 1),
            (12, 1),
            (13, 1),
            (14, 1),
            (15, 1),
        ];

        for (raw, step_count) in expected {
            let kind = crate::CardServiceKind::from_raw(raw).expect("catalog service kind");
            let selection =
                CardServiceSelectionState::new(kind).expect("catalog service must be supported");
            assert_eq!(selection.steps.len(), step_count, "service kind {kind:?}");
            assert_eq!(selection.selected_card_ids.len(), step_count);
            assert_eq!(CardServiceSelectionState::service_key(kind), kind.key());
        }
    }

    #[test]
    fn every_valid_kind_has_one_registry_definition() {
        for kind in crate::CardServiceKind::ALL {
            let definition = card_service_definition(*kind).expect("valid kind is registered");
            assert_eq!(definition.kind, *kind);
            assert_eq!(CardServiceSelectionState::service_key(*kind), kind.key());
        }
        assert!(definition::card_service_definition_raw(16).is_none());
        assert!(CardServiceSelectionState::new_raw(u8::MAX).is_none());
    }

    #[test]
    fn unknown_raw_kind_is_rejected_at_the_card_service_boundary() {
        let mut state = core_state();
        assert_eq!(
            state.begin_card_service_selection_raw(u8::MAX),
            Err(crate::CommandError::InvalidCardServiceKind { raw: u8::MAX })
        );
        assert_eq!(
            purchase_block_reasons_raw(u8::MAX, &deck(1, 1)),
            Err(crate::CommandError::InvalidCardServiceKind { raw: u8::MAX })
        );
        assert_eq!(
            purchase_is_available_raw(u8::MAX, &deck(1, 1)),
            Err(crate::CommandError::InvalidCardServiceKind { raw: u8::MAX })
        );
    }

    #[test]
    fn registry_selection_and_purchase_reasons_are_deterministic() {
        let deck = deck(1, 2);
        for &kind in crate::CardServiceKind::ALL {
            assert_eq!(
                CardServiceSelectionState::new(kind),
                CardServiceSelectionState::new(kind),
                "selection plan for service kind {kind:?}"
            );
            assert_eq!(
                purchase_block_reasons(kind, &deck),
                purchase_block_reasons(kind, &deck),
                "purchase reasons for service kind {kind:?}"
            );
        }
    }

    #[test]
    fn selection_state_serde_preserves_every_canonical_raw_kind() {
        for &kind in crate::CardServiceKind::ALL {
            let selection = CardServiceSelectionState::new(kind).expect("registered kind");
            let encoded = serde_json::to_string(&selection).expect("selection serializes");
            let decoded: CardServiceSelectionState =
                serde_json::from_str(&encoded).expect("selection deserializes");
            assert_eq!(decoded, selection);
            assert_eq!(decoded.service_kind(), Some(kind));
        }
    }

    #[test]
    fn representative_card_services_apply_only_their_core_mutation() {
        let cases = [
            (crate::CardServiceKind::LongSword, vec![vec![0]]),
            (crate::CardServiceKind::Brush, vec![vec![36]]),
            (crate::CardServiceKind::Eraser, vec![vec![0]]),
            (crate::CardServiceKind::MagicWand, vec![vec![0], vec![1]]),
            (crate::CardServiceKind::Screwdriver, vec![vec![0]]),
            (crate::CardServiceKind::Magnet, vec![vec![0, 1]]),
            (crate::CardServiceKind::Cactus, vec![vec![0]]),
        ];

        for (kind, selected) in cases {
            let mut state = core_state();
            state
                .edit_snapshot(|parts| {
                    if kind == crate::CardServiceKind::MagicWand {
                        parts.deck.all_cards[0].engraving = Some(1);
                        parts.deck.all_cards[1].engraving = None;
                    }
                    if kind == crate::CardServiceKind::Magnet {
                        parts.deck.all_cards[0].engraving = None;
                        parts.deck.all_cards[1].engraving = None;
                    }
                    if kind == crate::CardServiceKind::Cactus {
                        parts.deck.all_cards[0].engraving = None;
                    }
                })
                .expect("card service fixture must be valid");
            state
                .begin_card_service_selection(kind)
                .expect("service should begin");
            let before = state.deck().all_cards.clone();
            state
                .apply_card_service_selection_mutation(&selected)
                .expect("service selection should apply");

            match kind {
                crate::CardServiceKind::LongSword | crate::CardServiceKind::Brush => {
                    let card = state
                        .deck()
                        .get_card(selected[0][0])
                        .expect("selected card");
                    if kind == crate::CardServiceKind::LongSword {
                        assert_eq!(card.id, 0);
                        assert_eq!(card.suit, 0);
                        assert_eq!(card.polish_pct_raw, 2_000_000);
                    } else {
                        assert_eq!(card.id, 36);
                        assert_eq!(card.suit, 1);
                        assert_eq!(card.polish_pct_raw, 3_000_000);
                    }
                }
                crate::CardServiceKind::Eraser => assert!(state.deck().get_card(0).is_none()),
                crate::CardServiceKind::MagicWand => {
                    assert_eq!(state.deck().get_card(0).unwrap().engraving, None);
                    assert_eq!(state.deck().get_card(1).unwrap().engraving, Some(1));
                }
                crate::CardServiceKind::Screwdriver => {
                    assert_eq!(state.deck().get_card(0).unwrap().rank, 1)
                }
                crate::CardServiceKind::Magnet => {
                    assert_eq!(state.deck().get_card(0).unwrap().engraving, Some(0));
                    assert_eq!(state.deck().get_card(1).unwrap().engraving, Some(0));
                }
                crate::CardServiceKind::Cactus => {
                    assert_eq!(state.deck().get_card(0).unwrap().engraving, Some(2))
                }
                _ => unreachable!(),
            }
            assert_ne!(state.deck().all_cards, before);
            assert_eq!(state.pending_card_service_kind(), None);
        }
    }

    #[test]
    fn card_service_request_event_and_hash_are_seed_deterministic() {
        let mut first = core_state();
        let mut second = core_state();
        for state in [&mut first, &mut second] {
            state
                .begin_card_service_selection(crate::CardServiceKind::LongSword)
                .expect("long sword should begin");
            state
                .apply_card_service_selection_mutation(&[vec![0]])
                .expect("long sword should apply");
        }
        let first_event: Vec<_> = first.drain_events().collect();
        let second_event: Vec<_> = second.drain_events().collect();
        assert_eq!(first_event.len(), 1);
        assert_eq!(first_event, second_event);
        assert_eq!(
            crate::event_digest(&first_event),
            crate::event_digest(&second_event)
        );
        assert_eq!(
            crate::authoritative_hash(&first),
            crate::authoritative_hash(&second)
        );
    }
}

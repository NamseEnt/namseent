use super::entity::{CATEGORICAL_FIELDS, EntityRow, EntitySet};
use super::observation::{ENTITY_SET_COUNT, TypedObservation};
use crate::simulator::environment::{ActionKind, AgentAction, LegalAction, Observation};

fn tower_identity(
    tower: &crate::simulator::environment::TowerTemplateObservation,
) -> ([u32; 3], [f32; 2]) {
    (
        [
            tower.kind_id as u32,
            tower
                .suit
                .as_deref()
                .map(crate::simulator::ml::vocabulary::suit_id)
                .unwrap_or(0) as u32,
            tower
                .rank
                .as_deref()
                .map(crate::simulator::ml::vocabulary::rank_id)
                .unwrap_or(0) as u32,
        ],
        [
            tower.damage_raw as f32 / 10_000.0,
            tower.rerolled_count as f32 / 20.0,
        ],
    )
}

fn hand_tower(
    observation: &Observation,
    hand_slot_index: usize,
) -> Option<&crate::simulator::environment::TowerTemplateObservation> {
    observation.hand.iter().find_map(|item| {
        if item.index != hand_slot_index {
            return None;
        }
        match &item.item {
            crate::simulator::environment::HandItemObservation::Tower(tower) => Some(tower),
            crate::simulator::environment::HandItemObservation::Card(_) => None,
        }
    })
}

fn placement_route_features(observation: &Observation, left: usize, top: usize) -> (f32, f32) {
    let nearest = observation
        .route_coords
        .iter()
        .min_by_key(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top));
    nearest.map_or((1.0, 0.0), |coord| {
        (
            (coord.x.abs_diff(left) + coord.y.abs_diff(top)) as f32
                / (observation.map_width + observation.map_height).max(1) as f32,
            coord.index as f32 / observation.route_coords.len().max(1) as f32,
        )
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaddedEntityBatch {
    pub categorical: Vec<u32>,
    pub numeric: Vec<f32>,
    pub mask: Vec<f32>,
    pub batch_size: usize,
    pub max_entities: usize,
    pub numeric_width: usize,
}

impl PaddedEntityBatch {
    pub fn from_sets(sets: &[EntitySet]) -> Self {
        let batch_size = sets.len();
        let max_entities = sets
            .iter()
            .map(|set| set.rows.len())
            .max()
            .unwrap_or(0)
            .max(1);
        let numeric_width = sets
            .iter()
            .map(EntitySet::numeric_width)
            .max()
            .unwrap_or(0)
            .max(5);
        let mut categorical = vec![0; batch_size * max_entities * CATEGORICAL_FIELDS];
        let mut numeric = vec![0.0; batch_size * max_entities * numeric_width];
        let mut mask = vec![0.0; batch_size * max_entities];
        for (batch_index, set) in sets.iter().enumerate() {
            for (entity_index, row) in set.rows.iter().enumerate() {
                let mask_index = batch_index * max_entities + entity_index;
                mask[mask_index] = 1.0;
                let categorical_start = mask_index * CATEGORICAL_FIELDS;
                categorical[categorical_start..categorical_start + CATEGORICAL_FIELDS]
                    .copy_from_slice(&row.categorical);
                let numeric_start = mask_index * numeric_width;
                numeric[numeric_start..numeric_start + row.numeric.len()]
                    .copy_from_slice(&row.numeric);
            }
        }
        Self {
            categorical,
            numeric,
            mask,
            batch_size,
            max_entities,
            numeric_width,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.batch_size == 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyBatch {
    pub state_sets: [PaddedEntityBatch; ENTITY_SET_COUNT],
    pub candidate_rows: Vec<Vec<EntityRow>>,
    pub legal_candidate_mask: Vec<Vec<bool>>,
}

impl PolicyBatch {
    pub fn from_observation(observation: &Observation, legal: &[LegalAction]) -> Self {
        let typed = TypedObservation::from_observation(observation);
        let state_sets =
            std::array::from_fn(|index| PaddedEntityBatch::from_sets(&[typed.sets[index].clone()]));
        let candidate_rows = legal
            .iter()
            .map(|legal| candidate_entity_rows(observation, &legal.action))
            .collect::<Vec<_>>();
        Self {
            state_sets,
            legal_candidate_mask: vec![vec![true; candidate_rows.len()]],
            candidate_rows,
        }
    }
}

pub fn candidate_rows_for_legal_actions(
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Vec<EntityRow> {
    legal_actions
        .iter()
        .map(|legal| {
            candidate_entity_rows(observation, &legal.action)
                .pop()
                .expect("candidate entity row must exist")
        })
        .collect()
}

pub fn candidate_entity_rows(observation: &Observation, action: &AgentAction) -> Vec<EntityRow> {
    let kind = action.kind().index() as u32 + 1;
    let (first, second, third) = match action {
        AgentAction::PurchaseShopItem { slot_index }
        | AgentAction::SelectHandCard {
            hand_slot_index: slot_index,
        }
        | AgentAction::DeselectHandCard {
            hand_slot_index: slot_index,
        }
        | AgentAction::SelectCardServiceCard {
            card_index: slot_index,
        }
        | AgentAction::UseInventoryItem {
            item_index: slot_index,
        }
        | AgentAction::SelectTreasure {
            option_index: slot_index,
        } => (*slot_index as u32 + 1, 0, 0),
        AgentAction::PlaceTower {
            hand_slot_index,
            left,
            top,
        } => (
            *hand_slot_index as u32 + 1,
            *left as u32 + 1,
            *top as u32 + 1,
        ),
        AgentAction::Reroll {
            selected_slot_indices,
        }
        | AgentAction::SelectTower {
            selected_slot_indices,
        } => (
            selected_slot_indices.len() as u32,
            selected_slot_indices.iter().copied().sum::<usize>() as u32,
            selected_slot_indices
                .iter()
                .copied()
                .max()
                .map_or(0, |index| index as u32 + 1),
        ),
        AgentAction::RemoveTower { tower_id } => (*tower_id as u32, (*tower_id >> 32) as u32, 0),
        AgentAction::StartSelectingTower
        | AgentAction::BeginRerollSelection
        | AgentAction::BeginTowerSelection
        | AgentAction::ConfirmCardSelection
        | AgentAction::CancelCardSelection
        | AgentAction::ConfirmCardServiceSelection
        | AgentAction::StartDefense
        | AgentAction::Continue => (0, 0, 0),
    };
    let mut categorical = [kind, first, second, third];
    let mut numeric = vec![
        kind as f32 / ActionKind::COUNT as f32,
        first as f32 / 1024.0,
        second as f32 / 1024.0,
        third as f32 / 1024.0,
        1.0,
    ];
    match action {
        AgentAction::PlaceTower {
            hand_slot_index,
            left,
            top,
        } => {
            if let Some(tower) = hand_tower(observation, *hand_slot_index) {
                let (_, stats) = tower_identity(tower);
                categorical[1] = *hand_slot_index as u32 + 1;
                categorical[2] = *left as u32 + 1;
                categorical[3] = *top as u32 + 1;
                numeric[1] = stats[0];
                numeric[2] = stats[1];
                numeric[3] = placement_route_features(observation, *left, *top).0;
                numeric[4] = crate::simulator::ml::features::placement_coverage(
                    observation,
                    *left,
                    *top,
                    &tower.kind,
                );
            }
        }
        AgentAction::SelectHandCard { hand_slot_index }
        | AgentAction::DeselectHandCard { hand_slot_index } => {
            if let Some(item) = observation
                .hand
                .iter()
                .find(|item| item.index == *hand_slot_index)
                && let crate::simulator::environment::HandItemObservation::Card(card) = &item.item
            {
                categorical[1] = crate::simulator::ml::vocabulary::suit_id(&card.suit) as u32;
                categorical[2] = crate::simulator::ml::vocabulary::rank_id(&card.rank) as u32;
                categorical[3] = card.engraving.as_deref().map_or(0, |value| {
                    crate::simulator::ml::vocabulary::engraving_key_id(value) as u32
                });
                numeric[1] = card.polish_pct_raw as f32 / 1_000.0;
                numeric[2] = item.selected as u8 as f32;
                numeric[3] = matches!(action, AgentAction::SelectHandCard { .. }) as u8 as f32;
            }
            numeric[4] = match observation.card_selection_purpose.as_deref() {
                Some("reroll") => 0.5,
                Some("build_tower") => 1.0,
                _ => 0.0,
            };
        }
        AgentAction::SelectCardServiceCard { card_index } => {
            if let Some(card) = observation.deck.all_cards.get(*card_index) {
                categorical[1] = crate::simulator::ml::vocabulary::suit_id(&card.suit) as u32;
                categorical[2] = crate::simulator::ml::vocabulary::rank_id(&card.rank) as u32;
                categorical[3] = card.engraving.as_deref().map_or(0, |value| {
                    crate::simulator::ml::vocabulary::engraving_key_id(value) as u32
                });
                numeric[1] = card.polish_pct_raw as f32 / 1_000.0;
                numeric[2] = observation
                    .card_service
                    .as_ref()
                    .is_some_and(|service| service.selected_card_indices.contains(card_index))
                    as u8 as f32;
            }
        }
        _ => {}
    }
    vec![EntityRow::new(categorical, numeric)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulator::ml::encoding::entity::EntityRow;

    #[test]
    fn heterogeneous_sets_are_padded_without_leaking_values() {
        let sets = vec![
            EntitySet::new(vec![EntityRow::new([1, 2, 3, 4], vec![1.0, 2.0])]),
            EntitySet::new(vec![
                EntityRow::new([5, 6, 7, 8], vec![3.0, 4.0]),
                EntityRow::new([9, 10, 11, 12], vec![5.0, 6.0]),
            ]),
        ];
        let batch = PaddedEntityBatch::from_sets(&sets);
        assert_eq!(batch.batch_size, 2);
        assert_eq!(batch.max_entities, 2);
        assert_eq!(batch.mask, vec![1.0, 0.0, 1.0, 1.0]);
        assert_eq!(batch.numeric[2..4], [0.0, 0.0]);
    }

    #[test]
    fn empty_set_has_masked_padding() {
        let batch = PaddedEntityBatch::from_sets(&[EntitySet::default()]);
        assert_eq!(batch.max_entities, 1);
        assert_eq!(batch.numeric_width, 5);
        assert_eq!(batch.mask, vec![0.0]);
    }

    #[test]
    fn policy_batch_keeps_one_legal_candidate_per_row() {
        let environment = crate::simulator::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            7,
        );
        let legal = environment.legal_actions();
        let batch = PolicyBatch::from_observation(&environment.snapshot(), &legal);
        assert_eq!(batch.candidate_rows.len(), legal.len());
        assert!(
            batch.legal_candidate_mask[0]
                .iter()
                .all(|is_legal| *is_legal)
        );
        assert!(batch.candidate_rows.iter().all(|rows| rows.len() == 1));
    }

    #[test]
    fn flat_candidate_rows_preserve_legal_action_order() {
        let environment = crate::simulator::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            7,
        );
        let observation = environment.snapshot();
        let legal = environment.legal_actions();
        let batch = PolicyBatch::from_observation(&observation, &legal);
        let flat = candidate_rows_for_legal_actions(&observation, &legal);

        assert_eq!(
            batch
                .candidate_rows
                .iter()
                .map(|rows| rows.first().expect("candidate row").clone())
                .collect::<Vec<_>>(),
            flat
        );
    }

    #[test]
    fn candidate_encoding_distinguishes_typed_parameters() {
        let environment = crate::simulator::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            7,
        );
        let observation = environment.snapshot();
        let first = candidate_entity_rows(
            &observation,
            &AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 1,
                top: 2,
            },
        );
        let second = candidate_entity_rows(
            &observation,
            &AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 2,
                top: 2,
            },
        );
        assert_ne!(first, second);
        assert_ne!(first[0].categorical, second[0].categorical);
    }

    #[test]
    fn placement_candidate_encoding_distinguishes_route_features() {
        let environment = crate::simulator::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            7,
        );
        let observation = environment.snapshot();
        let first = candidate_entity_rows(
            &observation,
            &AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
        );
        let second = candidate_entity_rows(
            &observation,
            &AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 10,
            },
        );
        assert_ne!(first, second);
    }

    #[test]
    fn placement_candidate_encoding_includes_route_coverage() {
        let environment = crate::simulator::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            7,
        );
        let mut observation = environment.snapshot();
        let tower = crate::simulator::environment::TowerTemplateObservation {
            kind: "high".to_owned(),
            kind_id: 1,
            suit: None,
            rank: None,
            rerolled_count: 0,
            damage_raw: 1_000,
            used_cards: Vec::new(),
        };
        observation
            .hand
            .push(crate::simulator::environment::HandObservation {
                index: 0,
                selected: false,
                item: crate::simulator::environment::HandItemObservation::Tower(tower.clone()),
            });
        let tower_kind = tower.kind.clone();
        let (low_position, high_position) = (0..observation.map_height)
            .flat_map(|top| (0..observation.map_width).map(move |left| (left, top)))
            .min_by(|left, right| {
                crate::simulator::ml::features::placement_coverage(
                    &observation,
                    left.0,
                    left.1,
                    &tower_kind,
                )
                .total_cmp(&crate::simulator::ml::features::placement_coverage(
                    &observation,
                    right.0,
                    right.1,
                    &tower_kind,
                ))
            })
            .zip(
                (0..observation.map_height)
                    .flat_map(|top| (0..observation.map_width).map(move |left| (left, top)))
                    .max_by(|left, right| {
                        crate::simulator::ml::features::placement_coverage(
                            &observation,
                            left.0,
                            left.1,
                            &tower_kind,
                        )
                        .total_cmp(
                            &crate::simulator::ml::features::placement_coverage(
                                &observation,
                                right.0,
                                right.1,
                                &tower_kind,
                            ),
                        )
                    }),
            )
            .expect("coverage fixture positions");
        let first = candidate_entity_rows(
            &observation,
            &AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: low_position.0,
                top: low_position.1,
            },
        );
        let second = candidate_entity_rows(
            &observation,
            &AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: high_position.0,
                top: high_position.1,
            },
        );
        assert_ne!(first[0].numeric[4], second[0].numeric[4]);
    }

    #[test]
    fn candidate_kind_normalization_uses_action_kind_count() {
        let environment = crate::simulator::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            7,
        );
        let observation = environment.snapshot();
        let first = candidate_entity_rows(&observation, &AgentAction::StartSelectingTower);
        let last = candidate_entity_rows(&observation, &AgentAction::Continue);

        assert_eq!(first[0].numeric[0], 2.0 / ActionKind::COUNT as f32);
        assert_eq!(last[0].numeric[0], 18.0 / ActionKind::COUNT as f32);
    }
}

use super::vocabulary::{engraving_key_id, rank_id, shop_kind_id, suit_id, upgrade_key_id};
use crate::environment::{ActionKind, AgentAction, DecisionPoint, Observation};

pub const GLOBAL_FEATURE_COUNT: usize = 91;
pub const ACTION_FEATURE_COUNT: usize = ActionKind::COUNT + 11;

fn bounded_count(count: usize, scale: f32) -> f32 {
    let value = count as f32 / scale.max(1.0);
    value / (1.0 + value)
}

pub fn observation_features(observation: &Observation) -> Vec<f32> {
    let mut features = Vec::with_capacity(GLOBAL_FEATURE_COUNT);
    let decision_index = match observation.decision_point {
        DecisionPoint::Shop => 0,
        DecisionPoint::CardSelection => 1,
        DecisionPoint::CardServiceSelection => 2,
        DecisionPoint::TowerPlacement => 3,
        DecisionPoint::PreDefenseItem => 4,
        DecisionPoint::DamageResponseItem => 5,
        DecisionPoint::TreasureSelection => 6,
        DecisionPoint::Defense => 7,
        DecisionPoint::Terminal => 8,
    };
    for index in 0..9 {
        features.push((index == decision_index) as u8 as f32);
    }
    features.extend([
        (observation.card_selection_purpose.as_deref() == Some("reroll")) as u8 as f32,
        (observation.card_selection_purpose.as_deref() == Some("build_tower")) as u8 as f32,
        bounded_count(observation.selected_hand_slot_indices.len(), 4.0),
        observation.card_selection_confirmable as u8 as f32,
    ]);

    let max_hp = observation.max_hp_raw.max(1) as f32;
    let stage_total_hp = observation.stage_total_hp_raw.max(1) as f32;
    features.extend([
        observation.stage as f32 / 100.0,
        observation.sim_tick as f32 / 100_000.0,
        observation.hp_raw.max(0) as f32 / max_hp,
        observation.shield_raw.max(0) as f32 / max_hp,
        observation.gold as f32 / 10_000.0,
        observation.left_dice as f32 / 20.0,
        observation.rerolled_count as f32 / 20.0,
        observation.stage_progress_raw.max(0) as f32 / stage_total_hp,
        observation.stage_total_hp_raw.max(0) as f32 / 1_000_000.0,
        observation.active_monster_count as f32 / 100.0,
        observation.queued_monster_count as f32 / 100.0,
        observation.map_width as f32 / 32.0,
        observation.map_height as f32 / 32.0,
    ]);

    let card_items = observation
        .hand
        .iter()
        .filter_map(|item| match &item.item {
            crate::environment::HandItemObservation::Card(card) => Some(card),
            crate::environment::HandItemObservation::Tower(_) => None,
        })
        .collect::<Vec<_>>();
    let tower_items = observation
        .hand
        .iter()
        .filter_map(|item| match &item.item {
            crate::environment::HandItemObservation::Card(_) => None,
            crate::environment::HandItemObservation::Tower(tower) => Some(tower),
        })
        .collect::<Vec<_>>();
    features.extend([
        card_items.len() as f32 / 10.0,
        tower_items.len() as f32 / 10.0,
        observation.hand.iter().filter(|item| item.selected).count() as f32 / 10.0,
        mean(
            card_items
                .iter()
                .map(|card| card.polish_pct_raw as f32 / 1_000.0),
        ),
        mean(
            tower_items
                .iter()
                .map(|tower| tower.damage_raw as f32 / 10_000.0),
        ),
        mean(
            tower_items
                .iter()
                .map(|tower| tower.rerolled_count as f32 / 20.0),
        ),
    ]);

    features.extend([
        bounded_count(observation.deck.all_cards.len(), 32.0),
        bounded_count(observation.deck.draw_cards.len(), 32.0),
        bounded_count(observation.deck.discard_cards.len(), 32.0),
    ]);
    let deck_suit_counts =
        observation
            .deck
            .all_cards
            .iter()
            .fold([0_usize; 4], |mut counts, card| {
                let suit = suit_id(&card.suit);
                if (1..=4).contains(&suit) {
                    counts[suit as usize - 1] += 1;
                }
                counts
            });
    let deck_rank_counts =
        observation
            .deck
            .all_cards
            .iter()
            .fold([0_usize; 13], |mut counts, card| {
                let rank = rank_id(&card.rank);
                if (1..=13).contains(&rank) {
                    counts[rank as usize - 1] += 1;
                }
                counts
            });
    features.extend(deck_suit_counts.map(|count| bounded_count(count, 13.0)));
    features.extend([
        bounded_count(deck_rank_counts[..4].iter().sum(), 4.0),
        bounded_count(deck_rank_counts[4..8].iter().sum(), 4.0),
        bounded_count(deck_rank_counts[8..10].iter().sum(), 2.0),
        bounded_count(deck_rank_counts[10..13].iter().sum(), 3.0),
    ]);

    let shop_items = observation
        .shop
        .iter()
        .filter(|slot| slot.kind == "item")
        .count();
    let shop_upgrades = observation
        .shop
        .iter()
        .filter(|slot| slot.kind == "upgrade")
        .count();
    let shop_services = observation
        .shop
        .iter()
        .filter(|slot| slot.kind == "card_service")
        .count();
    features.extend([
        observation.shop.len() as f32 / 10.0,
        observation
            .shop
            .iter()
            .filter(|slot| slot.purchased)
            .count() as f32
            / 10.0,
        mean(
            observation
                .shop
                .iter()
                .map(|slot| slot.cost as f32 / 1_000.0),
        ),
        shop_items as f32 / 10.0,
        shop_upgrades as f32 / 10.0,
        shop_services as f32 / 10.0,
    ]);

    let tower_count = observation.towers.len().max(1) as f32;
    features.push(observation.inventory.len() as f32 / 20.0);
    features.extend([
        observation.towers.len() as f32 / 50.0,
        observation
            .tower_grid
            .iter()
            .filter(|tower| tower.is_some())
            .count() as f32
            / observation.tower_grid.len().max(1) as f32,
        observation
            .towers
            .iter()
            .map(|tower| tower.left as f32 / observation.map_width.max(1) as f32)
            .sum::<f32>()
            / tower_count,
        observation
            .towers
            .iter()
            .map(|tower| tower.top as f32 / observation.map_height.max(1) as f32)
            .sum::<f32>()
            / tower_count,
        mean(
            observation
                .towers
                .iter()
                .map(|tower| tower.template.damage_raw as f32 / 10_000.0),
        ),
    ]);

    let monster_count = observation.monsters.len().max(1) as f32;
    features.extend([
        observation.monsters.len() as f32 / 100.0,
        observation
            .monsters
            .iter()
            .map(|monster| monster.hp_raw.max(0) as f32 / monster.max_hp_raw.max(1) as f32)
            .sum::<f32>()
            / monster_count,
        observation
            .monsters
            .iter()
            .map(|monster| monster.route_progress_raw.max(0) as f32 / 1_000.0)
            .sum::<f32>()
            / monster_count,
        mean(
            observation
                .monsters
                .iter()
                .map(|monster| monster.max_hp_raw.max(0) as f32 / 100_000.0),
        ),
        mean(
            observation
                .monsters
                .iter()
                .map(|monster| monster.route_index as f32 / 100.0),
        ),
    ]);

    features.push(observation.treasure_options.len() as f32 / 10.0);
    let suit_counts = card_items.iter().fold([0_usize; 4], |mut counts, card| {
        let suit = suit_id(&card.suit);
        if (1..=4).contains(&suit) {
            counts[suit as usize - 1] += 1;
        }
        counts
    });
    let rank_counts = card_items.iter().fold([0_usize; 13], |mut counts, card| {
        let rank = rank_id(&card.rank);
        if (1..=13).contains(&rank) {
            counts[rank as usize - 1] += 1;
        }
        counts
    });
    features.extend(suit_counts.map(|count| count as f32 / 10.0));
    features.extend([
        rank_counts[..4].iter().sum::<usize>() as f32 / 10.0,
        rank_counts[4..8].iter().sum::<usize>() as f32 / 10.0,
        rank_counts[8..10].iter().sum::<usize>() as f32 / 10.0,
        rank_counts[10..13].iter().sum::<usize>() as f32 / 10.0,
    ]);
    if let Some(card_service) = &observation.card_service {
        features.extend([
            1.0,
            card_service.current_step as f32 / 10.0,
            card_service.step_count as f32 / 10.0,
            card_service.required_count as f32 / 10.0,
            card_service.selected_card_indices.len() as f32 / 20.0,
            card_service.candidate_card_indices.len() as f32 / 100.0,
        ]);
    } else {
        features.extend([0.0; 6]);
    }

    let modifiers = &observation.stage_modifiers;
    features.extend([
        modifiers.damage_multiplier_raw as f32 / 1_000.0,
        modifiers.damage_reduction_multiplier_raw as f32 / 1_000.0,
        modifiers.incoming_damage_multiplier_raw as f32 / 1_000.0,
        modifiers.gold_gain_multiplier_raw as f32 / 1_000.0,
        modifiers.enemy_health_multiplier_raw as f32 / 1_000.0,
        modifiers.enemy_speed_multiplier_raw as f32 / 1_000.0,
        modifiers.max_hand_slots_delta as f32 / 10.0,
        modifiers.max_rerolls_delta as f32 / 10.0,
        modifiers.reroll_health_cost as f32 / 1_000.0,
        modifiers.item_use_disabled as u8 as f32,
        modifiers.purchases_disabled as u8 as f32,
        modifiers.free_shop as u8 as f32,
    ]);
    let route_length = observation.route_coords.len();
    let route_occupancy = observation
        .route_coords
        .iter()
        .filter(|coord| {
            let index = coord.y * observation.map_width + coord.x;
            observation
                .tower_grid
                .get(index)
                .is_some_and(Option::is_some)
        })
        .count();
    features.extend([
        route_length as f32 / 1_000.0,
        route_occupancy as f32 / route_length.max(1) as f32,
        observation.route_coords.first().map_or(0.0, |coord| {
            coord.x as f32 / observation.map_width.max(1) as f32
        }),
        observation.route_coords.last().map_or(0.0, |coord| {
            coord.y as f32 / observation.map_height.max(1) as f32
        }),
    ]);

    debug_assert_eq!(features.len(), GLOBAL_FEATURE_COUNT);
    features
}

pub fn candidate_features(observation: &Observation, action: &AgentAction) -> Vec<f32> {
    let mut features = vec![0.0; ACTION_FEATURE_COUNT];
    let kind = action.kind().index();
    features[kind] = 1.0;

    let mut params = [0.0; 11];
    match action {
        AgentAction::PurchaseShopItem { slot_index } => {
            if let Some(slot) = observation
                .shop
                .iter()
                .find(|slot| slot.index == *slot_index)
            {
                params = [
                    shop_kind_id(&slot.kind) as f32 / 32.0,
                    slot.key_id as f32 / 32.0,
                    slot.cost as f32 / 1_000.0,
                    slot.purchased as u8 as f32,
                    *slot_index as f32 / 10.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ];
            }
        }
        AgentAction::SelectTreasure { option_index } => {
            params[0] = *option_index as f32 / 10.0;
            params[1] = observation
                .treasure_options
                .get(*option_index)
                .map_or(0.0, |option| upgrade_key_id(option) as f32 / 32.0);
        }
        AgentAction::SelectCardServiceCard { card_index } => {
            if let Some(card) = observation.deck.all_cards.get(*card_index) {
                params[1] = suit_id(&card.suit) as f32 / 4.0;
                params[2] = rank_id(&card.rank) as f32 / 13.0;
                params[3] = card.polish_pct_raw as f32 / 1_000.0;
                params[4] = card
                    .engraving
                    .as_deref()
                    .map_or(0.0, |value| engraving_key_id(value) as f32 / 32.0);
                params[5] = observation
                    .card_service
                    .as_ref()
                    .is_some_and(|service| service.selected_card_indices.contains(card_index))
                    as u8 as f32;
            }
        }
        AgentAction::UseInventoryItem { item_index } => {
            params[0] = *item_index as f32 / 20.0;
            if let Some(item) = observation.inventory.get(*item_index) {
                params[1] = item.key_id as f32 / 32.0;
            }
        }
        AgentAction::Reroll {
            selected_slot_indices,
        }
        | AgentAction::SelectTower {
            selected_slot_indices,
        } => {
            params[0] = selected_slot_indices.len() as f32 / 10.0;
            let cards = selected_slot_indices.iter().filter_map(|index| {
                observation
                    .hand
                    .get(*index)
                    .and_then(|item| match &item.item {
                        crate::environment::HandItemObservation::Card(card) => Some(card),
                        _ => None,
                    })
            });
            params[1] = mean(cards.clone().map(|card| suit_id(&card.suit) as f32 / 4.0));
            params[2] = mean(cards.clone().map(|card| rank_id(&card.rank) as f32 / 13.0));
            params[3] = mean(cards.map(|card| card.polish_pct_raw as f32 / 1_000.0));
            params[4] = selected_slot_indices.iter().sum::<usize>() as f32 / 100.0;
        }
        AgentAction::PlaceTower {
            hand_slot_index,
            left,
            top,
        } => {
            params[0] = *hand_slot_index as f32 / 10.0;
            params[1] = *left as f32 / 32.0;
            params[2] = *top as f32 / 32.0;
            if let Some(item) = observation.hand.get(*hand_slot_index)
                && let crate::environment::HandItemObservation::Tower(tower) = &item.item
            {
                params[3] = tower.kind_id as f32 / 32.0;
                params[4] = tower.damage_raw as f32 / 10_000.0;
                params[5] = tower.rerolled_count as f32 / 20.0;
                params[6] = route_distance(observation, *left, *top) as f32 / 50.0;
                params[7] = nearby_tower_occupancy(observation, *left, *top);
                params[8] = route_progress_at(observation, *left, *top);
                params[9] = adjacent_tower_count(observation, *left, *top) as f32 / 8.0;
                params[10] = placement_coverage(observation, *left, *top, &tower.kind);
            }
        }
        AgentAction::RemoveTower { tower_id } => {
            if let Some(tower) = observation
                .towers
                .iter()
                .find(|tower| tower.id == *tower_id)
            {
                params[0] = tower.template.kind_id as f32 / 32.0;
                params[1] = tower.left as f32 / observation.map_width.max(1) as f32;
                params[2] = tower.top as f32 / observation.map_height.max(1) as f32;
                params[3] = tower.template.damage_raw as f32 / 10_000.0;
                params[4] = tower.template.rerolled_count as f32 / 20.0;
            }
        }
        AgentAction::BeginRerollSelection
        | AgentAction::BeginTowerSelection
        | AgentAction::ConfirmCardSelection
        | AgentAction::CancelCardSelection
        | AgentAction::StartSelectingTower
        | AgentAction::StartDefense
        | AgentAction::ConfirmCardServiceSelection
        | AgentAction::Continue => {}
        AgentAction::SelectHandCard { hand_slot_index }
        | AgentAction::DeselectHandCard { hand_slot_index } => {
            if let Some(item) = observation
                .hand
                .iter()
                .find(|item| item.index == *hand_slot_index)
                && let crate::environment::HandItemObservation::Card(card) = &item.item
            {
                params[0] = suit_id(&card.suit) as f32 / 4.0;
                params[1] = rank_id(&card.rank) as f32 / 13.0;
                params[2] = card.polish_pct_raw as f32 / 1_000.0;
                params[3] = card
                    .engraving
                    .as_deref()
                    .map_or(0.0, |value| engraving_key_id(value) as f32 / 32.0);
                params[4] = item.selected as u8 as f32;
                params[5] = matches!(action, AgentAction::SelectHandCard { .. }) as u8 as f32;
                params[6] = *hand_slot_index as f32 / observation.hand.len().max(1) as f32;
            }
        }
    }
    features[ActionKind::COUNT..].copy_from_slice(&params);
    features
}

fn route_distance(observation: &Observation, left: usize, top: usize) -> usize {
    observation
        .route_coords
        .iter()
        .map(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
        .min()
        .unwrap_or(observation.map_width + observation.map_height)
}

pub(crate) fn placement_coverage(
    observation: &Observation,
    left: usize,
    top: usize,
    tower_kind: &str,
) -> f32 {
    let range_raw = tower_range_raw(tower_kind);
    let covered_route = observation
        .route_coords
        .iter()
        .filter(|coord| {
            let dx = (coord.x as i64 - left as i64)
                .saturating_mul(1_000_000)
                .saturating_sub(500_000);
            let dy = (coord.y as i64 - top as i64)
                .saturating_mul(1_000_000)
                .saturating_sub(500_000);
            dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
                <= range_raw.saturating_mul(range_raw)
        })
        .count();
    covered_route as f32 / observation.route_coords.len().max(1) as f32
}

fn tower_range_raw(kind: &str) -> i64 {
    match kind {
        "rubber_cone" | "high" => 4_000_000,
        "one_pair" => 5_000_000,
        "two_pair" => 6_000_000,
        "three_of_a_kind" => 7_000_000,
        "straight" | "flush" => 9_000_000,
        "full_house" | "four_of_a_kind" => 11_000_000,
        "straight_flush" => 14_000_000,
        "royal_flush" => 15_000_000,
        _ => 4_000_000,
    }
}

fn route_progress_at(observation: &Observation, left: usize, top: usize) -> f32 {
    observation
        .route_coords
        .iter()
        .min_by_key(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
        .map_or(0.0, |coord| {
            coord.index as f32 / observation.route_coords.len().max(1) as f32
        })
}

fn adjacent_tower_count(observation: &Observation, left: usize, top: usize) -> usize {
    let mut count = 0;
    for row in top.saturating_sub(1)..=(top + 2).min(observation.map_height.saturating_sub(1)) {
        for column in
            left.saturating_sub(1)..=(left + 2).min(observation.map_width.saturating_sub(1))
        {
            let index = row * observation.map_width + column;
            if observation
                .tower_grid
                .get(index)
                .is_some_and(Option::is_some)
            {
                count += 1;
            }
        }
    }
    count
}

fn nearby_tower_occupancy(observation: &Observation, left: usize, top: usize) -> f32 {
    adjacent_tower_count(observation, left, top) as f32 / 16.0
}

pub fn action_features(action: &AgentAction) -> Vec<f32> {
    candidate_features(
        &Observation {
            observation_schema_version: 0,
            catalog_schema_version: 0,
            action_wire_schema_version: 0,
            environment_version: 0,
            action_schema_version: 0,
            decision_point: DecisionPoint::Terminal,
            item_window_stage: None,
            damage_trigger_tick: None,
            card_selection_purpose: None,
            selected_hand_slot_indices: vec![],
            card_selection_confirmable: false,
            stage: 0,
            sim_tick: 0,
            theme: None,
            hp_raw: 0,
            max_hp_raw: 1,
            shield_raw: 0,
            gold: 0,
            left_dice: 0,
            rerolled_count: 0,
            stage_progress_raw: 0,
            stage_total_hp_raw: 1,
            active_monster_count: 0,
            queued_monster_count: 0,
            hand: vec![],
            deck: crate::environment::DeckObservation {
                all_cards: vec![],
                draw_cards: vec![],
                discard_cards: vec![],
            },
            shop: vec![],
            inventory: vec![],
            owned_upgrades: vec![],
            towers: vec![],
            tower_grid: vec![],
            map_width: 1,
            map_height: 1,
            route_coords: vec![],
            monsters: vec![],
            treasure_options: vec![],
            card_service: None,
            stage_modifiers: crate::environment::StageModifiersObservation {
                damage_multiplier_raw: 1000,
                damage_reduction_multiplier_raw: 1000,
                incoming_damage_multiplier_raw: 1000,
                gold_gain_multiplier_raw: 1000,
                enemy_health_multiplier_raw: 1000,
                enemy_speed_multiplier_raw: 1000,
                max_hand_slots_delta: 0,
                max_rerolls_delta: 0,
                reroll_health_cost: 0,
                item_use_disabled: false,
                purchases_disabled: false,
                free_shop: false,
            },
        },
        action,
    )
}

fn mean(values: impl Iterator<Item = f32>) -> f32 {
    let (sum, count) = values.fold((0.0, 0_usize), |(sum, count), value| {
        (sum + value, count + 1)
    });
    if count == 0 { 0.0 } else { sum / count as f32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::environment::GameEnvironment;
    use std::sync::Arc;

    #[test]
    fn feature_dimensions_are_content_independent() {
        let config = Arc::new(GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let observation = environment.snapshot();
        assert_eq!(
            observation_features(&observation).len(),
            GLOBAL_FEATURE_COUNT
        );
        assert_eq!(
            action_features(&AgentAction::Continue).len(),
            ACTION_FEATURE_COUNT
        );
    }

    #[test]
    fn candidate_content_changes_action_features() {
        let config = Arc::new(GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let observation = environment.snapshot();
        let first = candidate_features(&observation, &AgentAction::Continue);
        let second = candidate_features(&observation, &AgentAction::StartDefense);
        assert_ne!(first, second);
    }

    #[test]
    fn card_distribution_features_are_present() {
        let config = Arc::new(GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let features = observation_features(&environment.snapshot());
        assert_eq!(features.len(), GLOBAL_FEATURE_COUNT);
        assert!(features.iter().any(|feature| *feature > 0.0));
    }

    #[test]
    fn deck_distribution_features_are_present() {
        let config = Arc::new(GameConfig::default_config());
        let environment = GameEnvironment::new(config, 7);
        let features = observation_features(&environment.snapshot());
        assert_eq!(features.len(), GLOBAL_FEATURE_COUNT);
        assert!(features.iter().any(|feature| *feature > 0.0));
    }
}

use crate::{CardState, CoreState, HandItemState, TowerTemplateState, UpgradeCollection};
use std::collections::BTreeMap;

const CARD_COUNT: usize = 5;

struct StraightResult {
    royal: bool,
    top: CardState,
    cards: Vec<CardState>,
}

struct FlushResult {
    suit: u8,
}

pub fn get_highest_tower_template(
    cards: &[CardState],
    upgrades: &UpgradeCollection,
    config: &crate::GameConfig,
    rerolled_count: usize,
) -> Option<TowerTemplateState> {
    let straight_result = check_straight(cards, upgrades);
    let flush_result = check_flush(cards, upgrades);

    let straight_flush_result = flush_groups(cards, upgrades)
        .into_iter()
        .filter_map(|(suit, flush_cards)| {
            check_straight(&flush_cards, upgrades).map(|straight| (suit, straight))
        })
        .max_by_key(|(_, straight)| straight.top.rank);

    if let Some((suit, straight_result)) = straight_flush_result {
        return Some(build_template(
            if straight_result.royal { 10 } else { 9 },
            Some(suit),
            Some(if straight_result.royal {
                12
            } else {
                straight_result.top.rank.raw()
            }),
            straight_result.cards,
            rerolled_count,
            config,
        ));
    }

    let rank_map = count_rank(cards);
    let mut triple_cards = None;
    let mut pair_high_cards = None;
    let mut pair_low_cards = None;

    for rank in (0..13).rev() {
        let Some(cards_of_rank) = rank_map.get(&rank) else {
            continue;
        };
        if cards_of_rank.len() >= 4 {
            let top_card = cards_of_rank
                .iter()
                .max_by_key(|card| card_order_key(card))
                .unwrap();
            return Some(build_template(
                8,
                Some(top_card.suit.raw()),
                Some(top_card.rank.raw()),
                cards_of_rank.clone(),
                rerolled_count,
                config,
            ));
        }

        if cards_of_rank.len() >= 3 && triple_cards.is_none() {
            triple_cards = Some(cards_of_rank.clone());
        } else if cards_of_rank.len() >= 2 {
            let mut pair_cards = cards_of_rank.clone();
            pair_cards.sort_by_key(card_order_key);
            let pair_cards = pair_cards.into_iter().rev().take(2).collect::<Vec<_>>();
            if pair_high_cards.is_none() {
                pair_high_cards = Some(pair_cards);
            } else if pair_low_cards.is_none() {
                pair_low_cards = Some(pair_cards);
            }
        }
    }

    if let (Some(triple_cards_vec), Some(pair_high_cards_vec)) = (&triple_cards, &pair_high_cards) {
        let mut combined_cards = triple_cards_vec
            .iter()
            .chain(pair_high_cards_vec)
            .cloned()
            .collect::<Vec<_>>();
        combined_cards.sort_by_key(card_order_key);
        let top_card = combined_cards.last().unwrap();
        return Some(build_template(
            7,
            Some(top_card.suit.raw()),
            Some(top_card.rank.raw()),
            combined_cards,
            rerolled_count,
            config,
        ));
    }

    if let Some(flush_result) = flush_result {
        let flush_cards = flush_groups(cards, upgrades)
            .into_iter()
            .find(|(suit, _)| *suit == flush_result.suit)
            .map(|(_, cards)| cards)
            .unwrap_or_default();
        let top_card = flush_cards
            .iter()
            .max_by_key(|card| card_order_key(card))
            .unwrap();
        return Some(build_template(
            6,
            Some(flush_result.suit),
            Some(top_card.rank.raw()),
            flush_cards,
            rerolled_count,
            config,
        ));
    }

    if let Some(straight_result) = straight_result {
        return Some(build_template(
            5,
            Some(straight_result.top.suit.raw()),
            Some(straight_result.top.rank.raw()),
            straight_result.cards,
            rerolled_count,
            config,
        ));
    }

    if let Some(mut triple_cards_vec) = triple_cards {
        triple_cards_vec.sort_by_key(card_order_key);
        let top_card = triple_cards_vec.last().unwrap();
        return Some(build_template(
            4,
            Some(top_card.suit.raw()),
            Some(top_card.rank.raw()),
            triple_cards_vec,
            rerolled_count,
            config,
        ));
    }

    if let (Some(pair_high_cards_vec), Some(pair_low_cards_vec)) =
        (&pair_high_cards, &pair_low_cards)
    {
        let mut combined_cards = pair_high_cards_vec
            .iter()
            .chain(pair_low_cards_vec)
            .cloned()
            .collect::<Vec<_>>();
        combined_cards.sort_by_key(card_order_key);
        let top_card = combined_cards.last().unwrap();
        return Some(build_template(
            3,
            Some(top_card.suit.raw()),
            Some(top_card.rank.raw()),
            combined_cards,
            rerolled_count,
            config,
        ));
    }

    if let Some(mut pair_high_cards_vec) = pair_high_cards {
        pair_high_cards_vec.sort_by_key(card_order_key);
        let top_card = pair_high_cards_vec.last().unwrap();
        return Some(build_template(
            2,
            Some(top_card.suit.raw()),
            Some(top_card.rank.raw()),
            pair_high_cards_vec,
            rerolled_count,
            config,
        ));
    }

    let top_card = cards.iter().max_by_key(|card| card_order_key(card))?;
    Some(build_template(
        1,
        Some(top_card.suit.raw()),
        Some(top_card.rank.raw()),
        vec![top_card.clone()],
        rerolled_count,
        config,
    ))
}

pub(crate) fn select_tower_from_core(
    state: &mut CoreState,
    selected_slot_indices: &[usize],
) -> Result<(), crate::CommandError> {
    if !matches!(state.flow, crate::GameFlowState::SelectingTower) {
        return Err(crate::CommandError::InvalidFlow);
    }
    let selected_template = tower_template_for_selection(state, selected_slot_indices)?;
    start_placing_tower_from_template(state, selected_template);
    Ok(())
}

/// Resolves a `SelectTower` slot selection to the tower template the command
/// would build, without mutating state. Shared by `select_tower_from_core`
/// and the observation's `build_tower_candidates` preview so both follow the
/// same rules.
pub(crate) fn tower_template_for_selection(
    state: &CoreState,
    selected_slot_indices: &[usize],
) -> Result<TowerTemplateState, crate::CommandError> {
    let indices: Vec<usize> = if selected_slot_indices.is_empty() {
        (0..state.hand.slots.len()).collect()
    } else {
        selected_slot_indices.to_vec()
    };
    let mut cards = Vec::new();
    for index in indices {
        let slot = state
            .hand
            .slots
            .get(index)
            .ok_or(crate::CommandError::InvalidIndex)?;
        let HandItemState::Card(card) = &slot.item else {
            return Err(crate::CommandError::InvalidSelection);
        };
        cards.push(card.clone());
    }
    get_highest_tower_template(
        &cards,
        state.upgrades(),
        &state.config,
        state.progress.rerolled_count,
    )
    .ok_or(crate::CommandError::InvalidSelection)
}

pub(crate) fn start_placing_tower_from_template(
    state: &mut CoreState,
    initial_template: crate::TowerTemplateState,
) {
    let mut templates = vec![initial_template];
    let extra_tower_cards = std::mem::take(&mut state.stage_modifiers.extra_tower_cards);
    templates.extend(extra_tower_cards.iter().map(|extra| {
        build_template(
            extra.kind,
            extra.suit,
            extra.rank,
            Vec::new(),
            state.progress.rerolled_count,
            &state.config,
        )
    }));
    state.hand.slots.clear();
    for (index, template) in templates.into_iter().enumerate() {
        let id = state.hand.allocate_slot_id();
        state.hand.slots.push(crate::HandSlotState {
            id,
            item: crate::HandItemState::Tower(template),
            selected: index == 0,
        });
    }
    state.flow = crate::GameFlowState::PlacingTower;
}

pub(crate) fn build_template(
    kind: u8,
    suit: Option<u8>,
    rank: Option<u8>,
    used_cards: Vec<CardState>,
    rerolled_count: usize,
    config: &crate::GameConfig,
) -> TowerTemplateState {
    let stats = config
        .towers
        .entries
        .iter()
        .find(|entry| entry.kind == kind);
    let overcharge_count = used_cards
        .iter()
        .filter(|card| card.engraving == Some(1))
        .count();
    let base_interval = stats
        .map(|entry| entry.cooldown_ms.saturating_mul(60).div_ceil(1000))
        .unwrap_or(60);
    let shoot_interval = (0..overcharge_count).fold(base_interval, |interval, _| {
        interval.saturating_mul(666_667).saturating_add(999_999) / 1_000_000
    });
    TowerTemplateState {
        kind,
        rerolled_count,
        shoot_interval,
        default_attack_range_radius_raw: stats.map(|entry| entry.range_raw).unwrap_or(0),
        default_damage_raw: stats.map(|entry| entry.damage_raw).unwrap_or(0),
        suit,
        rank,
        skill_templates: Vec::new(),
        default_status_effects: Vec::new(),
        used_cards,
    }
}

fn flush_groups(cards: &[CardState], upgrades: &UpgradeCollection) -> Vec<(u8, Vec<CardState>)> {
    let flush_card_count = if upgrades.shorten_straight_flush_to_4_cards() {
        4
    } else {
        CARD_COUNT
    };
    let treat_suits_as_same = upgrades.treat_suits_as_same();

    if cards.len() < flush_card_count {
        return Vec::new();
    }

    let mut suit_map = BTreeMap::<u8, Vec<CardState>>::new();
    for card in cards {
        let suit = normalized_suit(card.suit.raw(), treat_suits_as_same);
        suit_map.entry(suit).or_default().push(card.clone());
    }

    suit_map
        .into_iter()
        .filter(|(_, cards)| cards.len() >= flush_card_count)
        .collect()
}

fn check_straight(cards: &[CardState], upgrades: &UpgradeCollection) -> Option<StraightResult> {
    let straight_card_count = if upgrades.shorten_straight_flush_to_4_cards() {
        4
    } else {
        CARD_COUNT
    };
    let skip_rank_for_straight = upgrades.skip_rank_for_straight();

    if cards.len() < straight_card_count {
        return None;
    }

    let mut best = None;
    for ace_high in [false, true] {
        let mut cards_by_value = BTreeMap::<usize, Vec<CardState>>::new();
        for card in cards {
            let value = if ace_high {
                if card.rank == crate::Rank::Ace {
                    13
                } else {
                    card.rank.ordinal() + 1
                }
            } else if card.rank == crate::Rank::Ace {
                0
            } else {
                card.rank.ordinal() + 1
            };
            cards_by_value.entry(value).or_default().push(card.clone());
        }

        let values = cards_by_value.keys().copied().collect::<Vec<_>>();
        for window in values.windows(straight_card_count) {
            let missing_count = window
                .last()
                .unwrap()
                .saturating_sub(*window.first().unwrap())
                .saturating_sub(straight_card_count - 1);
            if missing_count > usize::from(skip_rank_for_straight) {
                continue;
            }

            let selected_cards = window
                .iter()
                .map(|value| {
                    cards_by_value
                        .get(value)
                        .unwrap()
                        .iter()
                        .max_by_key(|card| card_order_key(card))
                        .unwrap()
                        .clone()
                })
                .collect::<Vec<_>>();
            let candidate = StraightResult {
                royal: is_royal(window, straight_card_count),
                top: selected_cards.last().unwrap().clone(),
                cards: selected_cards,
            };
            if best
                .as_ref()
                .is_none_or(|(_, best_value)| *best_value < *window.last().unwrap())
            {
                best = Some((candidate, *window.last().unwrap()));
            }
        }
    }

    best.map(|(result, _)| result)
}

fn is_royal(ranks: &[usize], straight_card_count: usize) -> bool {
    let royal_ranks = [9, 10, 11, 12, 13];
    if straight_card_count == 5 {
        return ranks.iter().all(|rank| royal_ranks.contains(rank));
    }
    straight_card_count == 4 && ranks.iter().all(|rank| royal_ranks.contains(rank))
}

fn check_flush(cards: &[CardState], upgrades: &UpgradeCollection) -> Option<FlushResult> {
    flush_groups(cards, upgrades)
        .into_iter()
        .max_by_key(|(_, cards)| cards.iter().map(|card| card.rank.ordinal()).max())
        .map(|(suit, _)| FlushResult { suit })
}

fn count_rank(cards: &[CardState]) -> BTreeMap<u8, Vec<CardState>> {
    let mut map = BTreeMap::new();
    for card in cards {
        map.entry(card.rank.raw())
            .or_insert_with(Vec::new)
            .push(card.clone());
    }
    map
}

fn normalized_suit(suit: u8, same: bool) -> u8 {
    if !same {
        suit
    } else if suit == 0 || suit == 3 {
        0
    } else {
        1
    }
}

fn card_order_key(card: &CardState) -> (u8, u8, usize) {
    (card.rank.raw(), card.suit.raw(), card.id)
}

#[cfg(test)]
mod tests;

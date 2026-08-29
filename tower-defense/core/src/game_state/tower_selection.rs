use crate::{CardState, CoreState, GameConfigState, HandItemState, TowerTemplateState};

const CARD_COUNT: usize = 5;

pub(crate) fn select_tower_build_template(
    cards: &[CardState],
    upgrades: &crate::UpgradeCollectionState,
    config: &GameConfigState,
    rerolled_count: usize,
) -> Option<TowerTemplateState> {
    if cards.is_empty() {
        return None;
    }
    let straight_count = if upgrades.shorten_straight_flush_to_4_cards() {
        4
    } else {
        CARD_COUNT
    };
    let treat_suits_as_same = upgrades.treat_suits_as_same();
    let skip_rank_for_straight = upgrades.skip_rank_for_straight();
    let mut best_straight: Option<(u8, bool, Option<u8>, Vec<CardState>)> = None;
    for suit in distinct_suits(cards, treat_suits_as_same) {
        let suited: Vec<CardState> = cards
            .iter()
            .filter(|card| normalized_suit(card.suit, treat_suits_as_same) == suit)
            .cloned()
            .collect();
        if suited.len() < straight_count {
            continue;
        }
        if let Some((top, royal, selected)) =
            best_straight_for_cards(&suited, straight_count, skip_rank_for_straight)
            && best_straight
                .as_ref()
                .is_none_or(|(best_top, _, _, _)| top > *best_top)
        {
            best_straight = Some((top, royal, Some(suit), selected));
        }
    }
    if let Some((top, royal, suit, used_cards)) = best_straight {
        return Some(build_template(
            if royal { 10 } else { 9 },
            suit,
            Some(top),
            used_cards,
            rerolled_count,
            config,
        ));
    }

    let by_rank = (0..13)
        .map(|rank| {
            cards
                .iter()
                .filter(|card| card.rank == rank)
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let four = (0..13).rev().find(|rank| by_rank[*rank].len() >= 4);
    if let Some(rank) = four {
        return Some(build_template(
            8,
            Some(by_rank[rank][0].suit),
            Some(rank as u8),
            by_rank[rank].clone(),
            rerolled_count,
            config,
        ));
    }
    let triple = (0..13).rev().find(|rank| by_rank[*rank].len() >= 3);
    let pairs: Vec<usize> = (0..13)
        .rev()
        .filter(|rank| by_rank[*rank].len() >= 2)
        .collect();
    if let Some(triple_rank) = triple
        && let Some(pair) = pairs.iter().copied().find(|rank| *rank != triple_rank)
    {
        let mut used = by_rank[triple_rank].clone();
        used.extend(by_rank[pair].clone());
        return Some(build_template(
            7,
            used[0].suit.into(),
            Some(triple_rank as u8),
            used,
            rerolled_count,
            config,
        ));
    }
    if let Some(suit) = distinct_suits(cards, treat_suits_as_same)
        .into_iter()
        .find(|suit| {
            cards
                .iter()
                .filter(|card| normalized_suit(card.suit, treat_suits_as_same) == *suit)
                .count()
                >= straight_count
        })
    {
        let used = cards
            .iter()
            .filter(|card| normalized_suit(card.suit, treat_suits_as_same) == suit)
            .cloned()
            .collect::<Vec<_>>();
        let top = used.iter().map(|card| card.rank).max().unwrap_or(0);
        return Some(build_template(
            6,
            Some(suit),
            Some(top),
            used,
            rerolled_count,
            config,
        ));
    }
    if let Some((top, _, used)) =
        best_straight_for_cards(cards, straight_count, skip_rank_for_straight)
    {
        return Some(build_template(
            5,
            None,
            Some(top),
            used,
            rerolled_count,
            config,
        ));
    }
    if let Some(triple) = triple {
        return Some(build_template(
            4,
            Some(by_rank[triple][0].suit),
            Some(triple as u8),
            by_rank[triple].clone(),
            rerolled_count,
            config,
        ));
    }
    if pairs.len() >= 2 {
        let mut used = by_rank[pairs[0]].clone();
        used.extend(by_rank[pairs[1]].clone());
        return Some(build_template(
            3,
            Some(used[0].suit),
            Some(pairs[0] as u8),
            used,
            rerolled_count,
            config,
        ));
    }
    if let Some(pair) = pairs.first().copied() {
        return Some(build_template(
            2,
            Some(by_rank[pair][0].suit),
            Some(pair as u8),
            by_rank[pair].clone(),
            rerolled_count,
            config,
        ));
    }
    let card = cards.iter().max_by_key(|card| card.rank)?;
    Some(build_template(
        1,
        Some(card.suit),
        Some(card.rank),
        vec![card.clone()],
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
    let selected_template = select_tower_build_template(
        &cards,
        &state.upgrades,
        &state.config,
        state.progress.rerolled_count,
    )
    .ok_or(crate::CommandError::InvalidSelection)?;
    start_placing_tower_from_template(state, selected_template);
    Ok(())
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

fn build_template(
    kind: u8,
    suit: Option<u8>,
    rank: Option<u8>,
    used_cards: Vec<CardState>,
    rerolled_count: usize,
    config: &GameConfigState,
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

fn normalized_suit(suit: u8, same: bool) -> u8 {
    if !same {
        suit
    } else if suit == 0 || suit == 3 {
        0
    } else {
        1
    }
}

fn distinct_suits(cards: &[CardState], same: bool) -> Vec<u8> {
    let mut suits = Vec::new();
    for card in cards {
        let suit = normalized_suit(card.suit, same);
        if !suits.contains(&suit) {
            suits.push(suit);
        }
    }
    suits
}

fn best_straight_for_cards(
    cards: &[CardState],
    count: usize,
    skip: bool,
) -> Option<(u8, bool, Vec<CardState>)> {
    let mut best = None;
    for ace_high in [false, true] {
        let mut values = cards
            .iter()
            .map(|card| {
                if card.rank == 12 {
                    if ace_high { 13 } else { 0 }
                } else {
                    card.rank + 1
                }
            })
            .collect::<Vec<_>>();
        values.sort_unstable();
        values.dedup();
        for window in values.windows(count) {
            let start = *window.first()?;
            let end = *window.last()?;
            let missing = (end as usize)
                .saturating_sub(start as usize)
                .saturating_add(1)
                .saturating_sub(window.len());
            if missing > usize::from(skip) {
                continue;
            }
            let mut used = Vec::new();
            for value in window {
                if let Some(card) = cards
                    .iter()
                    .filter(|card| {
                        let card_value = if card.rank == 12 {
                            if ace_high { 13 } else { 0 }
                        } else {
                            card.rank + 1
                        };
                        card_value == *value
                    })
                    .max_by_key(|card| (card.polish_pct_raw, card.id))
                {
                    used.push(card.clone());
                }
            }
            if used.len() + missing != count {
                continue;
            }
            let top_rank = used
                .iter()
                .max_by_key(|card| {
                    if card.rank == 12 && !ace_high {
                        0
                    } else {
                        card.rank + 1
                    }
                })?
                .rank;
            let royal = ace_high && start == 10 && end == 13;
            if best
                .as_ref()
                .is_none_or(|(top, _, _): &(u8, bool, Vec<CardState>)| top_rank > *top)
            {
                best = Some((top_rank, royal, used));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> GameConfigState {
        GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 30,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: (0..11)
                    .map(|kind| crate::TowerConfigEntryState {
                        kind,
                        damage_raw: i64::from(kind),
                        range_raw: i64::from(kind),
                        cooldown_ms: 1_000,
                    })
                    .collect(),
            },
            monsters: crate::MonsterConfigState {
                stats: Vec::new(),
                stage_waves: Vec::new(),
            },
        }
    }

    fn card(id: usize, suit: u8, rank: u8) -> CardState {
        CardState {
            id,
            suit,
            rank,
            polish_pct_raw: 0,
            engraving: None,
        }
    }

    #[test]
    fn raw_selection_matches_basic_poker_tower_kinds() {
        let upgrades = crate::UpgradeCollectionState {
            upgrades: Vec::new(),
            revision: 0,
        };
        let cards = vec![
            card(1, 0, 12),
            card(2, 1, 12),
            card(3, 2, 5),
            card(4, 3, 7),
            card(5, 0, 6),
        ];
        let template = select_tower_build_template(&cards, &upgrades, &config(), 0)
            .expect("template should be generated");
        assert_eq!(template.kind, 2);
        assert_eq!(template.rank, Some(12));
    }

    #[test]
    fn raw_selection_supports_four_card_straight_and_royal_flush_upgrades() {
        let upgrades = crate::UpgradeCollectionState {
            upgrades: vec![crate::UpgradeEntryState {
                id: 1,
                kind: 8,
                scalar_values: Vec::new(),
                ratio_values_raw: Vec::new(),
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            }],
            revision: 0,
        };
        let cards = vec![
            card(1, 1, 9),
            card(2, 1, 10),
            card(3, 1, 11),
            card(4, 1, 12),
        ];
        let template = select_tower_build_template(&cards, &upgrades, &config(), 0)
            .expect("template should be generated");
        assert_eq!(template.kind, 10);
        assert_eq!(template.rank, Some(12));
    }

    #[test]
    fn raw_selection_matches_root_skip_rank_selection_semantics() {
        let upgrades = crate::UpgradeCollectionState {
            upgrades: vec![crate::UpgradeEntryState {
                id: 1,
                kind: 9,
                scalar_values: Vec::new(),
                ratio_values_raw: Vec::new(),
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            }],
            revision: 0,
        };
        let cards = vec![card(1, 0, 5), card(2, 1, 6), card(3, 2, 7), card(4, 3, 9)];
        let template = select_tower_build_template(&cards, &upgrades, &config(), 0)
            .expect("template should be generated");
        assert_eq!(template.kind, 1);
        assert_eq!(template.rank, Some(9));
    }

    #[test]
    fn raw_selection_supports_ace_low_straight() {
        let upgrades = crate::UpgradeCollectionState {
            upgrades: Vec::new(),
            revision: 0,
        };
        let cards = vec![
            card(1, 0, 12),
            card(2, 1, 0),
            card(3, 2, 1),
            card(4, 3, 2),
            card(5, 0, 3),
        ];
        let template = select_tower_build_template(&cards, &upgrades, &config(), 0)
            .expect("template should be generated");
        assert_eq!(template.kind, 5);
        assert_eq!(template.rank, Some(3));
    }

    #[test]
    fn raw_template_derives_overcharge_interval_and_preserves_card_payload() {
        let upgrades = crate::UpgradeCollectionState {
            upgrades: Vec::new(),
            revision: 0,
        };
        let mut overcharge = card(1, 0, 12);
        overcharge.engraving = Some(1);
        let cards = vec![overcharge];
        let template = select_tower_build_template(&cards, &upgrades, &config(), 0)
            .expect("template should be generated");
        assert_eq!(template.shoot_interval, 41);
        assert_eq!(template.used_cards[0].engraving, Some(1));
    }
}

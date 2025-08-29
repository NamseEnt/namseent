#[cfg(test)]
mod tests;
mod tower_hand_ranking;
mod tower_skill_injector;
mod tower_status_effect_injector;
mod tower_template_factory;

use self::tower_hand_ranking::{check_flush, check_straight, count_rank};
use self::tower_skill_injector::inject_skills;
use self::tower_status_effect_injector::inject_status_effects;
use self::tower_template_factory::create_tower_template;
use crate::card::Card;
use crate::card::REVERSED_RANKS;
use crate::game_state::tower::TowerKind;
use crate::game_state::tower::TowerTemplate;
use crate::game_state::upgrade::UpgradeState;

pub fn get_highest_tower_template(
    cards: &[Card],
    upgrade_state: &UpgradeState,
    rerolled_count: usize,
) -> TowerTemplate {
    let straight_result = check_straight(cards, upgrade_state);
    let flush_result = check_flush(cards, upgrade_state);

    if let (Some(straight_result), Some(flush_result)) = (&straight_result, &flush_result) {
        if straight_result.royal && straight_result.top.rank == crate::card::Rank::Ace {
            let mut template = create_tower_template(
                TowerKind::RoyalFlush,
                flush_result.suit,
                crate::card::Rank::Ace,
            );
            inject_skills(&mut template);
            inject_status_effects(&mut template, upgrade_state, rerolled_count);
            return template;
        }
        let mut template = create_tower_template(
            TowerKind::StraightFlush,
            flush_result.suit,
            straight_result.top.rank,
        );
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    let rank_map = count_rank(cards);
    let mut triple_cards = None;
    let mut pair_high_cards = None;
    let mut pair_low_cards = None;

    for rank in REVERSED_RANKS {
        let Some(cards_of_rank) = rank_map.get(&rank) else {
            continue;
        };
        if cards_of_rank.len() == 4 {
            let mut sorted_cards = cards_of_rank.clone();
            sorted_cards.sort();
            let top_card = sorted_cards.last().unwrap();
            let mut template =
                create_tower_template(TowerKind::FourOfAKind, top_card.suit, top_card.rank);
            inject_skills(&mut template);
            inject_status_effects(&mut template, upgrade_state, rerolled_count);
            return template;
        }

        if cards_of_rank.len() == 3 && triple_cards.is_none() {
            triple_cards = Some(cards_of_rank.clone());
        } else if cards_of_rank.len() == 2 {
            if pair_high_cards.is_none() {
                pair_high_cards = Some(cards_of_rank.clone());
            } else if pair_low_cards.is_none() {
                pair_low_cards = Some(cards_of_rank.clone());
            }
        }
    }

    if let (Some(triple_cards_vec), Some(pair_high_cards_vec)) = (&triple_cards, &pair_high_cards) {
        let mut combined_cards = triple_cards_vec
            .iter()
            .chain(pair_high_cards_vec)
            .collect::<Vec<_>>();
        combined_cards.sort();
        let top_card = combined_cards.last().unwrap();
        let mut template =
            create_tower_template(TowerKind::FullHouse, top_card.suit, top_card.rank);
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    if let Some(flush_result) = flush_result {
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort();
        let top_card = sorted_cards.last().unwrap();
        let mut template =
            create_tower_template(TowerKind::Flush, flush_result.suit, top_card.rank);
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    if let Some(straight_result) = straight_result {
        let mut template = create_tower_template(
            TowerKind::Straight,
            straight_result.top.suit,
            straight_result.top.rank,
        );
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    if let Some(mut triple_cards_vec) = triple_cards {
        triple_cards_vec.sort();
        let top_card = triple_cards_vec.last().unwrap();
        let mut template =
            create_tower_template(TowerKind::ThreeOfAKind, top_card.suit, top_card.rank);
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    if let (Some(pair_high_cards_vec), Some(pair_low_cards_vec)) =
        (&pair_high_cards, &pair_low_cards)
    {
        let mut combined_cards = pair_high_cards_vec
            .iter()
            .chain(pair_low_cards_vec)
            .collect::<Vec<_>>();
        combined_cards.sort();
        let top_card = combined_cards.last().unwrap();
        let mut template = create_tower_template(TowerKind::TwoPair, top_card.suit, top_card.rank);
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    if let Some(mut pair_high_cards_vec) = pair_high_cards {
        pair_high_cards_vec.sort();
        let top_card = pair_high_cards_vec.last().unwrap();
        let mut template = create_tower_template(TowerKind::OnePair, top_card.suit, top_card.rank);
        inject_skills(&mut template);
        inject_status_effects(&mut template, upgrade_state, rerolled_count);
        return template;
    }

    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort();
    let top_card = sorted_cards.last().unwrap();
    let mut template = create_tower_template(TowerKind::High, top_card.suit, top_card.rank);
    inject_skills(&mut template);
    inject_status_effects(&mut template, upgrade_state, rerolled_count);
    template
}

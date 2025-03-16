use crate::{
    card::{Card, REVERSED_RANKS, Rank, Suit},
    game_state::{
        GameState,
        projectile::ProjectileKind,
        tower::{
            TowerKind, TowerSkillKind, TowerSkillTemplate, TowerStatusEffect, TowerStatusEffectEnd,
            TowerStatusEffectKind, TowerTemplate,
        },
        upgrade::TowerSelectUpgradeTarget,
    },
};
use namui::{DurationExt, Per};
use std::collections::HashMap;

pub fn get_highest_tower_template(cards: &[Card], game_state: &GameState) -> TowerTemplate {
    let mut highest_tower = highest_tower(cards);
    inject_skills(&mut highest_tower);
    inject_status_effects(&mut highest_tower, game_state);
    highest_tower
}

fn highest_tower(cards: &[Card]) -> TowerTemplate {
    let straight_result = check_straight(cards);
    let flush_result = check_flush(cards);

    if let (Some(straight_result), Some(flush_result)) = (&straight_result, &flush_result) {
        if straight_result.royal {
            create_tower_template(TowerKind::RoyalFlush, flush_result.suit, Rank::Ace);
        }
        return create_tower_template(
            TowerKind::StraightFlush,
            flush_result.suit,
            straight_result.top.rank,
        );
    }

    let rank_map = count_rank(cards);
    let mut triple_cards = None;
    let mut pair_high_cards = None;
    let mut pair_low_cards = None;

    for rank in REVERSED_RANKS {
        let Some(cards) = rank_map.get(&rank) else {
            continue;
        };
        if cards.len() == 4 {
            let mut cards = cards.clone();
            cards.sort();
            let top = cards.last().unwrap();
            return create_tower_template(TowerKind::FourOfAKind, top.suit, top.rank);
        }

        if cards.len() == 3 && triple_cards.is_none() {
            triple_cards = Some(cards.clone());
        } else if cards.len() == 2 {
            if pair_high_cards.is_none() {
                pair_high_cards = Some(cards.clone());
            } else if pair_low_cards.is_none() {
                pair_low_cards = Some(cards.clone());
            }
        }
    }

    if let (Some(triple_cards), Some(pair_high_cards)) = (&triple_cards, &pair_high_cards) {
        let mut cards = triple_cards
            .iter()
            .chain(pair_high_cards)
            .collect::<Vec<_>>();
        cards.sort();
        let top = cards.last().unwrap();
        return create_tower_template(TowerKind::FullHouse, top.suit, top.rank);
    }

    if let Some(flush_result) = flush_result {
        let mut cards = cards.to_vec();
        cards.sort();
        let top = cards.last().unwrap();
        return create_tower_template(TowerKind::Flush, flush_result.suit, top.rank);
    }

    if let Some(straight_result) = straight_result {
        return create_tower_template(
            TowerKind::Straight,
            straight_result.top.suit,
            straight_result.top.rank,
        );
    }

    if let Some(mut triple_cards) = triple_cards {
        triple_cards.sort();
        let top = triple_cards.last().unwrap();
        return create_tower_template(TowerKind::ThreeOfAKind, top.suit, top.rank);
    }

    if let (Some(pair_high_cards), Some(pair_low_cards)) = (&pair_high_cards, &pair_low_cards) {
        let mut cards = pair_high_cards
            .iter()
            .chain(pair_low_cards)
            .collect::<Vec<_>>();
        cards.sort();
        let top = cards.last().unwrap();
        return create_tower_template(TowerKind::TwoPair, top.suit, top.rank);
    }

    if let Some(mut cards) = pair_high_cards {
        cards.sort();
        let top = cards.last().unwrap();
        return create_tower_template(TowerKind::OnePair, top.suit, top.rank);
    }

    let mut cards = cards.to_vec();
    cards.sort();
    let top = cards.last().unwrap();

    create_tower_template(TowerKind::High, top.suit, top.rank)
}

fn inject_skills(tower: &mut TowerTemplate) {
    let hand_ranking_skill = match tower.kind {
        TowerKind::Barricade => None,
        TowerKind::High => None,
        TowerKind::OnePair => Some(TowerSkillTemplate {
            kind: TowerSkillKind::MoneyIncomeAdd { add: 1 },
            cooldown: 1.sec(),
            duration: 1.sec(),
        }),
        TowerKind::TwoPair => Some(TowerSkillTemplate {
            kind: TowerSkillKind::MoneyIncomeAdd { add: 2 },
            cooldown: 1.sec(),
            duration: 1.sec(),
        }),
        TowerKind::ThreeOfAKind => Some(TowerSkillTemplate {
            kind: TowerSkillKind::NearbyMonsterSpeedMul {
                mul: 0.9,
                range_radius: 4.0,
            },
            cooldown: 1.sec(),
            duration: 1.sec(),
        }),
        TowerKind::Straight => None,
        TowerKind::Flush => None,
        TowerKind::FullHouse => Some(TowerSkillTemplate {
            kind: TowerSkillKind::NearbyTowerAttackSpeedMul {
                mul: 2.0,
                range_radius: 2.0,
            },
            cooldown: 1.sec(),
            duration: 1.sec(),
        }),
        TowerKind::FourOfAKind => Some(TowerSkillTemplate {
            kind: TowerSkillKind::NearbyMonsterSpeedMul {
                mul: 0.75,
                range_radius: 4.0,
            },
            cooldown: 1.sec(),
            duration: 1.sec(),
        }),
        TowerKind::StraightFlush => None,
        TowerKind::RoyalFlush => Some(TowerSkillTemplate {
            kind: TowerSkillKind::NearbyTowerDamageMul {
                mul: 2.0,
                range_radius: 6.0,
            },
            cooldown: 1.sec(),
            duration: 1.sec(),
        }),
    };
    if let Some(skill) = hand_ranking_skill {
        tower.skill_templates.push(skill);
    }

    let top_card_effect = TowerSkillTemplate {
        kind: TowerSkillKind::TopCardBonus {
            rank: tower.rank,
            bonus_damage: tower.rank.bonus_damage(),
        },
        cooldown: 1.sec(),
        duration: 1.sec(),
    };
    tower.skill_templates.push(top_card_effect);

    // TODO: Inject effects from upgrades
}

fn inject_status_effects(tower: &mut TowerTemplate, game_state: &GameState) {
    if tower.kind.is_low_card_tower() {
        if let Some(upgrade) = game_state
            .upgrade_state
            .tower_select_upgrade_states
            .get(&TowerSelectUpgradeTarget::LowCard)
        {
            if upgrade.damage_plus > 0.0 {
                let upgrade_effect = TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageAdd {
                        add: upgrade.damage_plus as f32,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                };
                tower.default_status_effects.push(upgrade_effect);
            }

            if upgrade.damage_multiplier > 0.0 {
                let upgrade_effect = TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul {
                        mul: upgrade.damage_multiplier as f32,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                };
                tower.default_status_effects.push(upgrade_effect);
            }

            if upgrade.speed_plus > 0.0 {
                let upgrade_effect = TowerStatusEffect {
                    kind: TowerStatusEffectKind::AttackSpeedAdd {
                        add: upgrade.speed_plus as f32,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                };
                tower.default_status_effects.push(upgrade_effect);
            }

            if upgrade.speed_multiplier > 0.0 {
                let upgrade_effect = TowerStatusEffect {
                    kind: TowerStatusEffectKind::AttackSpeedMul {
                        mul: upgrade.speed_multiplier as f32,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                };
                tower.default_status_effects.push(upgrade_effect);
            }

            if upgrade.range_plus > 0.0 {
                let upgrade_effect = TowerStatusEffect {
                    kind: TowerStatusEffectKind::AttackRangeAdd {
                        add: upgrade.range_plus as f32,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                };
                tower.default_status_effects.push(upgrade_effect);
            }
        }
    }
}

struct StraightResult {
    royal: bool,
    top: Card,
}
fn check_straight(cards: &[Card]) -> Option<StraightResult> {
    if cards.len() != 5 {
        return None;
    }

    let mut cards_ace_as_high = cards
        .iter()
        .map(|card| {
            let mut rank = card.rank as usize;
            if rank == 0 {
                rank = Rank::King as usize + 1;
            }
            (rank, card)
        })
        .collect::<Vec<_>>();
    cards_ace_as_high.sort_by(|a, b| a.0.cmp(&b.0));
    let straight = check_rank(&cards_ace_as_high);
    if straight {
        return Some(StraightResult {
            royal: true,
            top: *cards_ace_as_high.last().unwrap().1,
        });
    }

    let mut cards_ace_as_low = cards
        .iter()
        .map(|card| (card.rank as usize, card))
        .collect::<Vec<_>>();
    cards_ace_as_low.sort_by(|a, b| a.0.cmp(&b.0));
    let straight = check_rank(&cards_ace_as_low);
    if straight {
        return Some(StraightResult {
            royal: false,
            top: *cards_ace_as_low.last().unwrap().1,
        });
    }

    return None;

    fn check_rank(cards: &[(usize, &Card)]) -> bool {
        let mut prev = cards[0];
        for (rank, card) in cards.iter().skip(1) {
            if *rank != prev.0 + 1 {
                return false;
            }
            prev = (*rank, card);
        }
        true
    }
}

struct FlushResult {
    suit: Suit,
}
fn check_flush(cards: &[Card]) -> Option<FlushResult> {
    if cards.len() != 5 {
        return None;
    }
    let suit = cards[0].suit;
    for card in cards.iter().skip(1) {
        if card.suit != suit {
            return None;
        }
    }
    Some(FlushResult { suit })
}

fn count_rank(cards: &[Card]) -> HashMap<Rank, Vec<Card>> {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank).or_insert_with(Vec::new).push(*card);
    }
    map
}

fn create_tower_template(kind: TowerKind, suit: Suit, rank: Rank) -> TowerTemplate {
    let shoot_interval = match kind {
        TowerKind::Barricade => 8192.0,
        TowerKind::High => 1.0,
        TowerKind::OnePair => 1.0,
        TowerKind::TwoPair => 1.0,
        TowerKind::ThreeOfAKind => 1.0,
        TowerKind::Straight => 1.0,
        TowerKind::Flush => 0.5,
        TowerKind::FullHouse => 1.0,
        TowerKind::FourOfAKind => 1.0,
        TowerKind::StraightFlush => 0.5,
        TowerKind::RoyalFlush => 1.0 / 3.0,
    }
    .sec();

    let default_attack_range_radius = match kind {
        TowerKind::Barricade => 0.0,
        TowerKind::High => 5.0,
        TowerKind::OnePair => 5.0,
        TowerKind::TwoPair => 5.0,
        TowerKind::ThreeOfAKind => 5.0,
        TowerKind::Straight => 10.0,
        TowerKind::Flush => 5.0,
        TowerKind::FullHouse => 5.0,
        TowerKind::FourOfAKind => 5.0,
        TowerKind::StraightFlush => 10.0,
        TowerKind::RoyalFlush => 15.0,
    };

    let default_damage = match kind {
        TowerKind::Barricade => 0.0,
        TowerKind::High => 1.0,
        TowerKind::OnePair => 5.0,
        TowerKind::TwoPair => 10.0,
        TowerKind::ThreeOfAKind => 25.0,
        TowerKind::Straight => 50.0,
        TowerKind::Flush => 75.0,
        TowerKind::FullHouse => 200.0,
        TowerKind::FourOfAKind => 250.0,
        TowerKind::StraightFlush => 1500.0,
        TowerKind::RoyalFlush => 3000.0,
    };

    TowerTemplate {
        kind,
        shoot_interval,
        default_attack_range_radius,
        projectile_kind: ProjectileKind::Ball,
        projectile_speed: Per::new(32.0, 1.sec()),
        default_damage,
        suit,
        rank,
        skill_templates: vec![],
        default_status_effects: vec![],
    }
}

use crate::game_state::tower::{TowerKind, TowerSkillKind, TowerSkillTemplate, TowerTemplate};
use namui::DurationExt;

pub fn inject_skills(tower: &mut TowerTemplate) {
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
}

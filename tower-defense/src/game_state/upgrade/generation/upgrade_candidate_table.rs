use super::*;
use crate::game_state::upgrade::UpgradeDiscriminants;
use crate::game_state::{GameState, tower::TowerKind};

pub struct CandidateRow {
    pub weight: f32,
    pub upgrade: Upgrade,
}

pub fn generate_tower_damage_upgrade_candidate_table(game_state: &GameState) -> Vec<CandidateRow> {
    game_state
        .config
        .upgrades
        .tower_damage_upgrades
        .iter()
        .map(|upgrade| {
            let disc: UpgradeDiscriminants = upgrade.name.parse().unwrap_or_else(|_| {
                panic!("Unknown tower damage upgrade config name: {}", upgrade.name)
            });
            CandidateRow {
                weight: upgrade.entry.weight,
                upgrade: disc.generate(&game_state.upgrade_state),
            }
        })
        .collect()
}

pub fn generate_treasure_upgrade_candidate_table(game_state: &GameState) -> Vec<CandidateRow> {
    let upgrade_state = &game_state.upgrade_state;

    let mut rows = Vec::with_capacity(16);
    let mut push_row = |upgrade: Upgrade, current_and_max: Option<(usize, usize)>, weight: f32| {
        let actual_weight = if let Some((current, max)) = current_and_max {
            if current >= max { 0.0 } else { weight }
        } else {
            weight
        };
        rows.push(CandidateRow {
            weight: actual_weight,
            upgrade,
        });
    };

    for upgrade in &game_state.config.upgrades.treasure_upgrades {
        let disc: UpgradeDiscriminants = upgrade
            .name
            .parse()
            .unwrap_or_else(|_| panic!("Unknown treasure upgrade config name: {}", upgrade.name));
        let weight = upgrade.entry.weight;
        let next_upgrade = disc.generate(upgrade_state);
        let current_and_max = disc.current_and_max(upgrade_state);
        push_row(next_upgrade, current_and_max, weight);
    }

    rows
}

#[allow(dead_code)]
fn get_tower_kind_with_weight(weights: &[f32; 10]) -> TowerKind {
    const TOWER_KINDS: [TowerKind; 10] = [
        TowerKind::High,
        TowerKind::OnePair,
        TowerKind::TwoPair,
        TowerKind::ThreeOfAKind,
        TowerKind::Straight,
        TowerKind::Flush,
        TowerKind::FullHouse,
        TowerKind::FourOfAKind,
        TowerKind::StraightFlush,
        TowerKind::RoyalFlush,
    ];

    *TOWER_KINDS
        .iter()
        .zip(weights)
        .collect::<Vec<_>>()
        .choose_weighted(&mut thread_rng(), |x| x.1)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_treasure_upgrade_kind_gen_maxed_cat_does_not_panic() {
        let upgrade_state = UpgradeState::with_upgrades(vec![
            crate::game_state::upgrade::CatUpgrade::into_upgrade(MAX_GOLD_EARN_PLUS),
        ]);
        let upgrade = UpgradeDiscriminants::Cat.generate(&upgrade_state);
        assert!(matches!(upgrade, Upgrade::Cat(..)));
    }
}

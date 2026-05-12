use super::*;
use crate::game_state::upgrade::UpgradeDiscriminants;
use crate::game_state::{GameState, tower::TowerKind};

pub struct CandidateRow {
    pub weight: f32,
    pub upgrade: Upgrade,
}

const TOWER_DAMAGE_UPGRADES: &[(UpgradeDiscriminants, f32)] = &[
    (UpgradeDiscriminants::Staff, 13.0),
    (UpgradeDiscriminants::LongSword, 13.0),
    (UpgradeDiscriminants::Mace, 13.0),
    (UpgradeDiscriminants::ClubSword, 13.0),
    (UpgradeDiscriminants::Tricycle, 50.0),
    (UpgradeDiscriminants::SingleChopstick, 20.0),
    (UpgradeDiscriminants::PairChopsticks, 20.0),
    (UpgradeDiscriminants::FountainPen, 20.0),
    (UpgradeDiscriminants::Brush, 20.0),
    (UpgradeDiscriminants::BrokenPottery, 20.0),
];

const TREASURE_UPGRADES: &[(UpgradeDiscriminants, f32)] = &[
    (UpgradeDiscriminants::Trophy, 10.0),
    (UpgradeDiscriminants::Crock, 10.0),
    (UpgradeDiscriminants::DemolitionHammer, 10.0),
    (UpgradeDiscriminants::Metronome, 10.0),
    (UpgradeDiscriminants::Tape, 10.0),
    (UpgradeDiscriminants::NameTag, 10.0),
    (UpgradeDiscriminants::ShoppingBag, 10.0),
    (UpgradeDiscriminants::Resolution, 10.0),
    (UpgradeDiscriminants::Mirror, 10.0),
    (UpgradeDiscriminants::IceCream, 10.0),
    (UpgradeDiscriminants::Spanner, 10.0),
    (UpgradeDiscriminants::Pea, 10.0),
    (UpgradeDiscriminants::SlotMachine, 10.0),
    (UpgradeDiscriminants::PiggyBank, 10.0),
    (UpgradeDiscriminants::Camera, 10.0),
    (UpgradeDiscriminants::GiftBox, 10.0),
    (UpgradeDiscriminants::Fang, 10.0),
    (UpgradeDiscriminants::Popcorn, 10.0),
    (UpgradeDiscriminants::MembershipCard, 10.0),
    (UpgradeDiscriminants::Eraser, 10.0),
];

pub fn generate_tower_damage_upgrade_candidate_table(game_state: &GameState) -> Vec<CandidateRow> {
    TOWER_DAMAGE_UPGRADES
        .iter()
        .map(|(discriminant, weight)| CandidateRow {
            weight: *weight,
            upgrade: discriminant.generate(&game_state.upgrade_state),
        })
        .collect()
}

pub fn generate_treasure_upgrade_candidate_table(game_state: &GameState) -> Vec<CandidateRow> {
    let upgrade_state = &game_state.upgrade_state;

    let mut rows = Vec::with_capacity(TREASURE_UPGRADES.len());
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

    for (discriminant, weight) in TREASURE_UPGRADES {
        let next_upgrade = discriminant.generate(upgrade_state);
        let current_and_max = discriminant.current_and_max(upgrade_state);
        push_row(next_upgrade, current_and_max, *weight);
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

    #[test]
    fn treasure_candidate_table_does_not_duplicate_broken_pottery() {
        let treasure_names: std::collections::HashSet<_> = TREASURE_UPGRADES
            .iter()
            .map(|(discriminant, _)| discriminant.as_ref())
            .collect();
        assert!(!treasure_names.contains("BrokenPottery"));
    }
}

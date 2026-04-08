use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeCandidateEntry {
    pub weight: f32,
    pub damage_multiplier_range: Option<(f32, f32)>,
}

#[cfg_attr(feature = "simulator", derive(serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeConfig {
    pub tower_damage_upgrades: Vec<(String, UpgradeCandidateEntry)>,
    pub treasure_upgrades: Vec<(String, UpgradeCandidateEntry)>,
}

pub fn default_upgrade_config() -> UpgradeConfig {
    UpgradeConfig {
        tower_damage_upgrades: vec![
            (
                "CainSword".into(),
                UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            ),
            (
                "LongSword".into(),
                UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            ),
            (
                "Mace".into(),
                UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            ),
            (
                "ClubSword".into(),
                UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            ),
            (
                "Spoon".into(),
                UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier_range: Some((1.3, 1.75)),
                },
            ),
            (
                "SingleChopstick".into(),
                UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            ),
            (
                "PairChopsticks".into(),
                UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            ),
            (
                "FountainPen".into(),
                UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            ),
            (
                "Brush".into(),
                UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            ),
            (
                "BrokenPottery".into(),
                UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.15, 1.25)),
                },
            ),
        ],
        treasure_upgrades: vec![
            (
                "Magnet".into(),
                UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "Backpack".into(),
                UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "DiceBundle".into(),
                UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "EnergyDrink".into(),
                UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "PerfectPottery".into(),
                UpgradeCandidateEntry {
                    weight: 25.0,
                    damage_multiplier_range: Some((1.3, 1.75)),
                },
            ),
            (
                "BrokenPottery".into(),
                UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.15, 1.25)),
                },
            ),
            (
                "FourLeafClover".into(),
                UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "Rabbit".into(),
                UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "BlackWhite".into(),
                UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            ),
            (
                "Eraser".into(),
                UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            ),
        ],
    }
}

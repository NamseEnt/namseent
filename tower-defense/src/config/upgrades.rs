use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeCandidateEntry {
    pub weight: f32,
    pub damage_multiplier_range: Option<(f32, f32)>,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeCandidate {
    pub name: String,
    pub entry: UpgradeCandidateEntry,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeConfig {
    pub tower_damage_upgrades: Vec<UpgradeCandidate>,
    pub treasure_upgrades: Vec<UpgradeCandidate>,
}

pub fn default_upgrade_config() -> UpgradeConfig {
    UpgradeConfig {
        tower_damage_upgrades: vec![
            UpgradeCandidate {
                name: "CainSword".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            },
            UpgradeCandidate {
                name: "LongSword".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            },
            UpgradeCandidate {
                name: "Mace".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            },
            UpgradeCandidate {
                name: "ClubSword".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier_range: Some((1.15, 1.5)),
                },
            },
            UpgradeCandidate {
                name: "Spoon".into(),
                entry: UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier_range: Some((1.3, 1.75)),
                },
            },
            UpgradeCandidate {
                name: "SingleChopstick".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            },
            UpgradeCandidate {
                name: "PairChopsticks".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            },
            UpgradeCandidate {
                name: "FountainPen".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            },
            UpgradeCandidate {
                name: "Brush".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.2, 1.4)),
                },
            },
            UpgradeCandidate {
                name: "BrokenPottery".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.15, 1.25)),
                },
            },
        ],
        treasure_upgrades: vec![
            UpgradeCandidate {
                name: "Magnet".into(),
                entry: UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "Backpack".into(),
                entry: UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "DiceBundle".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "EnergyDrink".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "PerfectPottery".into(),
                entry: UpgradeCandidateEntry {
                    weight: 25.0,
                    damage_multiplier_range: Some((1.3, 1.75)),
                },
            },
            UpgradeCandidate {
                name: "BrokenPottery".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier_range: Some((1.15, 1.25)),
                },
            },
            UpgradeCandidate {
                name: "FourLeafClover".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "Rabbit".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "BlackWhite".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            },
            UpgradeCandidate {
                name: "Eraser".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier_range: None,
                },
            },
        ],
    }
}

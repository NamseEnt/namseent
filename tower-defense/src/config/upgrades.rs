use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeCandidateEntry {
    pub weight: f32,
    pub damage_multiplier: Option<f32>,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize))]
#[derive(Clone, Debug, State)]
pub struct UpgradeCandidate {
    pub name: String,
    #[cfg_attr(feature = "simulator", serde(flatten))]
    pub entry: UpgradeCandidateEntry,
}

#[cfg(feature = "simulator")]
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum UpgradeCandidateHelper {
    Flattened {
        name: String,
        weight: f32,
        damage_multiplier: Option<f32>,
    },
    Nested {
        name: String,
        entry: UpgradeCandidateEntry,
    },
}

#[cfg(feature = "simulator")]
impl<'de> serde::Deserialize<'de> for UpgradeCandidate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match UpgradeCandidateHelper::deserialize(deserializer)? {
            UpgradeCandidateHelper::Flattened {
                name,
                weight,
                damage_multiplier,
            } => Ok(UpgradeCandidate {
                name,
                entry: UpgradeCandidateEntry {
                    weight,
                    damage_multiplier,
                },
            }),
            UpgradeCandidateHelper::Nested { name, entry } => Ok(UpgradeCandidate { name, entry }),
        }
    }
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
                name: "Staff".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier: Some(1.5),
                },
            },
            UpgradeCandidate {
                name: "LongSword".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier: Some(1.5),
                },
            },
            UpgradeCandidate {
                name: "Mace".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier: Some(1.5),
                },
            },
            UpgradeCandidate {
                name: "ClubSword".into(),
                entry: UpgradeCandidateEntry {
                    weight: 13.0,
                    damage_multiplier: Some(1.5),
                },
            },
            UpgradeCandidate {
                name: "Tricycle".into(),
                entry: UpgradeCandidateEntry {
                    weight: 50.0,
                    damage_multiplier: Some(1.75),
                },
            },
            UpgradeCandidate {
                name: "SingleChopstick".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier: Some(1.4),
                },
            },
            UpgradeCandidate {
                name: "PairChopsticks".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier: Some(1.4),
                },
            },
            UpgradeCandidate {
                name: "FountainPen".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier: Some(1.4),
                },
            },
            UpgradeCandidate {
                name: "Brush".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier: Some(1.4),
                },
            },
            UpgradeCandidate {
                name: "BrokenPottery".into(),
                entry: UpgradeCandidateEntry {
                    weight: 20.0,
                    damage_multiplier: Some(1.25),
                },
            },
        ],
        treasure_upgrades: vec![
            UpgradeCandidate {
                name: "Trophy".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(2.0),
                },
            },
            UpgradeCandidate {
                name: "Crock".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "DemolitionHammer".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(2.0),
                },
            },
            UpgradeCandidate {
                name: "Metronome".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "Tape".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "NameTag".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(3.0),
                },
            },
            UpgradeCandidate {
                name: "ShoppingBag".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(1.5),
                },
            },
            UpgradeCandidate {
                name: "Resolution".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(0.25),
                },
            },
            UpgradeCandidate {
                name: "Mirror".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "IceCream".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(3.0),
                },
            },
            UpgradeCandidate {
                name: "Spanner".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "Pea".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "SlotMachine".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "PiggyBank".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "Camera".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "GiftBox".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "Fang".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
            UpgradeCandidate {
                name: "Popcorn".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: Some(5.0),
                },
            },
            UpgradeCandidate {
                name: "MembershipCard".into(),
                entry: UpgradeCandidateEntry {
                    weight: 10.0,
                    damage_multiplier: None,
                },
            },
        ],
    }
}

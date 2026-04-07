//! Injectable game configuration data.
//!
//! All hardcoded game parameters are extracted here so they can be viewed at a glance
//! and modified for balance simulations.

use crate::card::Rank;
use crate::game_state::monster::MonsterKind;
use crate::game_state::tower::TowerKind;
use std::collections::HashMap;

/// Top-level configuration for a full game simulation.
#[derive(Clone, Debug)]
pub struct GameConfig {
    pub map: MapConfig,
    pub player: PlayerConfig,
    pub monsters: MonsterConfig,
    pub towers: TowerConfig,
    pub items: ItemConfig,
    pub shop: ShopConfig,
    pub upgrades: UpgradeConfig,
    pub rarity_weights: [usize; 4],
}

#[derive(Clone, Debug)]
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub travel_points: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
pub struct PlayerConfig {
    pub max_hp: f32,
    pub starting_gold: usize,
    pub starting_hp: f32,
    pub base_dice_chance: usize,
    pub max_stages: usize,
    pub base_hand_slots: usize,
}

#[derive(Clone, Debug)]
pub struct MonsterStats {
    pub base_hp: f32,
    pub velocity_mul: f32,
    pub damage: f32,
    pub reward: usize,
}

#[derive(Clone, Debug)]
pub struct StageWave {
    pub entries: Vec<(MonsterKind, usize)>,
}

#[derive(Clone, Debug)]
pub struct MonsterConfig {
    pub stats: HashMap<MonsterKind, MonsterStats>,
    pub stage_waves: HashMap<usize, StageWave>,
}

#[derive(Clone, Debug)]
pub struct TowerStats {
    pub damage: f32,
    pub range: f32,
    pub cooldown_ms: u64,
}

#[derive(Clone, Debug)]
pub struct TowerConfig {
    pub stats: HashMap<TowerKind, TowerStats>,
    pub rank_bonus_damage: HashMap<Rank, usize>,
}

#[derive(Clone, Debug)]
pub struct ItemCandidateConfig {
    pub weight: f32,
    pub min_value: f32,
    pub max_value: f32,
}

#[derive(Clone, Debug)]
pub struct ItemConfig {
    pub heal: ItemCandidateConfig,
    pub extra_reroll: ItemCandidateConfig,
    pub shield: ItemCandidateConfig,
    pub damage_reduction: ItemCandidateConfig,
    pub grant_barricades: ItemCandidateConfig,
    pub grant_card: ItemCandidateConfig,
}

#[derive(Clone, Debug)]
pub struct ShopConfig {
    pub base_cost: f32,
    pub value_cost_multiplier: f32,
    /// Distribution: [item%, tower_upgrade%, extra_upgrade%] out of 10
    pub slot_type_distribution: [usize; 3],
}

#[derive(Clone, Debug)]
pub struct UpgradeCandidateEntry {
    pub weight: f32,
    pub damage_multiplier_range: Option<(f32, f32)>,
}

#[derive(Clone, Debug)]
pub struct UpgradeConfig {
    pub tower_damage_upgrades: Vec<(String, UpgradeCandidateEntry)>,
    pub treasure_upgrades: Vec<(String, UpgradeCandidateEntry)>,
}

impl GameConfig {
    /// Returns the default configuration matching current hardcoded values.
    pub fn default_config() -> Self {
        Self {
            map: MapConfig {
                width: 36,
                height: 36,
                travel_points: vec![
                    (5, 0),
                    (5, 17),
                    (31, 17),
                    (31, 5),
                    (18, 5),
                    (18, 31),
                    (35, 31),
                ],
            },
            player: PlayerConfig {
                max_hp: 100.0,
                starting_gold: 100,
                starting_hp: 100.0,
                base_dice_chance: 1,
                max_stages: 50,
                base_hand_slots: 5,
            },
            monsters: default_monster_config(),
            towers: default_tower_config(),
            items: default_item_config(),
            shop: ShopConfig {
                base_cost: 50.0,
                value_cost_multiplier: 0.5,
                slot_type_distribution: [3, 5, 2], // 30% items, 50% tower upgrades, 20% extra
            },
            upgrades: default_upgrade_config(),
            rarity_weights: [90, 10, 1, 0],
        }
    }
}

fn default_monster_config() -> MonsterConfig {
    use MonsterKind::*;

    let mut stats = HashMap::new();
    let mut stage_waves = HashMap::new();

    // Monster base HP values
    let hp_table: Vec<(MonsterKind, f32)> = vec![
        (Mob01, 15.0),
        (Mob02, 21.0),
        (Mob03, 32.0),
        (Mob04, 60.0),
        (Mob05, 82.0),
        (Mob06, 101.0),
        (Mob07, 121.0),
        (Mob08, 143.0),
        (Mob09, 216.0),
        (Mob10, 297.0),
        (Mob11, 356.0),
        (Mob12, 421.0),
        (Mob13, 454.0),
        (Mob14, 513.0),
        (Mob15, 640.0),
        (Mob16, 762.0),
        (Mob17, 793.0),
        (Mob18, 860.0),
        (Mob19, 952.0),
        (Mob20, 1084.0),
        (Mob21, 1773.0),
        (Mob22, 2393.0),
        (Mob23, 2469.0),
        (Mob24, 2550.0),
        (Mob25, 2680.0),
        (Mob26, 2889.0),
        (Mob27, 3246.0),
        (Mob28, 3271.0),
        (Mob29, 3564.0),
        (Mob30, 4194.0),
        (Mob31, 4622.0),
        (Mob32, 6305.0),
        (Mob33, 6636.0),
        (Mob34, 7099.0),
        (Mob35, 7619.0),
        (Mob36, 8095.0),
        (Mob37, 9067.0),
        (Mob38, 10743.0),
        (Mob39, 12533.0),
        (Mob40, 13211.0),
        (Mob41, 14106.0),
        (Mob42, 15242.0),
        (Mob43, 16245.0),
        (Mob44, 17590.0),
        (Mob45, 19461.0),
        (Mob46, 21610.0),
        (Mob47, 21890.0),
        (Mob48, 22963.0),
        (Mob49, 23462.0),
        (Mob50, 24207.0),
        (Boss01, 1280.0),
        (Boss02, 5360.0),
        (Boss03, 8388.0),
        (Boss04, 15238.0),
        (Boss05, 26422.0),
        (Boss06, 38922.0),
        (Boss07, 43220.0),
        (Boss08, 43780.0),
        (Boss09, 45926.0),
        (Boss10, 46924.0),
        (Boss11, 48414.0),
    ];

    let damage_reward: Vec<(MonsterKind, f32, usize)> = vec![
        (Mob01, 1.0, 3),
        (Mob02, 1.0, 3),
        (Mob03, 1.0, 3),
        (Mob04, 1.0, 3),
        (Mob05, 1.0, 3),
        (Mob06, 1.0, 5),
        (Mob07, 1.0, 5),
        (Mob08, 1.0, 5),
        (Mob09, 1.0, 5),
        (Mob10, 1.0, 5),
        (Mob11, 1.0, 5),
        (Mob12, 1.0, 5),
        (Mob13, 1.0, 5),
        (Mob14, 1.0, 5),
        (Mob15, 1.0, 5),
        (Mob16, 1.0, 5),
        (Mob17, 1.0, 5),
        (Mob18, 1.0, 5),
        (Mob19, 1.0, 5),
        (Mob20, 1.0, 5),
        (Mob21, 1.0, 5),
        (Mob22, 1.0, 5),
        (Mob23, 1.0, 5),
        (Mob24, 1.0, 5),
        (Mob25, 1.0, 5),
        (Mob26, 1.0, 5),
        (Mob27, 1.0, 5),
        (Mob28, 1.0, 5),
        (Mob29, 1.0, 5),
        (Mob30, 1.0, 5),
        (Mob31, 1.0, 5),
        (Mob32, 1.0, 5),
        (Mob33, 1.0, 5),
        (Mob34, 1.0, 5),
        (Mob35, 1.0, 5),
        (Mob36, 1.0, 5),
        (Mob37, 1.0, 5),
        (Mob38, 1.0, 5),
        (Mob39, 1.0, 5),
        (Mob40, 1.0, 5),
        (Mob41, 1.0, 5),
        (Mob42, 1.0, 5),
        (Mob43, 1.0, 5),
        (Mob44, 1.0, 5),
        (Mob45, 1.0, 5),
        (Mob46, 1.0, 5),
        (Mob47, 1.0, 5),
        (Mob48, 1.0, 5),
        (Mob49, 1.0, 5),
        (Mob50, 1.0, 5),
        (Boss01, 15.0, 50),
        (Boss02, 20.0, 75),
        (Boss03, 20.0, 100),
        (Boss04, 25.0, 100),
        (Boss05, 25.0, 125),
        (Boss06, 25.0, 125),
        (Boss07, 50.0, 125),
        (Boss08, 50.0, 125),
        (Boss09, 50.0, 125),
        (Boss10, 50.0, 125),
        (Boss11, 50.0, 125),
    ];

    for (kind, hp) in &hp_table {
        let (_, damage, reward) = damage_reward.iter().find(|(k, _, _)| k == kind).unwrap();
        stats.insert(
            *kind,
            MonsterStats {
                base_hp: *hp,
                velocity_mul: 1.0,
                damage: *damage,
                reward: *reward,
            },
        );
    }

    // Stage wave composition
    let waves: Vec<(usize, Vec<(MonsterKind, usize)>)> = vec![
        (1, vec![(Mob01, 5)]),
        (2, vec![(Mob02, 5)]),
        (3, vec![(Mob03, 5)]),
        (4, vec![(Mob04, 5)]),
        (5, vec![(Mob05, 5)]),
        (6, vec![(Mob06, 7)]),
        (7, vec![(Mob07, 7)]),
        (8, vec![(Mob08, 7)]),
        (9, vec![(Mob09, 7)]),
        (10, vec![(Mob10, 7)]),
        (11, vec![(Mob11, 9)]),
        (12, vec![(Mob12, 9)]),
        (13, vec![(Mob13, 9)]),
        (14, vec![(Mob14, 9)]),
        (15, vec![(Mob15, 8), (Boss01, 1)]),
        (16, vec![(Mob16, 10)]),
        (17, vec![(Mob17, 10)]),
        (18, vec![(Mob18, 10)]),
        (19, vec![(Mob19, 10)]),
        (20, vec![(Mob20, 10)]),
        (21, vec![(Mob21, 11)]),
        (22, vec![(Mob22, 11)]),
        (23, vec![(Mob23, 11)]),
        (24, vec![(Mob24, 11)]),
        (25, vec![(Mob25, 10), (Boss02, 1)]),
        (26, vec![(Mob26, 11)]),
        (27, vec![(Mob27, 11)]),
        (28, vec![(Mob28, 11)]),
        (29, vec![(Mob29, 11)]),
        (30, vec![(Mob30, 10), (Boss03, 1)]),
        (31, vec![(Mob31, 12)]),
        (32, vec![(Mob32, 12)]),
        (33, vec![(Mob33, 12)]),
        (34, vec![(Mob34, 12)]),
        (35, vec![(Mob35, 11), (Boss04, 1)]),
        (36, vec![(Mob36, 13)]),
        (37, vec![(Mob37, 13)]),
        (38, vec![(Mob38, 13)]),
        (39, vec![(Mob39, 13)]),
        (40, vec![(Mob40, 12), (Boss05, 1)]),
        (41, vec![(Mob41, 14)]),
        (42, vec![(Mob42, 14)]),
        (43, vec![(Mob43, 14)]),
        (44, vec![(Mob44, 14)]),
        (45, vec![(Mob45, 13), (Boss06, 1)]),
        (46, vec![(Mob46, 14), (Boss07, 1)]),
        (47, vec![(Mob47, 14), (Boss08, 1)]),
        (48, vec![(Mob48, 14), (Boss09, 1)]),
        (49, vec![(Mob49, 14), (Boss10, 1)]),
        (50, vec![(Mob50, 14), (Boss11, 1)]),
    ];

    for (stage, entries) in waves {
        stage_waves.insert(stage, StageWave { entries });
    }

    MonsterConfig { stats, stage_waves }
}

fn default_tower_config() -> TowerConfig {
    use TowerKind::*;

    let mut stats = HashMap::new();
    let tower_data: Vec<(TowerKind, f32, f32, u64)> = vec![
        (Barricade, 0.0, 4.0, 1000),
        (High, 5.0, 4.0, 1000),
        (OnePair, 6.0, 5.0, 1000),
        (TwoPair, 10.0, 6.0, 1000),
        (ThreeOfAKind, 12.0, 7.0, 1000),
        (Straight, 14.0, 9.0, 500),
        (Flush, 32.0, 9.0, 1000),
        (FullHouse, 50.0, 11.0, 1000),
        (FourOfAKind, 100.0, 11.0, 1000),
        (StraightFlush, 250.0, 14.0, 500),
        (RoyalFlush, 1200.0, 15.0, 1000),
    ];

    for (kind, damage, range, cooldown_ms) in tower_data {
        stats.insert(
            kind,
            TowerStats {
                damage,
                range,
                cooldown_ms,
            },
        );
    }

    let mut rank_bonus = HashMap::new();
    use Rank::*;
    let bonus: Vec<(Rank, usize)> = vec![
        (Two, 0),
        (Three, 0),
        (Four, 0),
        (Five, 1),
        (Six, 1),
        (Seven, 1),
        (Eight, 2),
        (Nine, 3),
        (Ten, 4),
        (Jack, 6),
        (Queen, 8),
        (King, 10),
        (Ace, 15),
    ];
    for (rank, dmg) in bonus {
        rank_bonus.insert(rank, dmg);
    }

    TowerConfig {
        stats,
        rank_bonus_damage: rank_bonus,
    }
}

fn default_item_config() -> ItemConfig {
    ItemConfig {
        heal: ItemCandidateConfig {
            weight: 100.0,
            min_value: 10.0,
            max_value: 14.0,
        },
        extra_reroll: ItemCandidateConfig {
            weight: 10.0,
            min_value: 0.0,
            max_value: 0.0,
        },
        shield: ItemCandidateConfig {
            weight: 10.0,
            min_value: 15.0,
            max_value: 25.0,
        },
        damage_reduction: ItemCandidateConfig {
            weight: 10.0,
            min_value: 0.8,
            max_value: 0.85,
        },
        grant_barricades: ItemCandidateConfig {
            weight: 45.0,
            min_value: 5.0,
            max_value: 10.0,
        },
        grant_card: ItemCandidateConfig {
            weight: 35.0,
            min_value: 0.0,
            max_value: 0.0,
        },
    }
}

fn default_upgrade_config() -> UpgradeConfig {
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

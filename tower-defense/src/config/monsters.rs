use crate::game_state::monster::MonsterKind;
use namui::*;
use std::collections::HashMap;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct MonsterStats {
    pub base_hp: f32,
    pub velocity_mul: f32,
    pub damage: f32,
    pub reward: usize,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct StageWaveEntry {
    pub kind: MonsterKind,
    pub count: usize,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct StageWave {
    pub stage: usize,
    pub entries: Vec<StageWaveEntry>,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct MonsterConfig {
    pub stats: HashMap<MonsterKind, MonsterStats>,
    pub stage_waves: Vec<StageWave>,
}

pub fn default_monster_config() -> MonsterConfig {
    use MonsterKind::*;

    let mut stats = HashMap::new();
    let mut stage_waves = Vec::new();

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
        stage_waves.push(StageWave {
            stage,
            entries: entries
                .into_iter()
                .map(|(kind, count)| StageWaveEntry { kind, count })
                .collect(),
        });
    }

    MonsterConfig { stats, stage_waves }
}

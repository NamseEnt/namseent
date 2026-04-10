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
        (Mob01, 20.0),
        (Mob02, 26.0),
        (Mob03, 36.0),
        (Mob04, 60.0),
        (Mob05, 70.0),
        (Mob06, 82.0),
        (Mob07, 95.0),
        (Mob08, 105.0),
        (Mob09, 115.0),
        (Mob10, 135.0),
        (Mob11, 120.0),
        (Mob12, 130.0),
        (Mob13, 140.0),
        (Mob14, 190.0),
        (Mob15, 255.0),
        (Mob16, 345.0),
        (Mob17, 465.0),
        (Mob18, 625.0),
        (Mob19, 845.0),
        (Mob20, 1135.0),
        (Mob21, 1435.421),
        (Mob22, 1971.066),
        (Mob23, 2068.429),
        (Mob24, 2172.192),
        (Mob25, 2320.666),
        (Mob26, 2542.320),
        (Mob27, 2902.184),
        (Mob28, 2970.592),
        (Mob29, 3286.864),
        (Mob30, 3926.926),
        (Mob31, 12954.542),
        (Mob32, 18776.133),
        (Mob33, 20924.303),
        (Mob34, 30930.910),
        (Mob35, 34943.782),
        (Mob36, 38983.253),
        (Mob37, 45743.378),
        (Mob38, 56662.450),
        (Mob39, 68977.622),
        (Mob40, 75738.663),
        (Mob41, 215652.528),
        (Mob42, 239677.402),
        (Mob43, 262545.192),
        (Mob44, 291965.856),
        (Mob45, 331522.028),
        (Mob46, 363048.000),
        (Mob47, 376545.800),
        (Mob48, 405067.320),
        (Mob49, 423723.720),
        (Mob50, 447345.360),
        (Boss01, 105.0),
        (Boss02, 202.5),
        (Boss03, 382.5),
        (Boss04, 1702.5),
        (Boss05, 2572.8),
        (Boss06, 4026.24),
        (Boss07, 20159.874),
        (Boss08, 34956.306),
        (Boss09, 182154.96),
        (Boss10, 194490.0),
        (Boss11, 197010.0),
        (Boss12, 206667.0),
        (Boss13, 211158.0),
        (Boss14, 217863.0),
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
        (Boss12, 50.0, 125),
        (Boss13, 50.0, 125),
        (Boss14, 50.0, 125),
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
        (5, vec![(Mob05, 5), (Boss01, 1)]),
        (6, vec![(Mob06, 7)]),
        (7, vec![(Mob07, 7)]),
        (8, vec![(Mob08, 7)]),
        (9, vec![(Mob09, 7)]),
        (10, vec![(Mob10, 7), (Boss02, 1)]),
        (11, vec![(Mob11, 9)]),
        (12, vec![(Mob12, 9)]),
        (13, vec![(Mob13, 9)]),
        (14, vec![(Mob14, 9)]),
        (15, vec![(Mob15, 8), (Boss03, 1)]),
        (16, vec![(Mob16, 10)]),
        (17, vec![(Mob17, 10)]),
        (18, vec![(Mob18, 10)]),
        (19, vec![(Mob19, 10)]),
        (20, vec![(Mob20, 10), (Boss04, 1)]),
        (21, vec![(Mob21, 11)]),
        (22, vec![(Mob22, 11)]),
        (23, vec![(Mob23, 11)]),
        (24, vec![(Mob24, 11)]),
        (25, vec![(Mob25, 10), (Boss05, 1)]),
        (26, vec![(Mob26, 11)]),
        (27, vec![(Mob27, 11)]),
        (28, vec![(Mob28, 11)]),
        (29, vec![(Mob29, 11)]),
        (30, vec![(Mob30, 10), (Boss06, 1)]),
        (31, vec![(Mob31, 12)]),
        (32, vec![(Mob32, 12)]),
        (33, vec![(Mob33, 12)]),
        (34, vec![(Mob34, 12)]),
        (35, vec![(Mob35, 11), (Boss07, 1)]),
        (36, vec![(Mob36, 13)]),
        (37, vec![(Mob37, 13)]),
        (38, vec![(Mob38, 13)]),
        (39, vec![(Mob39, 13)]),
        (40, vec![(Mob40, 12), (Boss08, 1)]),
        (41, vec![(Mob41, 14)]),
        (42, vec![(Mob42, 14)]),
        (43, vec![(Mob43, 14)]),
        (44, vec![(Mob44, 14)]),
        (45, vec![(Mob45, 13), (Boss09, 1)]),
        (46, vec![(Mob46, 14), (Boss10, 1)]),
        (47, vec![(Mob47, 14), (Boss11, 1)]),
        (48, vec![(Mob48, 14), (Boss12, 1)]),
        (49, vec![(Mob49, 14), (Boss13, 1)]),
        (50, vec![(Mob50, 14), (Boss14, 1)]),
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

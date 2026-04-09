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
        (Mob21, 1134.72),
        (Mob22, 1531.52),
        (Mob23, 1580.16),
        (Mob24, 1632.0),
        (Mob25, 1715.2),
        (Mob26, 1848.96),
        (Mob27, 2077.44),
        (Mob28, 2093.44),
        (Mob29, 2280.96),
        (Mob30, 2684.16),
        (Mob31, 6228.145),
        (Mob32, 8495.9875),
        (Mob33, 8942.01),
        (Mob34, 12522.636),
        (Mob35, 13439.916),
        (Mob36, 14279.58),
        (Mob37, 15994.188),
        (Mob38, 18950.652),
        (Mob39, 22108.212),
        (Mob40, 23304.204),
        (Mob41, 88021.44),
        (Mob42, 95110.08),
        (Mob43, 101368.8),
        (Mob44, 109761.6),
        (Mob45, 121436.64),
        (Mob46, 129660.0),
        (Mob47, 131340.0),
        (Mob48, 137778.0),
        (Mob49, 140772.0),
        (Mob50, 145242.0),
        (Boss01, 87.5),
        (Boss02, 168.75),
        (Boss03, 318.75),
        (Boss04, 1418.75),
        (Boss05, 2144.0),
        (Boss06, 3355.2),
        (Boss07, 16799.895),
        (Boss08, 29130.255),
        (Boss09, 151795.8),
        (Boss10, 162075.0),
        (Boss11, 164175.0),
        (Boss12, 172222.5),
        (Boss13, 175965.0),
        (Boss14, 181552.5),
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

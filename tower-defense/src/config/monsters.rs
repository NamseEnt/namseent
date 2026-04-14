#![allow(clippy::excessive_precision)]

use crate::game_state::monster::MonsterKind;
use namui::*;
use std::collections::BTreeMap;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct MonsterStats {
    pub base_hp: f32,
    #[cfg_attr(
        feature = "simulator",
        serde(
            default = "default_velocity_mul",
            skip_serializing_if = "is_velocity_mul_default"
        )
    )]
    pub velocity_mul: f32,
    pub damage: f32,
    pub reward: usize,
}

#[cfg(feature = "simulator")]
fn default_velocity_mul() -> f32 {
    1.0
}

#[cfg(feature = "simulator")]
fn is_velocity_mul_default(value: &f32) -> bool {
    *value == 1.0
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
    pub stats: BTreeMap<MonsterKind, MonsterStats>,
    pub stage_waves: Vec<StageWave>,
}

pub fn default_monster_config() -> MonsterConfig {
    use MonsterKind::*;

    let mut stats = BTreeMap::new();
    let mut stage_waves = Vec::new();

    let hp_table: Vec<(MonsterKind, f32)> = vec![
        (Mob01, 36.05904),
        (Mob02, 46.876752),
        (Mob03, 64.906272),
        (Mob04, 108.17712),
        (Mob05, 126.20664),
        (Mob06, 147.842064),
        (Mob07, 171.28044),
        (Mob08, 189.30996),
        (Mob09, 207.33948),
        (Mob10, 243.39852),
        (Mob11, 216.35424),
        (Mob12, 234.38376),
        (Mob13, 252.41328),
        (Mob14, 342.56088),
        (Mob15, 459.75276),
        (Mob16, 622.01844),
        (Mob17, 838.37268),
        (Mob18, 933.02766),
        (Mob19, 1185.766373),
        (Mob20, 1592.707964),
        (Mob21, 1728.26291),
        (Mob22, 2373.186053),
        (Mob23, 2490.412189),
        (Mob24, 2615.344142),
        (Mob25, 3352.929802),
        (Mob26, 3673.178149),
        (Mob27, 4193.11545),
        (Mob28, 4291.952277),
        (Mob29, 4748.906455),
        (Mob30, 6013.03423),
        (Mob31, 16171.418703),
        (Mob32, 23438.603372),
        (Mob33, 26120.204998),
        (Mob34, 38573.178436),
        (Mob35, 43620.985285),
        (Mob36, 45058.829489),
        (Mob37, 52795.598014),
        (Mob38, 65493.352235),
        (Mob39, 79727.845582),
        (Mob40, 87542.601798),
        (Mob41, 254749.071426),
        (Mob42, 283129.513143),
        (Mob43, 310143.098887),
        (Mob44, 386285.263701),
        (Mob45, 438620.037481),
        (Mob46, 1761841.613186),
        (Mob47, 1827345.308055),
        (Mob48, 1965757.863151),
        (Mob49, 2056295.81542),
        (Mob50, 508121.593006),
        (Boss01, 189.30996),
        (Boss02, 365.09778),
        (Boss03, 689.62914),
        (Boss04, 2389.061946),
        (Boss05, 5029.394703),
        (Boss06, 9019.551345),
        (Boss07, 65431.4779275),
        (Boss08, 131313.902697),
        (Boss09, 657930.0562215),
        (Boss10, 2642762.419779),
        (Boss11, 2741017.9620825),
        (Boss12, 2948636.7947265),
        (Boss13, 3084443.72313),
        (Boss14, 762182.389509),
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

#![allow(clippy::excessive_precision)]

use crate::game_state::monster::MonsterKind;
use namui::*;
use std::collections::BTreeMap;

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
    pub stats: BTreeMap<MonsterKind, MonsterStats>,
    pub stage_waves: Vec<StageWave>,
}

pub fn default_monster_config() -> MonsterConfig {
    use MonsterKind::*;

    let mut stats = BTreeMap::new();
    let mut stage_waves = Vec::new();

    let hp_table: Vec<(MonsterKind, f32, f32, f32, usize)> = vec![
        (Mob01, 11.159947395324707, 1.0, 1.0, 3),
        (Mob02, 16.629634857177734, 1.0, 2.0, 3),
        (Mob03, 21.63849639892578, 0.75, 1.0, 3),
        (Mob04, 24.801904678344727, 1.25, 1.0, 3),
        (Mob05, 11.333049774169922, 1.0, 1.0, 3),
        (Mob06, 24.271482467651367, 1.0, 1.0, 5),
        (Mob07, 31.0927791595459, 1.0, 2.0, 5),
        (Mob08, 35.18561553955078, 0.75, 1.0, 5),
        (Mob09, 38.115047454833984, 1.25, 1.0, 5),
        (Mob10, 31.357189178466797, 1.0, 1.0, 5),
        (Mob11, 123.11029052734375, 1.0, 1.0, 5),
        (Mob12, 152.0933074951172, 1.0, 2.0, 5),
        (Mob13, 192.29974365234375, 0.75, 1.0, 5),
        (Mob14, 231.44918823242188, 1.25, 1.0, 5),
        (Mob15, 312.106201171875, 1.0, 1.0, 5),
        (Mob16, 353.5625305175781, 1.0, 1.0, 5),
        (Mob17, 435.1943054199219, 1.0, 2.0, 5),
        (Mob18, 558.5574340820312, 0.75, 1.0, 5),
        (Mob19, 691.9556274414062, 1.25, 1.0, 5),
        (Mob20, 899.2381591796875, 1.0, 1.0, 5),
        (Mob21, 1163.4063720703125, 1.0, 1.0, 5),
        (Mob22, 1439.0645751953125, 1.0, 2.0, 5),
        (Mob23, 1889.7984619140625, 0.75, 1.0, 5),
        (Mob24, 2341.726806640625, 1.25, 1.0, 5),
        (Mob25, 3466.861572265625, 1.0, 1.0, 5),
        (Mob26, 4436.10546875, 1.0, 1.0, 5),
        (Mob27, 5508.7021484375, 1.0, 2.0, 5),
        (Mob28, 7191.951171875, 0.75, 1.0, 5),
        (Mob29, 8879.296875, 1.25, 1.0, 5),
        (Mob30, 13443.1142578125, 1.0, 1.0, 5),
        (Mob31, 14235.20703125, 1.0, 1.0, 5),
        (Mob32, 17208.412109375, 1.0, 2.0, 5),
        (Mob33, 22520.205078125, 0.75, 1.0, 5),
        (Mob34, 27614.98046875, 1.25, 1.0, 5),
        (Mob35, 43119.890625, 1.0, 1.0, 5),
        (Mob36, 46515.19140625, 1.0, 1.0, 5),
        (Mob37, 57146.984375, 1.0, 2.0, 5),
        (Mob38, 81922.8125, 0.75, 1.0, 5),
        (Mob39, 108792.0703125, 1.25, 1.0, 5),
        (Mob40, 180621.34375, 1.0, 1.0, 5),
        (Mob41, 200966.171875, 1.0, 1.0, 5),
        (Mob42, 257917.59375, 1.0, 2.0, 5),
        (Mob43, 390106.90625, 0.75, 1.0, 5),
        (Mob44, 550543.6875, 1.25, 1.0, 5),
        (Mob45, 944978.875, 1.0, 1.0, 5),
        (Mob46, 1315346.875, 1.0, 1.0, 5),
        (Mob47, 2058359.625, 1.0, 4.0, 5),
        (Mob48, 2886561.25, 0.75, 2.0, 5),
        (Mob49, 4490391.5, 1.25, 2.0, 5),
        (Mob50, 7171733.5, 1.0, 2.0, 5),
        (Boss01, 16.999574661254883, 1.0, 5.0, 50),
        (Boss02, 47.03578186035156, 1.0, 5.0, 75),
        (Boss03, 468.1593017578125, 1.0, 5.0, 100),
        (Boss04, 1348.857177734375, 1.0, 7.0, 100),
        (Boss05, 5200.29248046875, 1.0, 7.0, 125),
        (Boss06, 20164.671875, 1.0, 7.0, 125),
        (Boss07, 64679.8359375, 1.0, 10.0, 125),
        (Boss08, 270932.0, 1.0, 10.0, 125),
        (Boss09, 1417468.25, 1.0, 10.0, 125),
        (Boss10, 1973020.25, 1.0, 10.0, 125),
        (Boss11, 3087539.5, 1.0, 15.0, 125),
        (Boss12, 4329842.0, 0.75, 10.0, 125),
        (Boss13, 6735587.0, 1.25, 10.0, 125),
        (Boss14, 10757600.0, 1.0, 10.0, 125),
    ];

    for (kind, hp, velocity_mul, damage, reward) in &hp_table {
        stats.insert(
            *kind,
            MonsterStats {
                base_hp: *hp,
                velocity_mul: *velocity_mul,
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

use crate::MonsterDamageResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AttackSourceState {
    pub tower_id: u64,
    pub tower_kind: u8,
    pub rank: Option<u8>,
    pub suit: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DamageSplash {
    pub radius_raw: i64,
    pub damage_pct_raw: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DamageHit {
    pub target_index: usize,
    pub damage_raw: i64,
    pub at_xy: [i64; 2],
    pub source_index: usize,
    pub splashes: Vec<DamageSplash>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageHitResult {
    pub target_index: usize,
    pub damage: MonsterDamageResult,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AreaDamageEvent {
    pub center_xy: [i64; 2],
    pub damage_raw: i64,
    pub source_index: usize,
    pub splashes: Vec<DamageSplash>,
}

pub(crate) fn damage_hit_sort_key(monster_ids: &[u64], target_index: usize) -> (u64, usize) {
    (
        monster_ids.get(target_index).copied().unwrap_or(u64::MAX),
        target_index,
    )
}

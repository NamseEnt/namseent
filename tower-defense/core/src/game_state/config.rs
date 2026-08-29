#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerConfigState {
    pub max_hp_raw: i64,
    pub starting_gold: usize,
    pub starting_hp_raw: i64,
    pub base_dice_chance: usize,
    pub max_stages: usize,
    pub base_hand_slots: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerConfigEntryState {
    pub kind: u8,
    pub damage_raw: i64,
    pub range_raw: i64,
    pub cooldown_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerConfigState {
    pub entries: Vec<TowerConfigEntryState>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterConfigEntryState {
    pub kind: u8,
    pub base_hp_raw: i64,
    pub velocity_mul_raw: i64,
    pub damage_raw: i64,
    pub reward: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageWaveEntryState {
    pub kind: u8,
    pub count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageWaveState {
    pub stage: usize,
    pub entries: Vec<StageWaveEntryState>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterConfigState {
    pub stats: Vec<MonsterConfigEntryState>,
    pub stage_waves: Vec<StageWaveState>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameConfigState {
    pub player: PlayerConfigState,
    pub towers: TowerConfigState,
    pub monsters: MonsterConfigState,
}

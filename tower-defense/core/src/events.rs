#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CoreEvent {
    DamageApplied {
        target_id: u64,
        amount: i64,
        position: [i64; 2],
    },
    BaseDamageApplied {
        amount: i64,
        actual_amount: i64,
    },
    MonsterDefeated {
        monster_id: u64,
        position: [i64; 2],
        monster_kind: u8,
        reward: usize,
        rotation_milliradians: i32,
    },
    TowerAttack {
        tower_id: u64,
        target_id: u64,
        attack_kind: u8,
        attack_ids: Vec<u64>,
        projectile_attack_ids: Vec<u64>,
    },
    ProjectileMoved {
        attack_id: u64,
        start_xy: [i64; 2],
        end_xy: [i64; 2],
    },
    TimedAttackExecuted {
        position: [i64; 2],
    },
    ProjectileHit {
        attack_id: u64,
        position: [i64; 2],
    },
    MonsterSpawned {
        monster_id: u64,
        monster_kind: u8,
        position: [i64; 2],
    },
    DefenseStarted {
        stage: usize,
    },
    TreasureSelected {
        upgrade: crate::UpgradeEntry,
    },
    DefenseEnded {
        stage: usize,
        perfect_clear: bool,
        transition: crate::DefenseEndTransitionState,
    },
    StageStarted {
        stage: usize,
        card_count: usize,
    },
    GameFinished {
        victory: bool,
    },
    CardServiceSelectionRequested {
        service_kind: String,
        step_counts: Vec<usize>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CoreEventQueue {
    pub(crate) events: Vec<CoreEvent>,
}

impl CoreEventQueue {
    pub fn extend(&mut self, events: impl IntoIterator<Item = CoreEvent>) {
        self.events.extend(events);
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

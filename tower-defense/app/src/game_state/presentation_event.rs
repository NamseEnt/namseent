#[derive(Clone)]
pub enum ParticleSpawnRequest {
    DamageText {
        position: [f32; 2],
        damage: f32,
    },
    MonsterSoul {
        position: [i64; 2],
        rotation_radians: f32,
    },
    MonsterCorpse {
        position: [i64; 2],
        rotation_radians: f32,
        monster_kind: crate::game_state::MonsterKind,
    },
}
#[derive(Clone, Copy)]
pub enum SoundCue {
    Coin,
    LuggageDrop,
    StartDefenseFanfare,
    Pickaxe,
    Fail,
    MonsterFootstep,
    PaperCrumpling,
    RedLaserShot,
    Whoop,
    Flamethrower,
    SmokeBomb,
}

#[derive(Clone, Copy)]
pub enum SoundVolume {
    Minimum,
    Low,
    High,
}

#[derive(Clone, Copy)]
pub enum BaseAnimationEvent {
    EnemySpawn,
    PlayerDamage { intensity: f32 },
}

#[derive(Clone)]
pub enum MonsterHitVisualSource {
    Tower {
        kind: crate::game_state::tower::TowerKind,
    },
}

#[derive(Clone)]
pub enum PresentationEvent {
    AnimateBase(BaseAnimationEvent),
    ShakeCamera {
        intensity: f32,
    },
    SpawnParticle(ParticleSpawnRequest),
    SpawnTowerAttackVisualEffect {
        tower_kind: crate::game_state::tower::TowerKind,
        start_xy: [f32; 2],
        end_xy: [f32; 2],
        attack_range_radius: f32,
    },
    SpawnMonsterHitVisualEffect {
        source: MonsterHitVisualSource,
        start_xy: [f32; 2],
        impact_xy: [f32; 2],
        chain_delay_ms: u64,
    },
    SpawnGoldReward {
        position: [f32; 2],
        amount: usize,
    },
    PlaySoundCue {
        cue: SoundCue,
        position: Option<[f32; 2]>,
        volume: SoundVolume,
        max_duration_ms: Option<i64>,
    },
    PlaySoundCueDelayed {
        cue: SoundCue,
        position: Option<[f32; 2]>,
        volume: SoundVolume,
        delay_ms: i64,
    },
    PlayCardDrawSounds {
        card_count: usize,
    },
    SaveDebugSnapshot,
    SpawnTowerRemoveDustBurst((f32, f32)),
}

#[derive(Clone, Default)]
pub struct PresentationEventQueue {
    pub events: Vec<PresentationEvent>,
}

impl PresentationEventQueue {
    pub fn push(&mut self, event: PresentationEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, PresentationEvent> {
        self.events.drain(..)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

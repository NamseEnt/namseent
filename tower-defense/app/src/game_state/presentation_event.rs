use crate::game_state::ProjectileTrail;
use crate::{AttackId, MonsterId, TowerId};

#[derive(Clone)]
pub enum ParticleSpawnRequest {
    DamageText {
        position: [f32; 2],
        damage: f32,
    },
    TrashBounce {
        projectile_kind: crate::game_state::projectile::ProjectileKind,
        start_xy: [f32; 2],
        end_xy: [f32; 2],
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
    KnifeSlash,
    Coin,
    LuggageDrop,
    StartDefenseFanfare,
    Pickaxe,
    Fail,
    MonsterFootstep,
    PaperCrumpling,
    RedLaserShot,
    Whoop,
    Wind,
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
pub enum PresentationEvent {
    AnimateBase(BaseAnimationEvent),
    ShakeCamera {
        intensity: f32,
    },
    SpawnRoyalStraightFlushVisual {
        tower_id: TowerId,
        target_xy: [f32; 2],
        target_monster_id: MonsterId,
        sim_tick: crate::SimTick,
    },
    SpawnParticle(ParticleSpawnRequest),
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
    SpawnProjectileTrail {
        trail: ProjectileTrail,
        start_xy: [f32; 2],
        end_xy: [f32; 2],
        count: usize,
    },
    SpawnProjectileHitEffect(crate::game_state::attack::ProjectileHitEffect, [f32; 2]),
    SpawnLaserBeam((f32, f32), (f32, f32)),
    SpawnTowerRemoveDustBurst((f32, f32)),
    SyncProjectileTrailState {
        projectile_id: AttackId,
        trail: ProjectileTrail,
        start_xy: [f32; 2],
        end_xy: [f32; 2],
        moved_distance: f32,
        dt_secs: f32,
    },
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

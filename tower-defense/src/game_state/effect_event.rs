use crate::MapCoordF32;
use crate::game_state::ProjectileTrail;
use crate::game_state::field_particle::*;
use crate::sound;
use crate::sound::EmitSoundParams;
use namui::*;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub(crate) static PROJECTILE_TRAIL_SOUND_IDS: LazyLock<
    Mutex<HashMap<u64, (ProjectileTrail, sound::SoundId)>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Default, Clone)]
pub(crate) struct ProjectileTrailEffectState {
    pub trail_distance_remainder: f32,
    pub whoosh_cooldown_secs: f32,
}

pub(crate) static PROJECTILE_TRAIL_EFFECT_STATE: LazyLock<
    Mutex<HashMap<u64, ProjectileTrailEffectState>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
pub enum ParticleSpawnRequest {
    DamageText(DamageTextParticle),
    Projectile(ProjectileParticle),
    Trash(TrashParticle),
    MonsterSoul(MonsterSoulParticle),
    MonsterCorpse(MonsterCorpseParticle),
    Card(CardParticle),
    Icon(IconParticle),
    Heart(HeartParticle),
    BlackSmoke(BlackSmokeParticle),
    Dust(DustParticle),
    Attack(AttackParticle),
}

#[derive(Clone)]
pub enum GameEffectEvent {
    SpawnParticle(ParticleSpawnRequest),
    PlaySound(EmitSoundParams),
    PlaySoundDelayed(EmitSoundParams, Duration),
    SpawnProjectileTrail {
        trail: ProjectileTrail,
        start_xy: MapCoordF32,
        end_xy: MapCoordF32,
        count: usize,
        now: Instant,
    },
    SpawnProjectileHitEffect(
        crate::game_state::attack::ProjectileHitEffect,
        MapCoordF32,
        Instant,
    ),
    SpawnLaserBeam((f32, f32), (f32, f32), Instant),
    SpawnTowerRemoveDustBurst((f32, f32), Instant),
    SyncProjectileTrailState {
        projectile_id: u64,
        trail: ProjectileTrail,
        start_xy: MapCoordF32,
        end_xy: MapCoordF32,
        moved_distance: f32,
        dt_secs: f32,
        now: Instant,
    },
}

#[derive(Clone, Default)]
pub struct EffectEventQueue {
    pub events: Vec<GameEffectEvent>,
}

impl EffectEventQueue {
    pub fn push(&mut self, event: GameEffectEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, GameEffectEvent> {
        self.events.drain(..)
    }
}

impl bincode::Encode for EffectEventQueue {
    fn encode<__E: bincode::enc::Encoder>(
        &self,
        _encoder: &mut __E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        Ok(())
    }
}

impl bincode::Decode<()> for EffectEventQueue {
    fn decode<__D: bincode::de::Decoder<Context = ()>>(
        _decoder: &mut __D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        Ok(Self::default())
    }
}

impl Serialize for EffectEventQueue {
    fn serialize(&self, _buf: &mut Vec<u8>) {}
    fn serialize_without_name(&self, _buf: &mut Vec<u8>) {}
}

impl Deserialize for EffectEventQueue {
    fn deserialize(_buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(Self::default())
    }
    fn deserialize_without_name(_buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(Self::default())
    }
}

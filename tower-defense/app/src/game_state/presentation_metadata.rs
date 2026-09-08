use crate::game_state::attack::ProjectileHitEffect;
use crate::game_state::projectile::ProjectileKind;
use crate::game_state::tower::{Animation, AnimationKind, RoyalStraightFlushVisual};
use crate::{Angle, AttackId, MonsterId, TowerId};
use namui::*;

#[derive(Clone, State, Default)]
pub(crate) struct PresentationMetadataStore {
    /// Headed-only animation and effect metadata keyed by authoritative IDs.
    ///
    /// Entity positions, health, status effects, and attack state are never
    /// read from these caches; those values come from `td_core`.
    pub(crate) monsters: Vec<MonsterPresentationCache>,
    pub(crate) projectiles: Vec<ProjectilePresentationCache>,
    pub(crate) towers: Vec<TowerPresentationCache>,
}

/// Presentation-only animation state retained across raw-core refreshes.
#[derive(Clone, State)]
pub(crate) struct MonsterPresentationCache {
    pub(crate) id: MonsterId,
    pub(crate) rotation: Angle,
    pub(crate) y_offset: f32,
}

/// Presentation-only projectile asset and effect metadata.
#[derive(Clone, State)]
pub(crate) struct ProjectilePresentationCache {
    pub(crate) id: AttackId,
    pub(crate) projectile_kind: ProjectileKind,
    pub(crate) trail: crate::game_state::projectile::ProjectileTrail,
    pub(crate) hit_effect: ProjectileHitEffect,
}

/// Presentation-only tower animation state retained across raw-core refreshes.
#[derive(Clone)]
pub(crate) struct TowerPresentationCache {
    pub(crate) id: TowerId,
    pub(crate) animation_kind: crate::game_state::tower::AnimationKind,
    pub(crate) y_ratio_offset: f32,
    pub(crate) animation: Animation,
    pub(crate) royal_straight_flush_visual: Option<RoyalStraightFlushVisual>,
}

impl namui::bincode::Encode for TowerPresentationCache {
    fn encode<__E: namui::bincode::enc::Encoder>(
        &self,
        encoder: &mut __E,
    ) -> Result<(), namui::bincode::error::EncodeError> {
        self.id.encode(encoder)?;
        self.animation_kind.encode(encoder)?;
        self.y_ratio_offset.encode(encoder)
    }
}

impl namui::bincode::Decode<()> for TowerPresentationCache {
    fn decode<__D: namui::bincode::de::Decoder<Context = ()>>(
        decoder: &mut __D,
    ) -> Result<Self, namui::bincode::error::DecodeError> {
        let id = TowerId::decode(decoder)?;
        let animation_kind = AnimationKind::decode(decoder)?;
        let y_ratio_offset = f32::decode(decoder)?;
        Ok(Self {
            id,
            animation_kind,
            y_ratio_offset,
            animation: Animation::default(),
            royal_straight_flush_visual: None,
        })
    }
}

impl namui::Serialize for TowerPresentationCache {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }

    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.write_string("id");
        self.id.serialize_without_name(buf);
        buf.write_string("animation_kind");
        self.animation_kind.serialize_without_name(buf);
        buf.write_string("y_ratio_offset");
        self.y_ratio_offset.serialize_without_name(buf);
    }
}

impl namui::Deserialize for TowerPresentationCache {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Self::deserialize_without_name(buf)
    }

    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, namui::DeserializeError> {
        buf.read_name("id")?;
        let id = TowerId::deserialize_without_name(buf)?;
        buf.read_name("animation_kind")?;
        let animation_kind = AnimationKind::deserialize_without_name(buf)?;
        buf.read_name("y_ratio_offset")?;
        let y_ratio_offset = f32::deserialize_without_name(buf)?;
        Ok(Self {
            id,
            animation_kind,
            y_ratio_offset,
            animation: Animation::default(),
            royal_straight_flush_visual: None,
        })
    }
}

impl PresentationMetadataStore {
    /// Refresh caches from the legacy projection during migration/reload.
    ///
    /// Runtime entity data is still read from `td_core::CoreState`; this
    /// method copies only headed presentation values.
    pub(crate) fn refresh_from_core(&mut self, raw: &td_core::CoreState) {
        let previous_towers = std::mem::take(&mut self.towers);
        let previous_monsters = std::mem::take(&mut self.monsters);
        let previous_projectiles = std::mem::take(&mut self.projectiles);
        self.monsters = raw
            .monsters()
            .iter()
            .map(|monster| {
                let id = MonsterId::from_raw(monster.id);
                previous_monsters
                    .iter()
                    .find(|cached| cached.id == id)
                    .cloned()
                    .unwrap_or(MonsterPresentationCache {
                        id,
                        rotation: 0.0.deg(),
                        y_offset: 0.0,
                    })
            })
            .collect();
        let active_attack_ids = raw
            .in_flight_attacks()
            .iter()
            .map(|attack| AttackId::from_raw(attack.id))
            .collect::<std::collections::HashSet<_>>();
        self.projectiles = previous_projectiles
            .into_iter()
            .filter(|projectile| active_attack_ids.contains(&projectile.id))
            .collect();
        self.towers = raw
            .towers()
            .iter()
            .filter_map(|tower| {
                let id = TowerId::from_raw(tower.id?);
                if let Some(mut cached) = previous_towers
                    .iter()
                    .find(|cached| cached.id == id)
                    .cloned()
                {
                    cached.animation_kind = cached.animation.kind;
                    cached.y_ratio_offset = cached.animation.y_ratio_offset;
                    return Some(cached);
                }
                Some(TowerPresentationCache {
                    id,
                    animation_kind: AnimationKind::Idle1,
                    y_ratio_offset: 0.0,
                    animation: Animation::new(crate::SimTick::from_ticks(raw.sim_tick().ticks())),
                    royal_straight_flush_visual: None,
                })
            })
            .collect();
    }

    pub(crate) fn insert_projectile(
        &mut self,
        id: AttackId,
        projectile_kind: ProjectileKind,
        trail: crate::game_state::projectile::ProjectileTrail,
        hit_effect: ProjectileHitEffect,
    ) {
        self.projectiles.retain(|projectile| projectile.id != id);
        self.projectiles.push(ProjectilePresentationCache {
            id,
            projectile_kind,
            trail,
            hit_effect,
        });
    }

    pub(crate) fn projectile(&self, id: AttackId) -> Option<&ProjectilePresentationCache> {
        self.projectiles
            .iter()
            .find(|projectile| projectile.id == id)
    }

    pub(crate) fn remove_projectile(&mut self, id: AttackId) {
        self.projectiles.retain(|projectile| projectile.id != id);
    }

    pub(crate) fn tower(&self, id: TowerId) -> Option<&TowerPresentationCache> {
        self.towers.iter().find(|tower| tower.id == id)
    }

    pub(crate) fn tower_mut(&mut self, id: TowerId) -> Option<&mut TowerPresentationCache> {
        self.towers.iter_mut().find(|tower| tower.id == id)
    }

    pub(crate) fn transition_tower_animation(
        &mut self,
        id: TowerId,
        kind: AnimationKind,
        sim_tick: crate::SimTick,
    ) {
        if let Some(tower) = self.tower_mut(id) {
            tower.animation.transition(kind, sim_tick);
            tower.animation_kind = tower.animation.kind;
            tower.y_ratio_offset = tower.animation.y_ratio_offset;
        }
    }

    pub(crate) fn rehydrate_runtime_state(&mut self, sim_tick: crate::SimTick) {
        for tower in &mut self.towers {
            tower.animation = Animation::new(sim_tick);
            tower.animation.kind = tower.animation_kind;
            tower.animation.y_ratio_offset = tower.y_ratio_offset;
        }
    }
}

use crate::game_state::attack::{InFlightAttackKind, SpatialAttack};
use crate::game_state::monster::Monster;
use crate::{AttackId, MapCoordF32, MonsterId, SimRenderTime, TowerId, WorldCoord, WorldVec};
use namui::*;

#[derive(Clone, State)]
pub(crate) struct WorldRenderSnapshot {
    pub(crate) sim_tick: crate::SimTick,
    monsters: Vec<MonsterRenderSnapshot>,
    spatial_projectiles: Vec<SpatialProjectileRenderSnapshot>,
    towers: Vec<TowerRenderSnapshot>,
    enemy_base_scale: Xy<f32>,
    player_base_scale: Xy<f32>,
}

#[derive(Clone, State)]
pub(crate) struct MonsterRenderSnapshot {
    pub(crate) id: MonsterId,
    pub(crate) position: WorldCoord,
    pub(crate) direction: WorldVec,
    pub(crate) motion_revision: u64,
    pub(crate) kind: crate::game_state::MonsterKind,
    pub(crate) hp: crate::Health,
    pub(crate) max_hp: crate::Health,
    pub(crate) rotation: Angle,
    pub(crate) y_offset: f32,
}

#[derive(Clone, State)]
pub(crate) struct SpatialProjectileRenderSnapshot {
    pub(crate) id: AttackId,
    pub(crate) position: WorldCoord,
    pub(crate) direction: WorldVec,
    pub(crate) projectile_kind: crate::game_state::projectile::ProjectileKind,
}

#[derive(Clone, State)]
pub(crate) struct TowerRenderSnapshot {
    pub(crate) id: TowerId,
    pub(crate) animation_kind: crate::game_state::tower::AnimationKind,
    pub(crate) y_ratio_offset: f32,
}

impl WorldRenderSnapshot {
    #[cfg(test)]
    pub(crate) fn empty(sim_tick: crate::SimTick) -> Self {
        Self {
            sim_tick,
            monsters: Vec::new(),
            spatial_projectiles: Vec::new(),
            towers: Vec::new(),
            enemy_base_scale: Xy::new(1.0, 1.0),
            player_base_scale: Xy::new(1.0, 1.0),
        }
    }

    pub(crate) fn capture(game_state: &crate::game_state::GameState) -> Self {
        let mut monsters = game_state
            .monsters
            .iter()
            .map(MonsterRenderSnapshot::from_monster)
            .collect::<Vec<_>>();
        monsters.sort_by_key(|monster| monster.id);

        let mut spatial_projectiles = game_state
            .in_flight_attacks
            .iter()
            .filter_map(|attack| {
                let InFlightAttackKind::Spatial(spatial) = &attack.kind else {
                    return None;
                };
                Some(SpatialProjectileRenderSnapshot::from_attack(
                    attack.id, spatial,
                ))
            })
            .collect::<Vec<_>>();
        spatial_projectiles.sort_by_key(|projectile| projectile.id);

        let mut towers = game_state
            .towers
            .iter()
            .map(|tower| {
                let (animation_kind, y_ratio_offset) = tower.render_animation_state();
                TowerRenderSnapshot {
                    id: tower.id(),
                    animation_kind,
                    y_ratio_offset,
                }
            })
            .collect::<Vec<_>>();
        towers.sort_by_key(|tower| tower.id);
        let (enemy_base_scale, player_base_scale) = game_state.render_base_scales();

        Self {
            sim_tick: game_state.sim_tick(),
            monsters,
            spatial_projectiles,
            towers,
            enemy_base_scale,
            player_base_scale,
        }
    }

    fn monster(&self, id: MonsterId) -> Option<&MonsterRenderSnapshot> {
        self.monsters
            .binary_search_by_key(&id, |monster| monster.id)
            .ok()
            .map(|index| &self.monsters[index])
    }

    fn spatial_projectile(&self, id: AttackId) -> Option<&SpatialProjectileRenderSnapshot> {
        self.spatial_projectiles
            .binary_search_by_key(&id, |projectile| projectile.id)
            .ok()
            .map(|index| &self.spatial_projectiles[index])
    }

    fn tower(&self, id: TowerId) -> Option<&TowerRenderSnapshot> {
        self.towers
            .binary_search_by_key(&id, |tower| tower.id)
            .ok()
            .map(|index| &self.towers[index])
    }

    pub(crate) fn monsters(&self) -> &[MonsterRenderSnapshot] {
        &self.monsters
    }

    pub(crate) fn spatial_projectiles(&self) -> &[SpatialProjectileRenderSnapshot] {
        &self.spatial_projectiles
    }
}

impl MonsterRenderSnapshot {
    fn from_monster(monster: &Monster) -> Self {
        Self {
            id: monster.id(),
            position: monster.world_xy(),
            direction: monster.move_on_route.direction(),
            motion_revision: monster.move_on_route.motion_revision(),
            kind: monster.kind,
            hp: monster.hp,
            max_hp: monster.max_hp,
            rotation: monster.animation.rotation,
            y_offset: monster.animation.y_offset,
        }
    }
}

impl SpatialProjectileRenderSnapshot {
    fn from_attack(id: AttackId, spatial: &SpatialAttack) -> Self {
        Self {
            id,
            position: spatial.xy,
            direction: spatial.velocity,
            projectile_kind: spatial.projectile_kind,
        }
    }
}

#[derive(Clone, Default, State)]
pub(crate) struct RenderSnapshotHistory {
    previous: Option<WorldRenderSnapshot>,
    current: Option<WorldRenderSnapshot>,
}

impl RenderSnapshotHistory {
    pub(crate) fn rebase(&mut self, snapshot: WorldRenderSnapshot) {
        self.previous = Some(snapshot.clone());
        self.current = Some(snapshot);
    }

    pub(crate) fn commit(&mut self, snapshot: WorldRenderSnapshot) {
        if let Some(current) = self.current.take() {
            self.previous = Some(current);
        } else {
            self.previous = Some(snapshot.clone());
        }
        self.current = Some(snapshot);
    }

    pub(crate) fn is_initialized(&self) -> bool {
        self.current.is_some()
    }

    pub(crate) fn current_tick(&self) -> Option<crate::SimTick> {
        self.current.as_ref().map(|snapshot| snapshot.sim_tick)
    }

    pub(crate) fn previous_tick(&self) -> Option<crate::SimTick> {
        self.previous.as_ref().map(|snapshot| snapshot.sim_tick)
    }

    pub(crate) fn current(&self) -> Option<&WorldRenderSnapshot> {
        self.current.as_ref()
    }

    pub(crate) fn sample_monster(
        &self,
        id: MonsterId,
        render_time: SimRenderTime,
        interpolate: bool,
    ) -> Option<MonsterRenderSample<'_>> {
        let current = self.current.as_ref()?.monster(id)?;
        let previous = self
            .previous
            .as_ref()
            .and_then(|snapshot| snapshot.monster(id));
        let can_interpolate = interpolate
            && previous.is_some_and(|previous| {
                previous.motion_revision == current.motion_revision && previous.kind == current.kind
            });
        let position = if can_interpolate {
            lerp_world_coord(
                previous.expect("checked above").position,
                current.position,
                render_time.alpha.as_f32(),
            )
        } else {
            current.position.as_map_coord_f32()
        };
        let direction = if can_interpolate {
            lerp_world_vec(
                previous.expect("checked above").direction,
                current.direction,
                render_time.alpha.as_f32(),
            )
        } else {
            world_vec_as_xy(current.direction)
        };
        let (rotation, y_offset) = if can_interpolate {
            let previous = previous.expect("checked above");
            (
                lerp_angle(
                    previous.rotation,
                    current.rotation,
                    render_time.alpha.as_f32(),
                ),
                previous.y_offset
                    + (current.y_offset - previous.y_offset) * render_time.alpha.as_f32(),
            )
        } else {
            (current.rotation, current.y_offset)
        };
        Some(MonsterRenderSample {
            current,
            position,
            direction,
            rotation,
            y_offset,
        })
    }

    pub(crate) fn sample_projectile(
        &self,
        id: AttackId,
        render_time: SimRenderTime,
        interpolate: bool,
    ) -> Option<SpatialProjectileRenderSample<'_>> {
        let current = self.current.as_ref()?.spatial_projectile(id)?;
        let previous = self
            .previous
            .as_ref()
            .and_then(|snapshot| snapshot.spatial_projectile(id));
        let can_interpolate = interpolate && previous.is_some();
        let position = if can_interpolate {
            lerp_world_coord(
                previous.expect("checked above").position,
                current.position,
                render_time.alpha.as_f32(),
            )
        } else {
            current.position.as_map_coord_f32()
        };
        let direction = if can_interpolate {
            lerp_world_vec(
                previous.expect("checked above").direction,
                current.direction,
                render_time.alpha.as_f32(),
            )
        } else {
            world_vec_as_xy(current.direction)
        };
        Some(SpatialProjectileRenderSample {
            current,
            position,
            direction,
        })
    }

    pub(crate) fn sample_tower_at(
        &self,
        id: TowerId,
        alpha: f32,
        interpolate: bool,
    ) -> Option<f32> {
        let current = self.current.as_ref()?.tower(id)?;
        let previous = self
            .previous
            .as_ref()
            .and_then(|snapshot| snapshot.tower(id));
        let can_interpolate = interpolate
            && previous.is_some_and(|previous| previous.animation_kind == current.animation_kind);
        let y_ratio_offset = if can_interpolate {
            let previous = previous.expect("checked above");
            previous.y_ratio_offset
                + (current.y_ratio_offset - previous.y_ratio_offset) * alpha.clamp(0.0, 1.0)
        } else {
            current.y_ratio_offset
        };
        Some(y_ratio_offset)
    }

    pub(crate) fn base_scales_at(
        &self,
        alpha: f32,
        interpolate: bool,
    ) -> Option<(Xy<f32>, Xy<f32>)> {
        let current = self.current.as_ref()?;
        let previous = self.previous.as_ref();
        if !interpolate {
            return Some((current.enemy_base_scale, current.player_base_scale));
        }
        let previous = previous?;
        Some((
            lerp_xy(previous.enemy_base_scale, current.enemy_base_scale, alpha),
            lerp_xy(previous.player_base_scale, current.player_base_scale, alpha),
        ))
    }
}

pub(crate) struct MonsterRenderSample<'a> {
    pub(crate) current: &'a MonsterRenderSnapshot,
    pub(crate) position: MapCoordF32,
    #[allow(dead_code)]
    pub(crate) direction: Xy<f32>,
    pub(crate) rotation: Angle,
    pub(crate) y_offset: f32,
}

pub(crate) struct SpatialProjectileRenderSample<'a> {
    pub(crate) current: &'a SpatialProjectileRenderSnapshot,
    pub(crate) position: MapCoordF32,
    pub(crate) direction: Xy<f32>,
}

fn lerp_world_coord(previous: WorldCoord, current: WorldCoord, alpha: f32) -> MapCoordF32 {
    let alpha = alpha.clamp(0.0, 1.0) as f64;
    let units = crate::world::WORLD_UNITS_PER_TILE as f64;
    MapCoordF32::new(
        ((previous.x as f64 + (current.x - previous.x) as f64 * alpha) / units) as f32,
        ((previous.y as f64 + (current.y - previous.y) as f64 * alpha) / units) as f32,
    )
}

fn lerp_world_vec(previous: WorldVec, current: WorldVec, alpha: f32) -> Xy<f32> {
    let alpha = alpha.clamp(0.0, 1.0) as f64;
    Xy::new(
        (previous.x as f64 + (current.x - previous.x) as f64 * alpha) as f32,
        (previous.y as f64 + (current.y - previous.y) as f64 * alpha) as f32,
    )
}

fn world_vec_as_xy(value: WorldVec) -> Xy<f32> {
    Xy::new(value.x as f32, value.y as f32)
}

fn lerp_angle(previous: Angle, current: Angle, alpha: f32) -> Angle {
    let tau = std::f32::consts::TAU;
    let mut delta = (current.as_radians() - previous.as_radians()).rem_euclid(tau);
    if delta > std::f32::consts::PI {
        delta -= tau;
    }
    (previous.as_radians() + delta * alpha.clamp(0.0, 1.0)).rad()
}

fn lerp_xy(previous: Xy<f32>, current: Xy<f32>, alpha: f32) -> Xy<f32> {
    let alpha = alpha.clamp(0.0, 1.0);
    Xy::new(
        previous.x + (current.x - previous.x) * alpha,
        previous.y + (current.y - previous.y) * alpha,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Health, InterpolationAlpha, SimTick};

    fn monster(id: u64, x: i64, motion_revision: u64) -> MonsterRenderSnapshot {
        MonsterRenderSnapshot {
            id: MonsterId::from_raw(id),
            position: WorldCoord::new(x, 0),
            direction: WorldVec::new(1, 0),
            motion_revision,
            kind: crate::game_state::MonsterKind::Mob01,
            hp: Health::from_integer(1),
            max_hp: Health::from_integer(1),
            rotation: 0.0.deg(),
            y_offset: 0.0,
        }
    }

    fn snapshot(tick: u64, monsters: Vec<MonsterRenderSnapshot>) -> WorldRenderSnapshot {
        let mut snapshot = WorldRenderSnapshot::empty(SimTick::from_ticks(tick));
        snapshot.monsters = monsters;
        snapshot
    }

    fn projectile(id: u64, x: i64, direction: WorldVec) -> SpatialProjectileRenderSnapshot {
        SpatialProjectileRenderSnapshot {
            id: AttackId::from_raw(id),
            position: WorldCoord::new(x, 0),
            direction,
            projectile_kind: crate::game_state::projectile::ProjectileKind::Cards00,
        }
    }

    #[test]
    fn world_coordinate_interpolation_stays_in_the_segment() {
        let previous = WorldCoord::from_tile(1, 2);
        let current = WorldCoord::from_tile(3, 6);
        let midpoint = lerp_world_coord(previous, current, 0.5);
        assert_eq!(midpoint.x, 2.0);
        assert_eq!(midpoint.y, 4.0);
    }

    #[test]
    fn history_rebase_and_commit_keep_two_fixed_ticks() {
        let empty = snapshot(0, Vec::new());
        let next = snapshot(1, Vec::new());
        let mut history = RenderSnapshotHistory::default();
        history.rebase(empty);
        assert_eq!(history.current_tick(), Some(SimTick::ZERO));
        history.commit(next);
        assert_eq!(history.previous.as_ref().unwrap().sim_tick, SimTick::ZERO);
        assert_eq!(history.current_tick(), Some(SimTick::from_ticks(1)));
        let _ = InterpolationAlpha::ZERO;
    }

    #[test]
    fn spawn_is_snapped_and_despawn_is_not_rendered() {
        let mut history = RenderSnapshotHistory::default();
        history.rebase(snapshot(0, vec![monster(1, 0, 0)]));
        history.commit(snapshot(1, vec![monster(1, 10, 0), monster(2, 20, 0)]));

        let time = SimRenderTime::new(SimTick::ZERO, InterpolationAlpha::from_f32(0.5));
        let existing = history
            .sample_monster(MonsterId::from_raw(1), time, true)
            .unwrap();
        assert_eq!(existing.position.x, 0.000005);
        let spawned = history
            .sample_monster(MonsterId::from_raw(2), time, true)
            .unwrap();
        assert_eq!(spawned.position.x, 0.00002);

        let mut despawned_history = RenderSnapshotHistory::default();
        despawned_history.rebase(snapshot(0, vec![monster(3, 30, 0)]));
        despawned_history.commit(snapshot(1, Vec::new()));
        assert!(
            despawned_history
                .sample_monster(MonsterId::from_raw(3), time, true)
                .is_none()
        );
    }

    #[test]
    fn route_reset_or_teleport_revision_snaps_current_position() {
        let mut history = RenderSnapshotHistory::default();
        history.rebase(snapshot(0, vec![monster(1, 1_000_000, 0)]));
        history.commit(snapshot(1, vec![monster(1, 0, 1)]));
        let sample = history
            .sample_monster(
                MonsterId::from_raw(1),
                SimRenderTime::new(SimTick::ZERO, InterpolationAlpha::from_f32(0.5)),
                true,
            )
            .unwrap();
        assert_eq!(sample.position.x, 0.0);
    }

    #[test]
    fn projectile_spawn_despawn_and_direction_are_sampled_by_id() {
        let mut history = RenderSnapshotHistory::default();
        let mut previous = WorldRenderSnapshot::empty(SimTick::ZERO);
        previous.spatial_projectiles = vec![projectile(1, 0, WorldVec::new(0, 10))];
        history.rebase(previous);
        let mut current = WorldRenderSnapshot::empty(SimTick::from_ticks(1));
        current.spatial_projectiles = vec![
            projectile(1, 1_000_000, WorldVec::new(10, 0)),
            projectile(2, 2_000_000, WorldVec::new(0, 20)),
        ];
        history.commit(current);

        let time = SimRenderTime::new(SimTick::ZERO, InterpolationAlpha::from_f32(0.5));
        let existing = history
            .sample_projectile(AttackId::from_raw(1), time, true)
            .unwrap();
        assert_eq!(existing.position.x, 0.5);
        assert_eq!(existing.direction, Xy::new(5.0, 5.0));

        let spawned = history
            .sample_projectile(AttackId::from_raw(2), time, true)
            .unwrap();
        assert_eq!(spawned.position.x, 2.0);

        let mut despawned = WorldRenderSnapshot::empty(SimTick::from_ticks(2));
        despawned.spatial_projectiles = vec![projectile(2, 3_000_000, WorldVec::new(0, 20))];
        history.commit(despawned);
        assert!(
            history
                .sample_projectile(AttackId::from_raw(1), time, true)
                .is_none()
        );
    }
}

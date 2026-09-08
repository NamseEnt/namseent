use super::*;
use crate::{MapCoordF32, WorldCoord, WorldDistance, WorldSpeed};
use std::sync::Arc;

#[derive(State, Clone)]
pub struct MoveOnRoute {
    route: Arc<Route>,
    route_index: usize,
    route_progress: WorldDistance,
    map_coord: WorldCoord,
    velocity: WorldSpeed,
    movement_remainder: i64,
    motion_revision: u64,
}

pub type Velocity = WorldSpeed;

impl MoveOnRoute {
    pub fn new(route: Arc<Route>, velocity: Velocity) -> Self {
        Self {
            map_coord: route.world_coords[0],
            route,
            route_index: 0,
            route_progress: WorldDistance::ZERO,
            velocity,
            movement_remainder: 0,
            motion_revision: 0,
        }
    }
    pub fn is_finished(&self) -> bool {
        td_core::move_on_route_is_finished(&self.to_core_state())
    }
    pub fn world_xy(&self) -> WorldCoord {
        let [x, y] = td_core::move_on_route_position(&self.to_core_state());
        WorldCoord::new(x, y)
    }
    pub fn xy(&self) -> MapCoordF32 {
        self.map_coord.as_map_coord_f32()
    }
    pub fn velocity(&self) -> Velocity {
        WorldSpeed::from_raw(td_core::move_on_route_velocity_raw(&self.to_core_state()))
    }
    pub fn direction(&self) -> WorldVec {
        if self.is_finished() {
            return WorldVec::ZERO;
        }
        self.route.world_coords[self.route_index + 1] - self.map_coord
    }
    pub fn motion_revision(&self) -> u64 {
        td_core::move_on_route_motion_revision(&self.to_core_state())
    }

    pub fn to_core_state(&self) -> td_core::MoveOnRouteState {
        td_core::MoveOnRouteState {
            route: self.route.to_core_state(),
            route_index: self.route_index,
            route_progress_raw: self.route_progress.raw(),
            map_coord: [self.map_coord.x, self.map_coord.y],
            velocity_raw: self.velocity.raw(),
            movement_remainder: self.movement_remainder,
            motion_revision: self.motion_revision,
        }
    }

    pub fn from_core_state(state: td_core::MoveOnRouteState) -> Option<Self> {
        let route = Arc::new(Route::from_core_state(state.route)?);
        if state.route_index >= route.world_coords.len()
            || state.route_progress_raw < 0
            || state.velocity_raw < 0
        {
            return None;
        }
        Some(Self {
            route,
            route_index: state.route_index,
            route_progress: WorldDistance::from_raw(state.route_progress_raw),
            map_coord: WorldCoord::new(state.map_coord[0], state.map_coord[1]),
            velocity: WorldSpeed::from_raw(state.velocity_raw),
            movement_remainder: state.movement_remainder,
            motion_revision: state.motion_revision,
        })
    }

    pub fn route_index(&self) -> usize {
        td_core::move_on_route_index(&self.to_core_state())
    }
    pub fn route_progress(&self) -> WorldDistance {
        WorldDistance::from_raw(td_core::move_on_route_progress_raw(&self.to_core_state()))
    }
    pub fn reset(&mut self) {
        let mut state = self.to_core_state();
        td_core::reset_move_on_route(&mut state);
        *self = Self::from_core_state(state).expect("MoveOnRoute adapter state must remain valid");
    }
    #[cfg(test)]
    pub(crate) fn move_one_tick(&mut self, speed: WorldSpeed) {
        let mut state = self.to_core_state();
        td_core::advance_move_on_route(&mut state, speed.raw());
        *self = Self::from_core_state(state).expect("MoveOnRoute adapter state must remain valid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route(length: i64) -> Arc<Route> {
        let start = WorldCoord::ZERO;
        let end = WorldCoord::new(length, 0);
        Arc::new(Route {
            map_coords: vec![MapCoord::new(0, 0), MapCoord::new(1, 0)],
            world_coords: vec![start, end],
            segment_lengths: vec![WorldDistance::from_raw(length)],
            cumulative_lengths: vec![WorldDistance::ZERO, WorldDistance::from_raw(length)],
        })
    }

    #[test]
    fn endpoint_is_reached_without_overshoot() {
        let mut mover = MoveOnRoute::new(route(1_000_000), WorldSpeed::from_raw(120_000_000));
        mover.move_one_tick(WorldSpeed::from_raw(120_000_000));
        assert_eq!(mover.world_xy(), WorldCoord::new(1_000_000, 0));
        assert!(mover.is_finished());
    }

    #[test]
    fn reset_increments_motion_revision_for_render_snap() {
        let mut mover = MoveOnRoute::new(route(1_000_000), WorldSpeed::from_raw(1));
        assert_eq!(mover.motion_revision(), 0);
        mover.reset();
        assert_eq!(mover.motion_revision(), 1);
    }

    #[test]
    fn remainder_is_preserved_for_one_hundred_thousand_ticks() {
        let mut mover = MoveOnRoute::new(route(10_000_000), WorldSpeed::from_raw(1));
        for _ in 0..100_000 {
            mover.move_one_tick(WorldSpeed::from_raw(1));
        }
        assert_eq!(
            mover.world_xy().x,
            100_000 / crate::world::SIM_TICKS_PER_SECOND
        );
        assert_eq!(mover.route_progress().raw(), mover.world_xy().x);
    }

    #[test]
    fn move_on_route_state_round_trips_authoritative_motion() {
        let mut mover = MoveOnRoute::new(route(10_000_000), WorldSpeed::from_raw(61));
        mover.move_one_tick(WorldSpeed::from_raw(61));
        mover.motion_revision = 4;

        let restored =
            MoveOnRoute::from_core_state(mover.to_core_state()).expect("valid movement state");
        assert_eq!(restored.route_index(), mover.route_index());
        assert_eq!(restored.route_progress(), mover.route_progress());
        assert_eq!(restored.world_xy(), mover.world_xy());
        assert_eq!(restored.velocity(), mover.velocity());
        assert_eq!(restored.motion_revision(), mover.motion_revision());
    }

    #[test]
    fn move_on_route_state_rejects_negative_velocity() {
        let mut state = MoveOnRoute::new(route(1_000), WorldSpeed::from_raw(1)).to_core_state();
        state.velocity_raw = -1;
        assert!(MoveOnRoute::from_core_state(state).is_none());
    }
}

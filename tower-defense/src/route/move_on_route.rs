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
        self.route_index >= self.route.world_coords.len().saturating_sub(1)
    }
    pub fn world_xy(&self) -> WorldCoord {
        self.map_coord
    }
    pub fn xy(&self) -> MapCoordF32 {
        self.map_coord.as_map_coord_f32()
    }
    pub fn velocity(&self) -> Velocity {
        self.velocity
    }
    pub fn direction(&self) -> WorldVec {
        if self.is_finished() {
            return WorldVec::ZERO;
        }
        self.route.world_coords[self.route_index + 1] - self.map_coord
    }
    pub fn motion_revision(&self) -> u64 {
        self.motion_revision
    }

    pub(crate) fn movement_remainder(&self) -> i64 {
        self.movement_remainder
    }
    pub fn route_index(&self) -> usize {
        self.route_index
    }
    pub fn route_progress(&self) -> WorldDistance {
        self.route_progress
    }
    pub fn reset(&mut self) {
        self.route_index = 0;
        self.route_progress = WorldDistance::ZERO;
        self.map_coord = self.route.world_coords[0];
        self.motion_revision = self.motion_revision.saturating_add(1);
    }
    pub(crate) fn move_one_tick(&mut self, speed: WorldSpeed) {
        let numerator = speed.raw() as i128 + self.movement_remainder as i128;
        let movable_distance = (numerator / crate::world::SIM_TICKS_PER_SECOND as i128)
            .clamp(0, i64::MAX as i128) as i64;
        self.movement_remainder = (numerator % crate::world::SIM_TICKS_PER_SECOND as i128) as i64;
        let mut movable_distance = movable_distance;
        while movable_distance > 0 && !self.is_finished() {
            let segment_start = self.route.world_coords[self.route_index];
            let segment_end = self.route.world_coords[self.route_index + 1];
            let segment_length = self.route.segment_lengths[self.route_index].raw();
            let travelled = self
                .route_progress
                .raw()
                .saturating_sub(self.route.cumulative_lengths[self.route_index].raw());
            let left = segment_length.saturating_sub(travelled);
            if movable_distance < left {
                let next_travelled = travelled.saturating_add(movable_distance);
                self.route_progress = WorldDistance::from_raw(
                    self.route.cumulative_lengths[self.route_index]
                        .raw()
                        .saturating_add(next_travelled),
                );
                self.map_coord = segment_start
                    + (segment_end - segment_start).scaled_by_distance(
                        WorldDistance::from_raw(next_travelled),
                        WorldDistance::from_raw(segment_length),
                    );
                return;
            }
            movable_distance = movable_distance.saturating_sub(left);
            self.route_index += 1;
            self.route_progress = self.route.cumulative_lengths[self.route_index];
            self.map_coord = segment_end;
        }
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
}

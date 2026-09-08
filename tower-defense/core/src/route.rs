use super::{RATIO_SCALE, SIM_TICKS_PER_SECOND};

use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RouteState {
    pub map_coords: Vec<[usize; 2]>,
    pub world_coords: Vec<[i64; 2]>,
    pub segment_lengths: Vec<i64>,
    pub cumulative_lengths: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MoveOnRouteState {
    pub route: RouteState,
    pub route_index: usize,
    pub route_progress_raw: i64,
    pub map_coord: [i64; 2],
    pub velocity_raw: i64,
    pub movement_remainder: i64,
    pub motion_revision: u64,
}

pub fn advance_move_on_route(state: &mut MoveOnRouteState, speed_raw: i64) {
    let numerator = speed_raw.max(0) as i128 + state.movement_remainder as i128;
    let movable_distance =
        (numerator / SIM_TICKS_PER_SECOND as i128).clamp(0, i64::MAX as i128) as i64;
    state.movement_remainder = (numerator % SIM_TICKS_PER_SECOND as i128) as i64;
    let mut movable_distance = movable_distance;

    while movable_distance > 0
        && state.route_index < state.route.world_coords.len().saturating_sub(1)
    {
        let segment_start = state.route.world_coords[state.route_index];
        let segment_end = state.route.world_coords[state.route_index + 1];
        let segment_length = state.route.segment_lengths[state.route_index];
        let travelled = state
            .route_progress_raw
            .saturating_sub(state.route.cumulative_lengths[state.route_index]);
        let left = segment_length.saturating_sub(travelled);

        if movable_distance < left {
            let next_travelled = travelled.saturating_add(movable_distance);
            state.route_progress_raw =
                state.route.cumulative_lengths[state.route_index].saturating_add(next_travelled);
            let denominator = segment_length.max(1) as i128;
            let interpolate = |start: i64, end: i64| {
                ((end as i128 - start as i128).saturating_mul(next_travelled as i128) / denominator)
                    .saturating_add(start as i128)
                    .clamp(i64::MIN as i128, i64::MAX as i128) as i64
            };
            state.map_coord = [
                interpolate(segment_start[0], segment_end[0]),
                interpolate(segment_start[1], segment_end[1]),
            ];
            return;
        }

        movable_distance = movable_distance.saturating_sub(left);
        state.route_index += 1;
        state.route_progress_raw = state.route.cumulative_lengths[state.route_index];
        state.map_coord = state.route.world_coords[state.route_index];
    }
}

pub(crate) fn multiply_ratio_raw(value: i64, multiplier: i64) -> i64 {
    (value as i128)
        .saturating_mul(multiplier as i128)
        .div_euclid(RATIO_SCALE as i128)
        .clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

pub fn apply_ratio_product_raw(amount: i64, factors: &[i64]) -> i64 {
    if amount <= 0 || factors.contains(&0) {
        return 0;
    }
    if factors.is_empty() {
        return amount;
    }

    let mut sorted_factors = factors.to_vec();
    sorted_factors.sort_unstable();
    let mut numerator = amount as i128;
    let mut denominator = 1_i128;
    for factor in sorted_factors {
        let gcd_left = gcd_i128(numerator, RATIO_SCALE as i128);
        numerator /= gcd_left;
        let scale = RATIO_SCALE as i128 / gcd_left;
        let gcd_right = gcd_i128(factor as i128, denominator);
        let factor = factor as i128 / gcd_right;
        denominator /= gcd_right;
        numerator = match numerator.checked_mul(factor) {
            Some(value) => value,
            None => return i64::MAX,
        };
        denominator = match denominator.checked_mul(scale) {
            Some(value) => value,
            None => return i64::MAX,
        };
    }

    div_round_positive(numerator, denominator).clamp(0, i64::MAX as i128) as i64
}

fn gcd_i128(mut left: i128, mut right: i128) -> i128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn div_round_positive(numerator: i128, denominator: i128) -> i128 {
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    if remainder.saturating_mul(2) >= denominator {
        quotient.saturating_add(1)
    } else {
        quotient
    }
}

pub fn move_on_route_is_finished(state: &MoveOnRouteState) -> bool {
    state.route_index >= state.route.world_coords.len().saturating_sub(1)
}

pub const fn move_on_route_index(state: &MoveOnRouteState) -> usize {
    state.route_index
}

pub const fn move_on_route_progress_raw(state: &MoveOnRouteState) -> i64 {
    state.route_progress_raw
}

pub const fn move_on_route_velocity_raw(state: &MoveOnRouteState) -> i64 {
    state.velocity_raw
}

pub const fn move_on_route_remainder(state: &MoveOnRouteState) -> i64 {
    state.movement_remainder
}

pub const fn move_on_route_motion_revision(state: &MoveOnRouteState) -> u64 {
    state.motion_revision
}

pub const fn move_on_route_position(state: &MoveOnRouteState) -> [i64; 2] {
    state.map_coord
}

pub fn reset_move_on_route(state: &mut MoveOnRouteState) {
    state.route_index = 0;
    state.route_progress_raw = 0;
    if let Some(start) = state.route.world_coords.first() {
        state.map_coord = *start;
    }
    state.motion_revision = state.motion_revision.saturating_add(1);
}

/// Shortest path on a grid using BFS.
///
/// Diagonal movement is blocked if both orthogonal neighbors are blockers.
/// Returns a path including both `start_xy` and `end_xy`.
pub fn find_shortest_route(
    wh: [usize; 2],
    start_xy: [usize; 2],
    end_xy: [usize; 2],
    blockers: &[[usize; 2]],
) -> Option<Vec<[usize; 2]>> {
    if end_xy == start_xy {
        return Some(vec![start_xy]);
    }
    if blockers.contains(&start_xy) || blockers.contains(&end_xy) {
        return None;
    }

    let mut map = RouteMap::new(wh, blockers);

    let mut queue = VecDeque::new();
    queue.push_back(start_xy);
    map.set_visit(start_xy, start_xy);

    while let Some(from_xy) = queue.pop_front() {
        for xy in neighbor_route(from_xy) {
            if map.cannot_visit(xy, from_xy) {
                continue;
            }
            queue.push_back(xy);
            map.set_visit(xy, from_xy);

            if xy == end_xy {
                return Some(map.gather_route(xy));
            }
        }
    }

    None
}

pub fn calculate_routes(
    blockers: &[[usize; 2]],
    travel_points: &[[usize; 2]],
    map_wh: [usize; 2],
) -> Option<RouteState> {
    let mut map_coords = Vec::new();
    for i in 0..travel_points.len().saturating_sub(1) {
        let start_xy = travel_points[i];
        let end_xy = travel_points[i + 1];
        let route = find_shortest_route(map_wh, start_xy, end_xy, blockers)?;
        map_coords.extend_from_slice(if i == 0 { &route } else { &route[1..] });
    }

    let world_coords = map_coords
        .iter()
        .map(|coord| {
            let x = (coord[0] as i64).saturating_mul(crate::WORLD_UNITS_PER_TILE);
            let y = (coord[1] as i64).saturating_mul(crate::WORLD_UNITS_PER_TILE);
            [x, y]
        })
        .collect::<Vec<_>>();
    let mut segment_lengths = Vec::new();
    let mut cumulative_lengths = vec![0];
    for pair in world_coords.windows(2) {
        let length = crate::vector_length_raw([
            pair[1][0].saturating_sub(pair[0][0]),
            pair[1][1].saturating_sub(pair[0][1]),
        ]);
        segment_lengths.push(length);
        cumulative_lengths.push(
            cumulative_lengths
                .last()
                .copied()
                .unwrap_or(0i64)
                .saturating_add(length),
        );
    }

    Some(RouteState {
        map_coords,
        world_coords,
        segment_lengths,
        cumulative_lengths,
    })
}

fn neighbor_route(last_xy: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    const DX_DY: [(isize, isize); 8] = [
        (0, -1),
        (-1, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    DX_DY.iter().filter_map(move |(dx, dy)| {
        let x = last_xy[0] as isize + dx;
        let y = last_xy[1] as isize + dy;
        if x >= 0 && y >= 0 {
            Some([x as usize, y as usize])
        } else {
            None
        }
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RouteBlock {
    Empty,
    Blocker,
    Visited { from_xy: [usize; 2] },
}

struct RouteMap {
    wh: [usize; 2],
    blocks: Vec<RouteBlock>,
}

impl RouteMap {
    fn new(wh: [usize; 2], blockers: &[[usize; 2]]) -> Self {
        let blocks = vec![RouteBlock::Empty; wh[0].saturating_mul(wh[1])];
        let mut this = Self { wh, blocks };
        for &blocker in blockers {
            *this.block_mut(blocker) = RouteBlock::Blocker;
        }
        this
    }

    fn block(&self, xy: [usize; 2]) -> RouteBlock {
        self.blocks[xy[1] * self.wh[0] + xy[0]]
    }

    fn block_mut(&mut self, xy: [usize; 2]) -> &mut RouteBlock {
        &mut self.blocks[xy[1] * self.wh[0] + xy[0]]
    }

    fn set_visit(&mut self, xy: [usize; 2], from_xy: [usize; 2]) {
        *self.block_mut(xy) = RouteBlock::Visited { from_xy };
    }

    fn cannot_visit(&self, xy: [usize; 2], from_xy: [usize; 2]) -> bool {
        if self.is_outside(xy) {
            return true;
        }
        match self.block(xy) {
            RouteBlock::Blocker | RouteBlock::Visited { .. } => true,
            RouteBlock::Empty => {
                !is_orthogonal(from_xy, xy) && self.blocked_on_diagonal(from_xy, xy)
            }
        }
    }

    fn is_outside(&self, xy: [usize; 2]) -> bool {
        xy[0] >= self.wh[0] || xy[1] >= self.wh[1]
    }

    fn gather_route(&self, end_xy: [usize; 2]) -> Vec<[usize; 2]> {
        let mut route = vec![end_xy];
        let mut xy = end_xy;
        loop {
            match self.block(xy) {
                RouteBlock::Visited { from_xy } => {
                    if xy == from_xy {
                        break;
                    }
                    route.push(from_xy);
                    xy = from_xy;
                }
                _ => unreachable!(),
            }
        }
        route.reverse();
        route
    }

    fn blocked_on_diagonal(&self, from_xy: [usize; 2], xy: [usize; 2]) -> bool {
        let a_xy = [from_xy[0], xy[1]];
        let b_xy = [xy[0], from_xy[1]];
        self.block(a_xy) == RouteBlock::Blocker && self.block(b_xy) == RouteBlock::Blocker
    }
}

fn is_orthogonal(from_xy: [usize; 2], xy: [usize; 2]) -> bool {
    from_xy[0] == xy[0] || from_xy[1] == xy[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_matches_root_bfs_reference() {
        let wh = [5, 5];
        let start = [2, 2];
        let end = [4, 4];
        let blockers = [[3, 2], [2, 3]];
        let route = find_shortest_route(wh, start, end, &blockers).unwrap();
        assert_eq!(route, vec![[2, 2], [3, 1], [4, 2], [4, 3], [4, 4]]);
    }

    #[test]
    fn route_world_coordinates_are_tile_scaled() {
        let routes = calculate_routes(&[], &[[0, 0], [1, 0]], [36, 36]).unwrap();
        assert_eq!(routes.map_coords, vec![[0, 0], [1, 0]]);
        assert_eq!(
            routes.world_coords,
            vec![[0, 0], [crate::WORLD_UNITS_PER_TILE, 0]]
        );
        assert_eq!(routes.segment_lengths, vec![crate::WORLD_UNITS_PER_TILE]);
        assert_eq!(
            routes.cumulative_lengths,
            vec![0, crate::WORLD_UNITS_PER_TILE]
        );
    }
}

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

    #[cfg(feature = "diagnostics")]
    crate::diagnostics::record(|counters| counters.shortest_route_bfs_calls += 1);
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

const WITNESS_ROUTES_PER_SEGMENT: usize = 3;

/// For each consecutive `travel_points` pair, finds up to
/// `WITNESS_ROUTES_PER_SEGMENT` witness routes and, for each, marks every
/// cell whose blocking could invalidate it: the route cells plus, for every
/// diagonal step, both orthogonal side cells, because a diagonal step is
/// only rejected when both of those are blockers. Each later witness avoids
/// the cells marked for the earlier ones (other than the pair itself), so a
/// footprint rarely touches all of them. Any extra blocker set that avoids
/// every cell marked for one witness leaves that witness valid, so the pair
/// stays connected without another search. `None` if a pair is already
/// disconnected.
pub(crate) fn route_dependency_grids(
    blockers: &[[usize; 2]],
    travel_points: &[[usize; 2]],
    map_wh: [usize; 2],
) -> Option<Vec<Vec<Vec<bool>>>> {
    travel_points
        .windows(2)
        .map(|points| {
            let mut witnesses = Vec::new();
            let mut avoided = blockers.to_vec();
            while witnesses.len() < WITNESS_ROUTES_PER_SEGMENT {
                let Some(route) = find_shortest_route(map_wh, points[0], points[1], &avoided)
                else {
                    break;
                };
                let mut grid = vec![false; map_wh[0].saturating_mul(map_wh[1])];
                let mut mark = |xy: [usize; 2]| {
                    if xy[0] < map_wh[0] && xy[1] < map_wh[1] {
                        grid[xy[1] * map_wh[0] + xy[0]] = true;
                    }
                };
                for &xy in &route {
                    mark(xy);
                }
                for step in route.windows(2) {
                    let [from_xy, to_xy] = [step[0], step[1]];
                    if !is_orthogonal(from_xy, to_xy) {
                        mark([from_xy[0], to_xy[1]]);
                        mark([to_xy[0], from_xy[1]]);
                    }
                }
                for y in 0..map_wh[1] {
                    for x in 0..map_wh[0] {
                        if grid[y * map_wh[0] + x] && [x, y] != points[0] && [x, y] != points[1] {
                            avoided.push([x, y]);
                        }
                    }
                }
                witnesses.push(grid);
            }
            (!witnesses.is_empty()).then_some(witnesses)
        })
        .collect()
}

pub(crate) const MAP_CELL_COUNT: usize = crate::MAP_SIZE[0] * crate::MAP_SIZE[1];

pub(crate) fn map_cell_index(xy: [usize; 2]) -> Option<usize> {
    (xy[0] < crate::MAP_SIZE[0] && xy[1] < crate::MAP_SIZE[1])
        .then(|| xy[1] * crate::MAP_SIZE[0] + xy[0])
}

/// Same reachability as [`path_exists_with_extra_blockers`] on the fixed
/// `MAP_SIZE` grid, with every blocker already marked in `blocked`, using
/// stack buffers instead of a freshly allocated route map.
pub(crate) fn grid_path_exists(
    blocked: &[bool; MAP_CELL_COUNT],
    start_xy: [usize; 2],
    end_xy: [usize; 2],
) -> bool {
    if end_xy == start_xy {
        return true;
    }
    let (Some(start), Some(end)) = (map_cell_index(start_xy), map_cell_index(end_xy)) else {
        return false;
    };
    if blocked[start] || blocked[end] {
        return false;
    }
    #[cfg(feature = "diagnostics")]
    crate::diagnostics::record(|counters| counters.placement_bfs_calls += 1);
    const WIDTH: isize = crate::MAP_SIZE[0] as isize;
    const HEIGHT: isize = crate::MAP_SIZE[1] as isize;
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
    let mut visited = [false; MAP_CELL_COUNT];
    let mut queue = [0u16; MAP_CELL_COUNT];
    let (mut head, mut tail) = (0usize, 1usize);
    visited[start] = true;
    queue[0] = start as u16;
    while head < tail {
        let cell = queue[head] as isize;
        head += 1;
        let (x, y) = (cell % WIDTH, cell / WIDTH);
        for (dx, dy) in DX_DY {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= WIDTH || ny >= HEIGHT {
                continue;
            }
            let next = (ny * WIDTH + nx) as usize;
            if blocked[next] || visited[next] {
                continue;
            }
            if dx != 0
                && dy != 0
                && blocked[(ny * WIDTH + x) as usize]
                && blocked[(y * WIDTH + nx) as usize]
            {
                continue;
            }
            if next == end {
                return true;
            }
            visited[next] = true;
            queue[tail] = next as u16;
            tail += 1;
        }
    }
    false
}

pub(crate) fn routes_exist_with_extra_blockers(
    blockers: &[[usize; 2]],
    extra_blockers: &[[usize; 2]],
    travel_points: &[[usize; 2]],
    map_wh: [usize; 2],
) -> bool {
    travel_points.windows(2).all(|points| {
        path_exists_with_extra_blockers(map_wh, points[0], points[1], blockers, extra_blockers)
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

pub(crate) fn path_exists_with_extra_blockers(
    wh: [usize; 2],
    start_xy: [usize; 2],
    end_xy: [usize; 2],
    blockers: &[[usize; 2]],
    extra_blockers: &[[usize; 2]],
) -> bool {
    if end_xy == start_xy {
        return true;
    }
    if blockers.contains(&start_xy)
        || blockers.contains(&end_xy)
        || extra_blockers.contains(&start_xy)
        || extra_blockers.contains(&end_xy)
    {
        return false;
    }

    #[cfg(feature = "diagnostics")]
    crate::diagnostics::record(|counters| counters.placement_bfs_calls += 1);
    let mut map = RouteMap::new(wh, blockers);
    for &blocker in extra_blockers {
        if !map.is_outside(blocker) {
            *map.block_mut(blocker) = RouteBlock::Blocker;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(start_xy);
    map.set_visit(start_xy, start_xy);

    while let Some(from_xy) = queue.pop_front() {
        for xy in neighbor_route(from_xy) {
            if map.cannot_visit(xy, from_xy) {
                continue;
            }
            if xy == end_xy {
                return true;
            }
            queue.push_back(xy);
            map.set_visit(xy, from_xy);
        }
    }

    false
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
    fn grid_path_exists_matches_route_map_search_on_random_blockers() {
        use rand::{Rng, SeedableRng};
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0x9a7d);
        let mut connected = 0usize;
        let mut disconnected = 0usize;
        for _ in 0..40000 {
            let density = rng.gen_range(0.0..0.55);
            let mut blocked = [false; MAP_CELL_COUNT];
            let mut blockers = Vec::new();
            for y in 0..crate::MAP_SIZE[1] {
                for x in 0..crate::MAP_SIZE[0] {
                    if rng.gen_bool(density) {
                        blocked[y * crate::MAP_SIZE[0] + x] = true;
                        blockers.push([x, y]);
                    }
                }
            }
            let start = [
                rng.gen_range(0..crate::MAP_SIZE[0]),
                rng.gen_range(0..crate::MAP_SIZE[1]),
            ];
            let end = [
                rng.gen_range(0..crate::MAP_SIZE[0]),
                rng.gen_range(0..crate::MAP_SIZE[1]),
            ];
            let expected =
                path_exists_with_extra_blockers(crate::MAP_SIZE, start, end, &blockers, &[]);
            assert_eq!(grid_path_exists(&blocked, start, end), expected);
            if expected {
                connected += 1;
            } else {
                disconnected += 1;
            }
        }
        assert!(connected > 500 && disconnected > 500);
    }

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

    #[test]
    fn route_existence_with_extra_blockers_matches_route_calculation() {
        let travel_points = [[0, 0], [4, 4]];
        let blockers = [[3, 3]];
        let extra_blockers = [[1, 1], [1, 2], [2, 1]];
        let mut combined = blockers.to_vec();
        combined.extend(extra_blockers);

        assert_eq!(
            routes_exist_with_extra_blockers(&blockers, &extra_blockers, &travel_points, [5, 5]),
            calculate_routes(&combined, &travel_points, [5, 5]).is_some()
        );

        let sealed_blockers = [[1, 0], [0, 1]];
        assert_eq!(
            routes_exist_with_extra_blockers(&sealed_blockers, &[], &[[0, 0], [4, 4]], [5, 5]),
            calculate_routes(&sealed_blockers, &[[0, 0], [4, 4]], [5, 5]).is_some()
        );
    }
}

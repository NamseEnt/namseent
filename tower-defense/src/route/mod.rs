mod move_on_route;
mod render_route;
mod render_route_flag;
mod route_find;

use crate::*;
pub use move_on_route::*;
use namui::State;
pub use render_route::*;
pub use render_route_flag::*;
pub use route_find::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, State)]
pub struct Route {
    map_coords: Vec<MapCoord>,
    pub(crate) world_coords: Vec<WorldCoord>,
    pub(crate) segment_lengths: Vec<WorldDistance>,
    pub(crate) cumulative_lengths: Vec<WorldDistance>,
}

impl Route {
    pub fn iter_coords(&self) -> &[MapCoord] {
        &self.map_coords
    }
}

pub fn calculate_routes(
    blockers: &[MapCoord],
    travel_points: &[MapCoord],
    map_wh: Wh<usize>,
) -> Option<Arc<Route>> {
    let mut map_coords = vec![];

    for i in 0..travel_points.len() - 1 {
        let start_xy = travel_points[i];
        let end_xy = travel_points[i + 1];
        let route = crate::route::find_shortest_route(map_wh, start_xy, end_xy, blockers)?;

        map_coords.extend_from_slice(if i == 0 { &route } else { &route[1..] });
    }

    let world_coords = map_coords
        .iter()
        .map(|coord| WorldCoord::from_tile(coord.x as i64, coord.y as i64))
        .collect::<Vec<_>>();
    let mut segment_lengths = Vec::new();
    let mut cumulative_lengths = vec![WorldDistance::ZERO];
    for pair in world_coords.windows(2) {
        let length = (pair[1] - pair[0]).length();
        segment_lengths.push(length);
        cumulative_lengths.push(WorldDistance::from_raw(
            cumulative_lengths
                .last()
                .unwrap()
                .raw()
                .saturating_add(length.raw()),
        ));
    }

    Some(Arc::new(Route {
        map_coords,
        world_coords,
        segment_lengths,
        cumulative_lengths,
    }))
}

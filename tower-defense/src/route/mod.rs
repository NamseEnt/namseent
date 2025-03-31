mod move_on_route;
mod route_find;

use crate::*;
pub use move_on_route::*;
pub use route_find::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Route {
    map_coords: Vec<MapCoord>,
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
        let Some(route) = crate::route::find_shortest_route(map_wh, start_xy, end_xy, blockers)
        else {
            return None;
        };
        map_coords.extend(route);
    }

    Some(Arc::new(Route { map_coords }))
}

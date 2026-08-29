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
#[cfg(any(test, feature = "debug-tools"))]
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

    pub fn to_core_state(&self) -> td_core::RouteState {
        td_core::RouteState {
            map_coords: self
                .map_coords
                .iter()
                .map(|coord| [coord.x, coord.y])
                .collect(),
            world_coords: self
                .world_coords
                .iter()
                .map(|coord| [coord.x, coord.y])
                .collect(),
            segment_lengths: self
                .segment_lengths
                .iter()
                .map(|length| length.raw())
                .collect(),
            cumulative_lengths: self
                .cumulative_lengths
                .iter()
                .map(|length| length.raw())
                .collect(),
        }
    }

    pub fn from_core_state(state: td_core::RouteState) -> Option<Self> {
        if state.world_coords.is_empty()
            || state.world_coords.len() != state.map_coords.len()
            || state.segment_lengths.len() + 1 != state.world_coords.len()
            || state.cumulative_lengths.len() != state.world_coords.len()
        {
            return None;
        }
        Some(Self {
            map_coords: state
                .map_coords
                .into_iter()
                .map(|[x, y]| MapCoord::new(x, y))
                .collect(),
            world_coords: state
                .world_coords
                .into_iter()
                .map(|[x, y]| WorldCoord::new(x, y))
                .collect(),
            segment_lengths: state
                .segment_lengths
                .into_iter()
                .map(WorldDistance::from_raw)
                .collect(),
            cumulative_lengths: state
                .cumulative_lengths
                .into_iter()
                .map(WorldDistance::from_raw)
                .collect(),
        })
    }
}

#[cfg(any(test, feature = "debug-tools"))]
pub fn calculate_routes(
    blockers: &[MapCoord],
    travel_points: &[MapCoord],
    map_wh: Wh<usize>,
) -> Option<Arc<Route>> {
    let blockers = blockers
        .iter()
        .map(|coord| [coord.x, coord.y])
        .collect::<Vec<_>>();
    let travel_points = travel_points
        .iter()
        .map(|coord| [coord.x, coord.y])
        .collect::<Vec<_>>();
    let state =
        td_core::calculate_routes(&blockers, &travel_points, [map_wh.width, map_wh.height])?;
    Route::from_core_state(state).map(Arc::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_state_round_trips_raw_coordinates_and_distances() {
        let route = Route {
            map_coords: vec![MapCoord::new(0, 0), MapCoord::new(1, 0)],
            world_coords: vec![WorldCoord::new(0, 0), WorldCoord::new(1_000, 0)],
            segment_lengths: vec![WorldDistance::from_raw(1_000)],
            cumulative_lengths: vec![WorldDistance::ZERO, WorldDistance::from_raw(1_000)],
        };

        let raw = route.to_core_state();
        let restored = Route::from_core_state(raw).expect("valid route state");
        assert_eq!(restored.map_coords, route.map_coords);
        assert_eq!(restored.world_coords, route.world_coords);
        assert_eq!(restored.segment_lengths, route.segment_lengths);
        assert_eq!(restored.cumulative_lengths, route.cumulative_lengths);
    }

    #[test]
    fn route_state_rejects_inconsistent_lengths() {
        assert!(
            Route::from_core_state(td_core::RouteState {
                map_coords: vec![[0, 0]],
                world_coords: vec![[0, 0], [1, 0]],
                segment_lengths: vec![],
                cumulative_lengths: vec![0, 1],
            })
            .is_none()
        );
    }
}

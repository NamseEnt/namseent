mod route_find;

use crate::*;
pub use route_find::*;
use std::sync::Arc;

pub struct Route {
    map_coords: Vec<MapCoord>,
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

pub struct MoveOnRoute {
    route: Arc<Route>,
    route_index: usize,
    /// must be in [0.0, 1.0]
    route_progress: f32,
    map_coord: MapCoordF32,
}

impl MoveOnRoute {
    fn new(route: Arc<Route>) -> Self {
        Self {
            map_coord: route.map_coords[0].map(|x| x as f32),
            route,
            route_index: 0,
            route_progress: 0.0,
        }
    }
    fn is_finished(&self) -> bool {
        self.route_index >= self.route.map_coords.len() - 1
    }
    fn tick(&mut self, velocity: f32, dt: f32) {
        let mut movable_distance = velocity * dt;

        while movable_distance > 0.0 {
            let Some(next_route_xy) = self
                .route
                .map_coords
                .get(self.route_index + 1)
                .map(|x| x.map(|x| x as f32))
            else {
                return;
            };
            let last_route_xy = self.route.map_coords[self.route_index].map(|x| x as f32);
            let left_distance_to_next_route_xy = (next_route_xy - self.map_coord).length();

            if movable_distance < left_distance_to_next_route_xy {
                let distance_between_route_xy = (next_route_xy - last_route_xy).length();
                self.route_progress += movable_distance / distance_between_route_xy;
                // protect from floating point error... gpt recommendation
                self.route_progress = self.route_progress.clamp(0.0, 1.0);
                self.map_coord =
                    last_route_xy + (next_route_xy - last_route_xy) * self.route_progress;
                return;
            }
            movable_distance -= left_distance_to_next_route_xy;
            self.route_index += 1;
            self.route_progress = 0.0;
            self.map_coord = next_route_xy;
        }
    }
    pub fn xy(&self) -> Xy<f32> {
        self.map_coord
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_moves_monster_forward() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0)],
        });
        let mut move_on_route = MoveOnRoute::new(route);
        move_on_route.tick(1.0, 2.5);
        move_on_route.tick(1.0, 2.5);
        assert_eq!(move_on_route.route_index, 0);
        assert!(move_on_route.route_progress > 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(5.0, 0.0));
    }

    #[test]
    fn test_tick_reaches_next_point() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0)],
        });
        let mut move_on_route = MoveOnRoute::new(route);
        move_on_route.tick(1.0, 5.0);
        move_on_route.tick(1.0, 5.0);
        assert_eq!(move_on_route.route_index, 1);
        assert_eq!(move_on_route.route_progress, 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 0.0));
    }

    #[test]
    fn test_tick_finishes_route() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0)],
        });
        let mut move_on_route = MoveOnRoute::new(route);
        move_on_route.tick(1.0, 10.0);
        move_on_route.tick(1.0, 10.0);
        assert!(move_on_route.is_finished());
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 0.0));
    }

    #[test]
    fn test_tick_multiple_points() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0), Xy::new(10, 10)],
        });
        let mut move_on_route = MoveOnRoute::new(route);
        move_on_route.tick(1.0, 5.0);
        assert_eq!(move_on_route.route_index, 0);
        assert!(move_on_route.route_progress > 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(5.0, 0.0));
        move_on_route.tick(1.0, 5.0);
        assert_eq!(move_on_route.route_index, 1);
        assert_eq!(move_on_route.route_progress, 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 0.0));
        move_on_route.tick(1.0, 5.0);
        assert_eq!(move_on_route.route_index, 1);
        assert!(move_on_route.route_progress > 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 5.0));
        move_on_route.tick(1.0, 5.0);
        assert_eq!(move_on_route.route_index, 2);
        assert_eq!(move_on_route.route_progress, 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 10.0));
    }
}

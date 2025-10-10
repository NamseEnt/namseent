use super::*;
use crate::*;
use std::sync::Arc;

#[derive(State)]
pub struct MoveOnRoute {
    route: Arc<Route>,
    route_index: usize,
    /// must be in [0.0, 1.0]
    route_progress: f32,
    map_coord: MapCoordF32,
    velocity: Velocity,
}

pub type Velocity = Per<f32, Duration>;

impl MoveOnRoute {
    pub fn new(route: Arc<Route>, velocity: Velocity) -> Self {
        Self {
            map_coord: route.map_coords[0].map(|x| x as f32),
            route,
            route_index: 0,
            route_progress: 0.0,
            velocity,
        }
    }
    pub fn is_finished(&self) -> bool {
        self.route_index >= self.route.map_coords.len() - 1
    }
    pub fn xy(&self) -> Xy<f32> {
        self.map_coord
    }
    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub(crate) fn move_by(&mut self, dt: Duration) {
        let mut movable_distance = self.velocity * dt;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE_VELOCITY: Velocity = Per::new(1.0, Duration::from_secs(1));

    #[test]
    fn test_tick_moves_monster_forward() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0)],
        });
        let mut move_on_route = MoveOnRoute::new(route, ONE_VELOCITY);
        move_on_route.move_by(Duration::from_secs_f32(2.5));
        move_on_route.move_by(Duration::from_secs_f32(2.5));
        assert_eq!(move_on_route.route_index, 0);
        assert!(move_on_route.route_progress > 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(5.0, 0.0));
    }

    #[test]
    fn test_tick_reaches_next_point() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0)],
        });
        let mut move_on_route = MoveOnRoute::new(route, ONE_VELOCITY);
        move_on_route.move_by(Duration::from_secs_f32(5.0));
        move_on_route.move_by(Duration::from_secs_f32(5.0));
        assert_eq!(move_on_route.route_index, 1);
        assert_eq!(move_on_route.route_progress, 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 0.0));
    }

    #[test]
    fn test_tick_finishes_route() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0)],
        });
        let mut move_on_route = MoveOnRoute::new(route, ONE_VELOCITY);
        move_on_route.move_by(Duration::from_secs_f32(10.0));
        move_on_route.move_by(Duration::from_secs_f32(10.0));
        assert!(move_on_route.is_finished());
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 0.0));
    }

    #[test]
    fn test_tick_multiple_points() {
        let route = Arc::new(Route {
            map_coords: vec![Xy::new(0, 0), Xy::new(10, 0), Xy::new(10, 10)],
        });
        let mut move_on_route = MoveOnRoute::new(route, ONE_VELOCITY);
        move_on_route.move_by(Duration::from_secs_f32(5.0));
        assert_eq!(move_on_route.route_index, 0);
        assert!(move_on_route.route_progress > 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(5.0, 0.0));
        move_on_route.move_by(Duration::from_secs_f32(5.0));
        assert_eq!(move_on_route.route_index, 1);
        assert_eq!(move_on_route.route_progress, 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 0.0));
        move_on_route.move_by(Duration::from_secs_f32(5.0));
        assert_eq!(move_on_route.route_index, 1);
        assert!(move_on_route.route_progress > 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 5.0));
        move_on_route.move_by(Duration::from_secs_f32(5.0));
        assert_eq!(move_on_route.route_index, 2);
        assert_eq!(move_on_route.route_progress, 0.0);
        assert_eq!(move_on_route.xy(), Xy::new(10.0, 10.0));
    }
}

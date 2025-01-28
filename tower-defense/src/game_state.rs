use std::sync::Arc;

use namui::*;

use crate::route_find;

const MAP_SIZE: Wh<usize> = Wh::new(10, 10);

/// ```text
/// ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■
/// ■ 1 ■ ■ ■ 5 ← ← ← ← 4 ■
/// ■ ↓ ■ ■ ■ ↓ ■ ■ ■ ■ ↑ ■
/// ■ ↓ ■ ■ ■ ↓ ■ ■ ■ ■ ↑ ■
/// ■ ↓ ■ ■ ■ ↓ ■ ■ ■ ■ ↑ ■
/// ■ 2 → → → ┼ → → → → 3 ■
/// ■ ■ ■ ■ ■ ↓ ■ ■ ■ ■ ■ ■
/// ■ ■ ■ ■ ■ ↓ ■ ■ ■ ■ ■ ■
/// ■ ■ ■ ■ ■ ↓ ■ ■ ■ ■ ■ ■
/// ■ ■ ■ ■ ■ 6 → → → → 7 ■
/// ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■ ■
const TRAVEL_POINTS: [Xy<usize>; 8] = [
    Xy::new(1, 1),
    Xy::new(1, 5),
    Xy::new(5, 9),
    Xy::new(9, 9),
    Xy::new(9, 5),
    Xy::new(5, 1),
    Xy::new(5, 5),
    Xy::new(9, 5),
];

pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: Vec<Tower>,
    pub camera: Camera,
    pub route: Arc<Route>,
}

pub struct Monster {
    pub move_on_route: MoveOnRoute,
    pub kind: MonsterKind,
}
pub enum MonsterKind {}
pub struct Tower {
    pub position: Xy<usize>,
    pub kind: TowerKind,
}
pub enum TowerKind {}
pub struct Camera {
    pub left_top: Xy<f32>,
    pub zoom_level: ZoomLevel,
}
pub enum ZoomLevel {
    Default,
    ZoomOut,
}

static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

pub struct GameSetting {
    pub map_size: Xy<usize>,
}

pub fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, || GameState {
        monsters: vec![],
        towers: vec![],
        camera: Camera {
            left_top: Xy::new(0.0, 0.0),
            zoom_level: ZoomLevel::Default,
        },
        route: calculate_routes(&[]).unwrap(),
    })
    .0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state<F>(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}

pub struct Route {
    xys: Vec<Xy<usize>>,
}

fn calculate_routes(blockers: &[Xy<usize>]) -> Option<Arc<Route>> {
    let mut xys = vec![];

    for i in 0..TRAVEL_POINTS.len() - 1 {
        let start_xy = TRAVEL_POINTS[i];
        let end_xy = TRAVEL_POINTS[i + 1];
        let Some(route) = route_find::find_shortest_route(MAP_SIZE, start_xy, end_xy, blockers)
        else {
            return None;
        };
        xys.extend(route);
    }

    Some(Arc::new(Route { xys }))
}

pub struct MoveOnRoute {
    route: Arc<Route>,
    route_index: usize,
    route_progress: f32,
    xy: Xy<f32>,
}

impl MoveOnRoute {
    fn new(route: Arc<Route>) -> Self {
        Self {
            xy: route.xys[0].map(|x| x as f32),
            route,
            route_index: 0,
            route_progress: 0.0,
        }
    }
    fn is_finished(&self) -> bool {
        self.route_index >= self.route.xys.len() - 1
    }
    fn tick(&mut self, velocity: f32, dt: f32) {
        let mut movable_distance = velocity * dt;

        while movable_distance > 0.0 {
            let Some(next_route_xy) = self
                .route
                .xys
                .get(self.route_index + 1)
                .map(|x| x.map(|x| x as f32))
            else {
                return;
            };
            let last_route_xy = self.route.xys[self.route_index].map(|x| x as f32);
            let left_distance_to_next_route_xy = (next_route_xy - self.xy).length();

            if movable_distance < left_distance_to_next_route_xy {
                let distance_between_route_xy = (next_route_xy - last_route_xy).length();
                self.route_progress += movable_distance / distance_between_route_xy;
                // protect from floating point error... gpt recommendation
                self.route_progress = self.route_progress.clamp(0.0, 1.0);
                self.xy = last_route_xy + (next_route_xy - last_route_xy) * self.route_progress;
                return;
            }
            movable_distance -= left_distance_to_next_route_xy;
            self.route_index += 1;
            self.route_progress = 0.0;
            self.xy = next_route_xy;
        }
    }
    pub fn xy(&self) -> Xy<f32> {
        self.xy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_moves_monster_forward() {
        let route = Arc::new(Route {
            xys: vec![Xy::new(0, 0), Xy::new(10, 0)],
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
            xys: vec![Xy::new(0, 0), Xy::new(10, 0)],
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
            xys: vec![Xy::new(0, 0), Xy::new(10, 0)],
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
            xys: vec![Xy::new(0, 0), Xy::new(10, 0), Xy::new(10, 10)],
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

use std::sync::Arc;

use namui::*;
use namui_prebuilt::{table::*, *};

const MAP_SIZE: Xy<usize> = Xy::new(10, 10);

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
    pub travel_destination: TravelDestination,
    pub position: Xy<f32>,
    pub kind: MonsterKind,
}
pub enum TravelDestination {
    One,
    Two,
    Three,
    Four,
    Five,
    Finish,
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

pub fn init_gate_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
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

pub fn main() {
    namui::start(|ctx| {
        return;
    });
}

pub struct Route {
    xys: Vec<Xy<usize>>,
}

fn calculate_routes(towers: &[Xy<usize>]) -> Option<Arc<Route>> {
    for i in 0..TRAVEL_POINTS.len() - 1 {
        let from = TRAVEL_POINTS[i];
        let to = TRAVEL_POINTS[i + 1];
    }
}

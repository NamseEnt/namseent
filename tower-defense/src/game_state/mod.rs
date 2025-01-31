mod can_place_tower;
mod render;

use crate::route::*;
use crate::*;
use namui::*;
use std::{collections::BTreeMap, sync::Arc};

const MAP_SIZE: Wh<BlockUnit> = Wh::new(10, 10);

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
const TRAVEL_POINTS: [MapCoord; 8] = [
    MapCoord::new(1, 1),
    MapCoord::new(1, 5),
    MapCoord::new(5, 9),
    MapCoord::new(9, 9),
    MapCoord::new(9, 5),
    MapCoord::new(5, 1),
    MapCoord::new(5, 5),
    MapCoord::new(9, 5),
];

pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub floor_tiles: BTreeMap<MapCoord, FloorTile>,
}

impl Component for &GameState {
    fn render(self, ctx: &RenderCtx) {
        ctx.scale(self.camera.zoom_scale()).compose(|ctx| {
            self.render_monsters(&ctx);
            self.render_towers(&ctx);
            self.render_floor_tiles(&ctx);
        });
    }
}

#[derive(Clone, Copy)]
pub enum FloorTile {}
impl Component for &FloorTile {
    fn render(self, ctx: &RenderCtx) {}
}
pub struct Monster {
    pub move_on_route: MoveOnRoute,
    pub kind: MonsterKind,
}
impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {}
}
#[derive(Clone, Copy)]
pub enum MonsterKind {}

#[derive(Clone, Copy)]
pub struct Tower {
    pub kind: TowerKind,
}
impl Component for &Tower {
    fn render(self, ctx: &RenderCtx) {}
}
#[derive(Clone, Copy)]
pub enum TowerKind {}
pub struct Camera {
    pub left_top: MapCoordF32,
    pub zoom_level: ZoomLevel,
}
impl Camera {
    fn map_coord_to_screen_px_ratio(&self) -> Px {
        todo!()
    }

    fn zoom_scale(&self) -> Xy<f32> {
        todo!()
    }
}
pub enum ZoomLevel {
    Default,
    ZoomOut,
}

static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

pub fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, || GameState {
        monsters: Default::default(),
        towers: Default::default(),
        camera: Camera {
            left_top: Xy::new(0.0, 0.0),
            zoom_level: ZoomLevel::Default,
        },
        route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
        floor_tiles: Default::default(),
    })
    .0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state<F>(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}

/// Assume that the tower's size is 2x2.
#[derive(Default)]
pub struct PlacedTowers {
    /// key is the left-top coord of the tower.
    inner: BTreeMap<MapCoord, Tower>,
}

impl PlacedTowers {
    pub fn iter(&self) -> impl Iterator<Item = (&MapCoord, &Tower)> {
        self.inner.iter()
    }

    pub fn coords(&self) -> Vec<MapCoord> {
        self.inner
            .keys()
            .flat_map(|&left_top| {
                let right_top = left_top + MapCoord::new(1, 0);
                let left_bottom = left_top + MapCoord::new(0, 1);
                let right_bottom = left_top + MapCoord::new(1, 1);
                [left_top, right_top, left_bottom, right_bottom]
            })
            .collect()
    }
}

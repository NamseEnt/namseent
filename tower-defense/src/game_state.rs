use crate::route::*;
use crate::*;
use namui::*;
use std::{collections::BTreeMap, fmt::Debug, sync::Arc};

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
    pub towers: BTreeMap<MapCoord, Tower>,
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

// ASSUME: NO EFFECT AND STATE IN INNER RENDER
// Render in the 1:1 scale, without thinking about the camera zoom level.
impl GameState {
    fn render_floor_tiles(&self, ctx: &ComposeCtx) {
        self.render_stuffs(ctx, self.floor_tiles.iter());
    }

    fn render_towers(&self, ctx: &ComposeCtx) {
        self.render_stuffs(ctx, self.towers.iter());
    }

    fn render_monsters(&self, ctx: &ComposeCtx) {
        self.render_stuffs(
            ctx,
            self.monsters
                .iter()
                .map(|monster| (monster.move_on_route.xy(), monster)),
        );
    }

    fn render_stuffs<'a, C, MapCoord, MapAxis>(
        &self,
        ctx: &ComposeCtx,
        stuffs: impl Iterator<Item = (MapCoord, &'a C)>,
    ) where
        C: 'a,
        &'a C: Component,
        MapCoord: AsRef<Xy<MapAxis>>,
        MapAxis: Ratio + Debug + Clone,
    {
        for (xy, stuff) in stuffs {
            let px_xy = xy
                .as_ref()
                .clone()
                .map(|t| self.camera.map_coord_to_screen_px_ratio() * t);
            ctx.translate(px_xy).add(stuff);
        }
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

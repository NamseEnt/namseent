mod camera;
mod can_place_tower;
pub mod flow;
pub mod item;
mod monster;
mod monster_spawn;
pub mod projectile;
mod quest;
mod render;
mod tick;
pub mod tower;

use crate::shop::ShopSlot;
use crate::*;
use crate::{route::*, upgrade::Upgrade};
use camera::*;
use flow::GameFlow;
use monster::*;
use monster_spawn::*;
use namui::*;
use projectile::*;
use std::{num::NonZeroUsize, sync::Arc};
use tower::*;

/// The size of a tile in pixels, with zoom level 1.0.
const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
const MAP_SIZE: Wh<BlockUnit> = Wh::new(48, 48);
const TRAVEL_POINTS: [MapCoord; 7] = [
    MapCoord::new(6, 0),
    MapCoord::new(6, 23),
    MapCoord::new(41, 23),
    MapCoord::new(41, 6),
    MapCoord::new(24, 6),
    MapCoord::new(24, 41),
    MapCoord::new(47, 41),
];

pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub floor_tiles: Vec<FloorTile>,
    pub upgrades: Vec<Upgrade>,
    pub flow: GameFlow,
    /// one-based
    pub stage: usize,
    pub shop_slot: usize,
    pub quest_slot: usize,
    pub quest_board_slot: usize,
    pub reroll: usize,
    monster_spawn_state: MonsterSpawnState,
    pub projectiles: Vec<Projectile>,
    pub items: Vec<item::Item>,
    pub money: u32,
    pub shop_slots: [ShopSlot; 5],
}

impl Component for &GameState {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(tick::Ticker {});

        ctx.scale(Xy::single(self.camera.zoom_level))
            .translate(TILE_PX_SIZE.as_xy() * self.camera.left_top * -1.0)
            .compose(|ctx| {
                render::render(self, ctx);
            });
    }
}

#[derive(Clone, Copy)]
pub struct FloorTile {
    pub coord: MapCoord,
}
impl Component for &FloorTile {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(simple_rect(
            TILE_PX_SIZE,
            palette::OUTLINE,
            1.px(),
            Color::TRANSPARENT,
        ));
    }
}

static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

pub fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, || GameState {
        monsters: Default::default(),
        towers: Default::default(),
        camera: Camera::new(),
        route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
        floor_tiles: Vec::from_iter((0..MAP_SIZE.width).flat_map(|x| {
            (0..MAP_SIZE.height).map(move |y| FloorTile {
                coord: MapCoord::new(x, y),
            })
        })),
        upgrades: Default::default(),
        flow: GameFlow::SelectingTower,
        stage: 1,
        shop_slot: 3,
        quest_slot: 3,
        quest_board_slot: 1,
        reroll: 1,
        monster_spawn_state: MonsterSpawnState::Idle,
        projectiles: Default::default(),
        items: Default::default(),
        money: 10,
        shop_slots: Default::default(),
    })
    .0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}

/// Assume that the tower's size is 2x2.
/// All iteration in this struct will be in the order of left-top, right-top, left-bottom, right-bottom.
#[derive(Default)]
pub struct PlacedTowers {
    /// key is the left-top coord of the tower.
    inner: Vec<Tower>,
}

impl PlacedTowers {
    pub fn iter(&self) -> impl Iterator<Item = &Tower> {
        self.inner.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tower> {
        self.inner.iter_mut()
    }

    pub fn coords(&self) -> Vec<MapCoord> {
        self.iter()
            .flat_map(|tower| {
                let left_top = tower.left_top;
                let right_top = left_top + MapCoord::new(1, 0);
                let left_bottom = left_top + MapCoord::new(0, 1);
                let right_bottom = left_top + MapCoord::new(1, 1);
                [left_top, right_top, left_bottom, right_bottom]
            })
            .collect()
    }

    pub fn place_tower(&mut self, tower: Tower) {
        // let's find the right place of tower and insert it

        let Some(index) = self.inner.iter().position(|placed_tower| {
            tower.left_top.y < placed_tower.left_top.y || tower.left_top.x < placed_tower.left_top.x
        }) else {
            self.inner.push(tower);
            return;
        };

        self.inner.insert(index, tower);
    }
}

/// Make sure that the tower can be placed at the given coord.
pub fn place_tower(tower: Tower) {
    crate::game_state::mutate_game_state(move |game_state| {
        game_state.towers.place_tower(tower);
    });
}

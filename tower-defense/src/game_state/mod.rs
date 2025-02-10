mod can_place_tower;
pub mod flow;
mod monster_spawn;
mod projectile;
mod render;
mod tick;

use crate::*;
use crate::{route::*, upgrade::Upgrade};
use flow::GameFlow;
use monster_spawn::*;
use namui::*;
use projectile::*;
use std::{collections::BTreeMap, num::NonZeroUsize, sync::Arc};

const MAP_SIZE: Wh<BlockUnit> = Wh::new(49, 43);

const TRAVEL_POINTS: [MapCoord; 7] = [
    MapCoord::new(7, 1),
    MapCoord::new(7, 24),
    MapCoord::new(42, 24),
    MapCoord::new(42, 7),
    MapCoord::new(25, 7),
    MapCoord::new(25, 42),
    MapCoord::new(48, 42),
];

pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub floor_tiles: BTreeMap<MapCoord, FloorTile>,
    pub upgrades: Vec<Upgrade>,
    pub flow: GameFlow,
    monster_spawn_state: MonsterSpawnState,
    pub projectiles: Vec<Projectile>,
}

impl Component for &GameState {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(tick::Ticker {});

        ctx.scale(self.camera.zoom_scale()).compose(|ctx| {
            render::render(self, ctx);
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
    pub projectile_target_indicator: ProjectileTargetIndicator,
    pub hp: usize,
}
impl Monster {
    fn get_damage(&mut self, damage: usize) {
        self.hp = self.hp.saturating_sub(damage);
    }

    fn dead(&self) -> bool {
        self.hp == 0
    }
}
impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {}
}
#[derive(Clone, Copy)]
pub enum MonsterKind {}

impl MonsterKind {
    fn max_hp(&self) -> usize {
        todo!()
    }
}

#[derive(Clone)]
pub struct Tower {
    pub left_top: MapCoord,
    pub kind: TowerKind,
    pub last_shoot_time: Instant,
    pub shoot_interval: Duration,
    pub attack_range_radius: f32,
    pub projectile_kind: ProjectileKind,
    pub projectile_speed: Velocity,
    pub damage: usize,
}
impl Tower {
    fn in_cooltime(&self, now: Instant) -> bool {
        now < self.last_shoot_time + self.shoot_interval
    }

    fn shoot(&mut self, target_indicator: ProjectileTargetIndicator, now: Instant) -> Projectile {
        self.last_shoot_time = now;

        Projectile {
            kind: self.projectile_kind,
            xy: self.left_top.map(|t| t as f32 + 0.5),
            velocity: self.projectile_speed,
            target_indicator,
            damage: self.damage,
        }
    }
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
        upgrades: Default::default(),
        flow: GameFlow::SelectingTower,
        monster_spawn_state: MonsterSpawnState::Idle,
        projectiles: Default::default(),
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

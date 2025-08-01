pub mod background;
mod camera;
mod can_place_tower;
pub mod cursor_preview;
pub mod fast_forward;
mod field_area_effect;
pub mod field_particle;
pub mod flow;
pub mod hand;
pub mod item;
mod level_rarity_weight;
mod monster;
mod monster_spawn;
pub mod projectile;
pub mod quest;
mod render;
pub mod schedule;
mod status_effect_particle_generator;
mod tick;
pub mod tower;
pub mod upgrade;
mod user_status_effect;
pub mod play_history;
mod event_handlers;
mod placed_towers;

use crate::quest_board::QuestBoardSlot;
use crate::route::*;
use crate::shop::ShopSlot;
use crate::*;
use background::{Background, generate_backgrounds};
use camera::*;
use cursor_preview::CursorPreview;
use fast_forward::FastForwardMultiplier;
use field_area_effect::FieldAreaEffect;

use flow::GameFlow;
use item::Item;
pub use level_rarity_weight::level_rarity_weight;
pub use monster::*;
use monster_spawn::*;
use namui::*;
use projectile::*;
use quest::*;
use status_effect_particle_generator::StatusEffectParticleGenerator;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tower::*;
use upgrade::UpgradeState;
use user_status_effect::UserStatusEffect;
use play_history::PlayHistory;
use placed_towers::PlacedTowers;

/// The size of a tile in pixels, with zoom level 1.0.
pub const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
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
pub const MAX_HP: f32 = 100.0;
pub const MAX_INVENTORY_SLOT: usize = 9;

pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub backgrounds: Vec<Background>,
    pub upgrade_state: UpgradeState,
    pub flow: GameFlow,
    /// one-based
    pub stage: usize,
    pub left_reroll_chance: usize,
    monster_spawn_state: MonsterSpawnState,
    pub projectiles: Vec<Projectile>,
    pub items: Vec<item::Item>,
    pub gold: usize,
    pub shop_slots: [ShopSlot; 5],
    pub quest_board_slots: [QuestBoardSlot; 3],
    pub cursor_preview: CursorPreview,
    pub hp: f32,
    pub shield: f32,
    pub user_status_effects: Vec<UserStatusEffect>,
    pub field_area_effects: Vec<FieldAreaEffect>,
    pub left_shop_refresh_chance: usize,
    pub left_quest_board_refresh_chance: usize,
    pub quest_states: Vec<QuestState>,
    pub item_used: bool,
    pub level: NonZeroUsize,
    game_now: Instant,
    pub fast_forward_multiplier: FastForwardMultiplier,
    pub rerolled_count: usize,
    pub field_particle_system_manager: field_particle::FieldParticleSystemManager,
    status_effect_particle_generator: StatusEffectParticleGenerator,
    pub locale: crate::l10n::Locale,
    pub hand: hand::Hand,
    pub play_history: PlayHistory,
}
impl GameState {
    /// 현대적인 텍스트 매니저 반환
    pub fn text(&self) -> crate::l10n::TextManager {
        crate::l10n::TextManager::new(self.locale)
    }

    pub fn in_even_stage(&self) -> bool {
        matches!(self.stage % 2, 0)
    }

    pub fn max_shop_slot(&self) -> usize {
        self.upgrade_state.shop_slot_expand + 3
    }
    pub fn max_quest_slot(&self) -> usize {
        self.upgrade_state.quest_slot_expand + 3
    }
    pub fn max_quest_board_slot(&self) -> usize {
        self.upgrade_state.quest_board_slot_expand + 1
    }
    pub fn max_shop_refresh_chance(&self) -> usize {
        self.upgrade_state.shop_refresh_chance_plus + 1
    }
    pub fn max_quest_board_refresh_chance(&self) -> usize {
        self.upgrade_state.quest_board_refresh_chance_plus + 1
    }
    pub fn max_reroll_chance(&self) -> usize {
        self.upgrade_state.reroll_chance_plus + 1
    }
    pub fn rerolled(&self) -> bool {
        self.rerolled_count > 0
    }


    pub fn now(&self) -> Instant {
        self.game_now
    }

    pub fn level_up_cost(&self) -> usize {
        match self.level.get() {
            1 => 25,
            2 => 50,
            3 => 75,
            4 => 100,
            5 => 150,
            6 => 200,
            7 => 300,
            8 => 500,
            9 => 750,
            10 => 0,
            _ => unreachable!("Level up cost not defined for level {}", self.level),
        }
    }
}

impl Component for &GameState {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(tick::Ticker {});

        ctx.scale(Xy::single(self.camera.zoom_level))
            .translate(TILE_PX_SIZE.to_xy() * self.camera.left_top * -1.0)
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
    ctx.init_atom(&GAME_STATE_ATOM, || {
        let mut game_state = GameState {
            monsters: Default::default(),
            towers: Default::default(),
            camera: Camera::new(),
            route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
            backgrounds: generate_backgrounds(),
            upgrade_state: Default::default(),
            flow: GameFlow::Initializing,
            stage: 1,
            left_reroll_chance: 1,
            monster_spawn_state: MonsterSpawnState::Idle,
            projectiles: Default::default(),
            items: vec![
                Item {
                    kind: item::ItemKind::ExtraReroll,
                    rarity: rarity::Rarity::Epic,
                },
                Item {
                    kind: item::ItemKind::ExtraReroll,
                    rarity: rarity::Rarity::Epic,
                },
                // For debugging purpose, should be removed in production.
                Item {
                    kind: item::ItemKind::RoundDamageOverTime {
                        rank: crate::card::Rank::Ace,
                        suit: crate::card::Suit::Hearts,
                        damage: 20.0,
                        radius: 2.5,
                        duration: namui::Duration::from_secs_f32(3.0),
                    },
                    rarity: rarity::Rarity::Epic,
                },
                Item {
                    kind: item::ItemKind::AttackPowerMultiplyBuff {
                        amount: 2.9,
                        duration: 3.sec(),
                        radius: 4.0,
                    },
                    rarity: rarity::Rarity::Epic,
                },
                Item {
                    kind: item::ItemKind::MovementSpeedDebuff {
                        amount: 0.4,
                        duration: 3.sec(),
                        radius: 4.0,
                    },
                    rarity: rarity::Rarity::Epic,
                },
            ],
            quest_states: Default::default(),
            gold: 100,
            shop_slots: Default::default(),
            quest_board_slots: Default::default(),
            cursor_preview: Default::default(),
            hp: 100.0,
            shield: 0.0,
            user_status_effects: Default::default(),
            field_area_effects: Default::default(),
            left_shop_refresh_chance: 0,
            left_quest_board_refresh_chance: 0,
            item_used: false,
            level: NonZeroUsize::new(1).unwrap(),
            game_now: Instant::now(),
            fast_forward_multiplier: Default::default(),
            rerolled_count: 0,
            field_particle_system_manager: field_particle::FieldParticleSystemManager::default(),
            status_effect_particle_generator: StatusEffectParticleGenerator::new(Instant::now()),
            locale: crate::l10n::Locale::ENGLISH,
            hand: Default::default(),
            play_history: PlayHistory::new(),
        };

        game_state.goto_selecting_tower();
        game_state
    })
    .0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}


/// Make sure that the tower can be placed at the given coord.
pub fn place_tower(tower: Tower) {
    crate::game_state::mutate_game_state(move |game_state| {
        game_state.place_tower(tower);
    });
}

pub fn is_boss_stage(stage: usize) -> bool {
    matches!(stage, 15 | 25 | 30 | 35 | 40 | 45 | 46 | 47 | 48 | 49 | 50)
}

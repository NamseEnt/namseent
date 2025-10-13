pub mod background;
mod camera;
pub mod can_place_tower;
pub mod contract;
pub mod cursor_preview;
pub mod effect;
mod event_handlers;
pub mod fast_forward;
pub mod field_particle;
pub mod flow;
pub mod item;
mod level_rarity_weight;
mod modal;
mod monster;
mod monster_spawn;
mod placed_towers;
pub mod play_history;
pub mod projectile;
mod render;
pub mod stage_modifiers;
mod start_confirm_modal;
mod tick;
pub mod tower;
mod tower_info_popup;
mod ui_state;
pub mod upgrade;
mod user_status_effect;

use crate::game_state::stage_modifiers::StageModifiers;
use crate::hand::HandSlotId;
use crate::route::*;
use crate::*;
use background::{Background, generate_backgrounds};
use camera::*;
use cursor_preview::CursorPreview;
use fast_forward::FastForwardMultiplier;
use flow::GameFlow;
use item::{Effect, Item};
pub use level_rarity_weight::level_rarity_weight;
pub use modal::Modal;
pub use monster::*;
use monster_spawn::*;
use namui::*;
use placed_towers::PlacedTowers;
use play_history::PlayHistory;
use projectile::*;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tower::*;
pub use ui_state::UIState;
use upgrade::UpgradeState;
use user_status_effect::UserStatusEffect;

/// The size of a tile in pixels, with zoom level 1.0.
pub const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
pub const MAP_SIZE: Wh<BlockUnit> = Wh::new(48, 48);
pub const TRAVEL_POINTS: [MapCoord; 7] = [
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
    pub cursor_preview: CursorPreview,
    pub hp: f32,
    pub shield: f32,
    pub user_status_effects: Vec<UserStatusEffect>,
    pub left_shop_refresh_chance: usize,
    pub left_quest_board_refresh_chance: usize,
    pub item_used: bool,
    pub level: NonZeroUsize,
    game_now: Instant,
    pub fast_forward_multiplier: FastForwardMultiplier,
    pub rerolled_count: usize,
    pub field_particle_system_manager: field_particle::FieldParticleSystemManager,
    pub locale: crate::l10n::Locale,
    pub play_history: PlayHistory,
    pub opened_modal: Option<Modal>,
    pub contracts: Vec<contract::Contract>,
    pub stage_modifiers: StageModifiers,
    pub ui_state: UIState,
    pub keyboard_nav: KeyboardNav,
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
        self.upgrade_state.shop_slot_expand + 2
    }
    pub fn max_quest_slot(&self) -> usize {
        self.upgrade_state.quest_slot_expand + 3
    }
    pub fn max_quest_board_slot(&self) -> usize {
        self.upgrade_state.quest_board_slot_expand + 1
    }
    pub fn max_shop_refresh_chance(&self) -> usize {
        (self.upgrade_state.shop_refresh_chance_plus
            + 1
            + self.stage_modifiers.get_shop_max_rerolls_bonus())
        .saturating_sub(self.stage_modifiers.get_shop_max_rerolls_penalty())
    }
    pub fn max_quest_board_refresh_chance(&self) -> usize {
        self.upgrade_state.quest_board_refresh_chance_plus + 1
    }
    pub fn max_reroll_chance(&self) -> usize {
        (self.upgrade_state.reroll_chance_plus
            + 1
            + self
                .stage_modifiers
                .get_card_selection_hand_max_rerolls_bonus())
        .saturating_sub(
            self.stage_modifiers
                .get_card_selection_hand_max_rerolls_penalty(),
        )
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

    pub fn calculate_tower_damage(&self, tower: &tower::Tower) -> f32 {
        let tower_upgrade_states = self.upgrade_state.tower_upgrades(tower);
        let contract_multiplier: f32 = self.stage_modifiers.get_damage_multiplier();
        tower.calculate_projectile_damage(&tower_upgrade_states, contract_multiplier)
    }

    pub fn set_selected_tower(&mut self, tower_id: Option<usize>) {
        self.ui_state.set_selected_tower(tower_id, self.now());
    }

    pub fn cleanup_unused_tower_popup_states(&mut self) {
        let existing_tower_ids: std::collections::HashSet<usize> =
            self.towers.iter().map(|tower| tower.id()).collect();

        self.ui_state.cleanup_unused_states(&existing_tower_ids);
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
        GameState {
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
                    effect: Effect::ExtraReroll,
                    rarity: rarity::Rarity::Epic,
                    value: 0.5.into(),
                },
                Item {
                    effect: Effect::ExtraReroll,
                    rarity: rarity::Rarity::Epic,
                    value: 0.5.into(),
                },
                // For debugging purpose, should be removed in production.
                Item {
                    effect: Effect::Heal { amount: 20.0 },
                    rarity: rarity::Rarity::Epic,
                    value: 0.0.into(), // 디버깅용 - 최소값
                },
            ],
            gold: 100,
            cursor_preview: Default::default(),
            hp: 100.0,
            shield: 0.0,
            user_status_effects: Default::default(),
            left_shop_refresh_chance: 0,
            left_quest_board_refresh_chance: 0,
            item_used: false,
            level: NonZeroUsize::new(1).unwrap(),
            game_now: Instant::now(),
            fast_forward_multiplier: Default::default(),
            rerolled_count: 0,
            field_particle_system_manager: field_particle::FieldParticleSystemManager::default(),
            locale: crate::l10n::Locale::KOREAN,
            play_history: PlayHistory::new(),
            opened_modal: None,
            contracts: vec![],
            stage_modifiers: StageModifiers::new(),
            ui_state: UIState::new(),
            keyboard_nav: KeyboardNav::default(),
        }
    })
    .0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}

pub fn set_modal(modal: Option<Modal>) {
    mutate_game_state(|game_state| {
        game_state.opened_modal = modal;
    });
}

pub fn force_start() {
    mutate_game_state(|game_state| {
        game_state.goto_defense();
    });
}

pub fn is_boss_stage(stage: usize) -> bool {
    matches!(stage, 15 | 25 | 30 | 35 | 40 | 45 | 46 | 47 | 48 | 49 | 50)
}

/// Make sure that the tower can be placed at the given coord.
pub fn place_tower(tower: Tower, placing_tower_slot_id: HandSlotId) {
    crate::game_state::mutate_game_state(move |game_state| {
        game_state.place_tower(tower);
        let GameFlow::PlacingTower { hand } = &mut game_state.flow else {
            unreachable!()
        };
        hand.delete_slots(&[placing_tower_slot_id]);

        // Auto-select the first card (tower or barricade) if available
        if let Some(first_slot_id) = hand.get_slot_id_by_index(0)
            && hand.get_item(first_slot_id).is_some()
        {
            hand.select_slot(first_slot_id);
        }

        if hand.is_empty() {
            game_state.goto_defense();
        }
    });
}

#[derive(Clone, Copy, Default)]
pub struct KeyboardNav {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

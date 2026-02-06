pub mod attack;
pub mod background;
mod camera;
pub mod can_place_tower;
pub mod contract;
pub mod cursor_preview;
#[cfg(feature = "debug-tools")]
mod debug_tools;
pub mod effect;
mod event_handlers;
pub mod fast_forward;
pub mod field_particle;
pub mod flow;
pub mod item;
mod level_rarity_weight;
mod modal;
pub mod monster;
mod monster_spawn;
mod placed_towers;
pub mod play_history;
pub mod projectile;
mod render;
pub mod stage_modifiers;
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
pub use render::*;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tower::*;
pub use ui_state::UIState;
use upgrade::UpgradeState;
use user_status_effect::UserStatusEffect;

/// The size of a tile in pixels, with zoom level 1.0.
pub const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
pub const MAP_SIZE: Wh<BlockUnit> = Wh::new(36, 36);
pub const TRAVEL_POINTS: [MapCoord; 7] = [
    MapCoord::new(5, 0),
    MapCoord::new(5, 17),
    MapCoord::new(31, 17),
    MapCoord::new(31, 5),
    MapCoord::new(18, 5),
    MapCoord::new(18, 31),
    MapCoord::new(35, 31),
];
pub const MAX_HP: f32 = 100.0;

#[derive(State)]
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
    pub monster_spawn_state: MonsterSpawnState,
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
    pub just_cleared_boss_stage: bool,
}
impl GameState {
    /// 현대적인 텍스트 매니저 반환
    pub fn text(&self) -> crate::l10n::TextManager {
        crate::l10n::TextManager::new(self.locale)
    }

    pub fn max_shop_slot(&self) -> usize {
        self.upgrade_state.shop_slot_expand + 2
    }
    pub fn max_shop_refresh_chance(&self) -> usize {
        (self.upgrade_state.shop_refresh_chance_plus
            + 1
            + self.stage_modifiers.get_shop_max_rerolls_bonus())
        .saturating_sub(self.stage_modifiers.get_shop_max_rerolls_penalty())
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

    pub fn set_selected_tower(&mut self, tower_id: Option<usize>) {
        self.ui_state.set_selected_tower(tower_id, self.now());
    }

    pub fn cleanup_unused_tower_popup_states(&mut self) {
        let existing_tower_ids: std::collections::HashSet<usize> =
            self.towers.iter().map(|tower| tower.id()).collect();

        self.ui_state.cleanup_unused_states(&existing_tower_ids);
    }

    pub fn update_camera_shake(&mut self, dt: Duration) {
        self.camera
            .update_shake(dt, self.game_now - Instant::new(Duration::ZERO));
    }
}

#[derive(Clone, Copy, State)]
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

fn create_initial_game_state() -> GameState {
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
        monster_spawn_state: MonsterSpawnState::idle(),
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
            Item {
                effect: Effect::AddTowerCardToPlacementHand {
                    tower_kind: TowerKind::Barricade,
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                    count: 5,
                },
                rarity: rarity::Rarity::Common,
                value: 1.0.into(),
            },
            Item {
                effect: Effect::AddTowerCardToPlacementHand {
                    tower_kind: TowerKind::High,
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                    count: 1,
                },
                rarity: rarity::Rarity::Common,
                value: 1.0.into(),
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
        just_cleared_boss_stage: false,
    };

    game_state.record_game_start();
    game_state.goto_next_stage();
    game_state
}

pub fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, create_initial_game_state).0
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

pub fn restart_game() {
    GAME_STATE_ATOM.mutate(|game_state| {
        *game_state = create_initial_game_state();
    });
}

impl GameState {
    /// Create a deep-ish clone of the current state for debug snapshotting.
    /// Particle systems are cleared and opened modal is dropped to avoid UI leakage.
    pub fn clone_for_debug(&self) -> GameState {
        GameState {
            monsters: self.monsters.clone(),
            towers: self.towers.clone(),
            camera: self.camera.clone(),
            route: Arc::clone(&self.route),
            backgrounds: self.backgrounds.clone(),
            upgrade_state: self.upgrade_state.clone(),
            flow: self.flow.clone(),
            stage: self.stage,
            left_reroll_chance: self.left_reroll_chance,
            monster_spawn_state: self.monster_spawn_state.clone(),
            projectiles: self.projectiles.clone(),
            items: self.items.clone(),
            gold: self.gold,
            cursor_preview: self.cursor_preview.clone(),
            hp: self.hp,
            shield: self.shield,
            user_status_effects: self.user_status_effects.clone(),
            left_shop_refresh_chance: self.left_shop_refresh_chance,
            left_quest_board_refresh_chance: self.left_quest_board_refresh_chance,
            item_used: self.item_used,
            level: self.level,
            game_now: self.game_now,
            fast_forward_multiplier: self.fast_forward_multiplier,
            rerolled_count: self.rerolled_count,
            field_particle_system_manager: field_particle::FieldParticleSystemManager::default(),
            locale: self.locale,
            play_history: self.play_history.clone(),
            opened_modal: None,
            contracts: self.contracts.clone(),
            stage_modifiers: self.stage_modifiers.clone(),
            ui_state: self.ui_state.clone(),
            just_cleared_boss_stage: self.just_cleared_boss_stage,
        }
    }

    /// 현재 스테이지의 클리어율을 계산합니다.
    /// 각 스테이지는 2% (100/50), 스테이지 내에서는 (총 체력 - 남은 체력) / 총 체력 비율로 계산
    /// 체력 회복을 고려하여 실제 남은 몬스터 체력을 기준으로 계산합니다.
    pub fn calculate_clear_rate(&self) -> f32 {
        let total_stages = 50.0;
        let stage_weight = 100.0 / total_stages; // 2%

        // 이전 스테이지 완료율
        let previous_stages_progress = (self.stage.saturating_sub(1) as f32) * stage_weight;

        // 스테이지 진행 데이터는 DefenseFlow에 저장되어 있음
        let (start_total_hp, processed_hp_so_far) = match &self.flow {
            crate::game_state::flow::GameFlow::Defense(defense_flow) => (
                defense_flow.stage_progress.start_total_hp,
                defense_flow.stage_progress.processed_hp,
            ),
            _ => (
                Self::calculate_stage_total_hp(self.stage, &self.stage_modifiers),
                0.0,
            ),
        };

        // 현재 남아있는 몬스터들의 이미 소모된 체력(= max_hp - 현재 hp)을 합산
        let remaining_processed_hp: f32 = self
            .monsters
            .iter()
            .map(|monster| (monster.max_hp - monster.hp.max(0.0)).max(0.0))
            .sum();

        let total_processed_hp = processed_hp_so_far + remaining_processed_hp;

        let current_stage_progress = if start_total_hp > 0.0 {
            (total_processed_hp / start_total_hp).min(1.0) * stage_weight
        } else {
            0.0
        };

        (previous_stages_progress + current_stage_progress).min(100.0)
    }

    /// 특정 스테이지의 총 몬스터 체력을 계산합니다.
    pub fn calculate_stage_total_hp(stage: usize, stage_modifiers: &StageModifiers) -> f32 {
        let health_multiplier = stage_modifiers.get_enemy_health_multiplier();
        let (template_queue, _) = monster_spawn::monster_template_queue_table(stage);
        template_queue
            .iter()
            .map(|t| t.max_hp * health_multiplier)
            .sum()
    }
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
    });
}

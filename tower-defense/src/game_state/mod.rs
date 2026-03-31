pub mod attack;
pub mod background;
mod camera;
pub mod can_place_tower;
pub mod cursor_preview;
#[cfg(feature = "debug-tools")]
mod debug_tools;
pub mod difficulty;
pub mod effect;
mod event_handlers;
pub mod fast_forward;
pub mod field_particle;
pub mod flow;
pub mod item;
mod modal;
pub mod poker_action;
pub use upgrade::{UpgradeInfo, UpgradeInfoDescription, get_upgrade_infos};
pub mod monster;
mod monster_spawn;
mod placed_towers;
pub mod play_history;
pub mod projectile;
mod render;
pub mod stage_modifiers;
mod status_effect_particle_generator;
mod tick;
pub mod tower;
mod tower_info_popup;
mod ui_state;
pub mod upgrade;
mod user_status_effect;

use crate::game_state::stage_modifiers::StageModifiers;
use crate::hand::{Hand, HandItem, HandSlotId};
use crate::route::*;
use crate::*;
use background::{Background, generate_backgrounds};
use camera::*;
use cursor_preview::CursorPreview;
use fast_forward::FastForwardMultiplier;
use flow::GameFlow;
use item::{Effect, Item};
pub use modal::Modal;
pub use monster::*;
use monster_spawn::*;
use namui::*;
use placed_towers::PlacedTowers;
use play_history::PlayHistory;
use projectile::*;
pub use render::*;
use status_effect_particle_generator::StatusEffectParticleGenerator;
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

pub const BASE_DICE_CHANCE: usize = 1;
pub const MAX_DOPAMINE: u8 = 5;

#[derive(State)]
pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub backgrounds: Vec<Background>,
    pub upgrade_state: UpgradeState,
    pub flow: GameFlow,
    pub hand: Hand<HandItem>,
    /// one-based
    pub stage: usize,
    pub left_dice: usize,
    pub monster_spawn_state: MonsterSpawnState,
    pub projectiles: Vec<Projectile>,
    pub delayed_hits: Vec<attack::DelayedHit>,
    pub items: Vec<item::Item>,
    pub gold: usize,
    pub cursor_preview: CursorPreview,
    pub hp: f32,
    pub shield: f32,
    pub user_status_effects: Vec<UserStatusEffect>,
    pub left_quest_board_refresh_chance: usize,
    pub item_used: bool,
    game_now: Instant,
    pub fast_forward_multiplier: FastForwardMultiplier,
    pub rerolled_count: usize,
    pub locale: crate::l10n::Locale,
    pub play_history: PlayHistory,
    pub opened_modal: Option<Modal>,
    pub stage_modifiers: StageModifiers,
    pub stage_difficulty_choices: difficulty::DifficultyChoices,
    pub ui_state: UIState,
    pub dopamine: u8,
    pub treasure_tokens: u8,
    pub pending_next_stage_offer: poker_action::NextStageOffer,
    pub shop_panel_mode: poker_action::NextStageOffer,
    pub status_effect_particle_generator: StatusEffectParticleGenerator,
    pub black_smoke_sources: Vec<field_particle::emitter::BlackSmokeSource>,

    // panel open states controlled by input/flow
    pub hand_panel_forced_open: bool,
    pub shop_panel_forced_open: bool,
}
impl GameState {
    /// 현대적인 텍스트 매니저 반환
    pub fn text(&self) -> crate::l10n::TextManager {
        crate::l10n::TextManager::new(self.locale)
    }

    pub fn max_shop_slot(&self) -> usize {
        self.upgrade_state.shop_slot_expand + 2
    }
    pub fn max_dice_chance(&self) -> usize {
        (self.upgrade_state.dice_chance_plus
            + BASE_DICE_CHANCE
            + self.stage_modifiers.get_max_rerolls_bonus())
        .saturating_sub(self.stage_modifiers.get_max_rerolls_penalty())
    }

    pub fn generate_rarity(&self) -> crate::rarity::Rarity {
        const WEIGHTS: [usize; 4] = [90, 10, 1, 0];
        const RARITIES: [crate::rarity::Rarity; 4] = [
            crate::rarity::Rarity::Common,
            crate::rarity::Rarity::Rare,
            crate::rarity::Rarity::Epic,
            crate::rarity::Rarity::Legendary,
        ];

        let total_weight: usize = WEIGHTS.iter().sum();
        let random_value = rand::random::<usize>() % total_weight;

        let mut cumulative_weight = 0;
        for (i, &weight) in WEIGHTS.iter().enumerate() {
            cumulative_weight += weight;
            if random_value < cumulative_weight {
                return RARITIES[i];
            }
        }
        unreachable!()
    }

    /// Returns whether the hand panel is allowed to be opened based on current flow.
    pub fn can_open_hand_panel(&self) -> bool {
        matches!(
            self.flow,
            GameFlow::SelectingTower(_) | GameFlow::PlacingTower
        )
    }

    /// Returns whether the shop panel is allowed to be opened based on current flow and panel mode.
    pub fn can_open_shop_panel(&self) -> bool {
        matches!(self.flow, GameFlow::SelectingTower(_))
            && self.shop_panel_mode != crate::game_state::poker_action::NextStageOffer::None
    }

    /// Toggle panels according to rules described in UI feature request.
    ///
    /// * If any panel that is allowed is currently open, close both.
    /// * Otherwise open all allowed panels (disallowed ones stay closed).
    pub fn toggle_panels(&mut self) {
        let hand_allowed = self.can_open_hand_panel();
        let shop_allowed = self.can_open_shop_panel();
        let hand_open = hand_allowed && self.hand_panel_forced_open;
        let shop_open = shop_allowed && self.shop_panel_forced_open;
        if hand_open || shop_open {
            self.hand_panel_forced_open = false;
            self.shop_panel_forced_open = false;
        } else {
            if hand_allowed {
                self.hand_panel_forced_open = true;
            }
            if shop_allowed {
                self.shop_panel_forced_open = true;
            }
        }
    }

    pub fn now(&self) -> Instant {
        self.game_now
    }

    pub fn apply_dopamine_delta(&mut self, delta: i8) {
        if delta >= 0 {
            self.dopamine = self.dopamine.saturating_add(delta as u8).min(MAX_DOPAMINE);
        } else {
            self.dopamine = self.dopamine.saturating_sub((-delta) as u8);
        }
    }

    pub fn is_dopamine_depleted(&self) -> bool {
        self.dopamine == 0
    }

    pub fn add_treasure_token(&mut self, amount: u8) {
        self.treasure_tokens = self
            .treasure_tokens
            .saturating_add(amount)
            .min(self.upgrade_state.max_treasure_tokens);
    }

    pub fn spend_treasure_token(&mut self, amount: u8) -> bool {
        if self.treasure_tokens < amount {
            return false;
        }
        self.treasure_tokens -= amount;
        true
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
    let now = Instant::now();
    let mut game_state = GameState {
        monsters: Default::default(),
        towers: Default::default(),
        camera: Camera::new(),
        route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
        backgrounds: generate_backgrounds(),
        upgrade_state: Default::default(),
        flow: GameFlow::Initializing,
        hand: Hand::new(std::iter::empty::<HandItem>()),
        stage: 1,
        left_dice: 0,
        monster_spawn_state: MonsterSpawnState::idle(),
        projectiles: Default::default(),
        delayed_hits: Default::default(),
        items: vec![
            Item {
                effect: Effect::ExtraDice,
                value: 0.5.into(),
            },
            Item {
                effect: Effect::ExtraDice,
                value: 0.5.into(),
            },
            Item {
                effect: Effect::AddTowerCardToPlacementHand {
                    tower_kind: TowerKind::Barricade,
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                    count: 5,
                },
                value: 1.0.into(),
            },
            Item {
                effect: Effect::AddTowerCardToPlacementHand {
                    tower_kind: TowerKind::High,
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                    count: 1,
                },
                value: 1.0.into(),
            },
        ],
        gold: 100,
        cursor_preview: Default::default(),
        hp: 100.0,
        shield: 0.0,
        user_status_effects: Default::default(),
        left_quest_board_refresh_chance: 0,
        item_used: false,
        game_now: now,
        fast_forward_multiplier: Default::default(),
        rerolled_count: 0,
        locale: crate::l10n::Locale::KOREAN,
        play_history: PlayHistory::new(),
        opened_modal: None,
        stage_modifiers: StageModifiers::new(),
        stage_difficulty_choices: difficulty::generate_difficulty_choices(1),
        ui_state: UIState::new(),
        dopamine: MAX_DOPAMINE.div_ceil(2),
        treasure_tokens: 0,
        pending_next_stage_offer: poker_action::NextStageOffer::None,
        shop_panel_mode: poker_action::NextStageOffer::None,
        status_effect_particle_generator: StatusEffectParticleGenerator::new(now),
        black_smoke_sources: Default::default(),

        // start panels in opened state by default (if flow allows later)
        hand_panel_forced_open: true,
        shop_panel_forced_open: true,
    };

    // Start with selecting tower flow and default shop mode (normal shop).
    game_state.goto_selecting_tower();
    game_state.record_game_start();
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
            hand: self.hand.clone(),
            stage: self.stage,
            left_dice: self.left_dice,
            monster_spawn_state: self.monster_spawn_state.clone(),
            projectiles: self.projectiles.clone(),
            delayed_hits: self.delayed_hits.clone(),
            items: self.items.clone(),
            gold: self.gold,
            cursor_preview: self.cursor_preview.clone(),
            hp: self.hp,
            shield: self.shield,
            user_status_effects: self.user_status_effects.clone(),
            left_quest_board_refresh_chance: self.left_quest_board_refresh_chance,
            item_used: self.item_used,
            game_now: self.game_now,
            fast_forward_multiplier: self.fast_forward_multiplier,
            rerolled_count: self.rerolled_count,
            locale: self.locale,
            play_history: self.play_history.clone(),
            opened_modal: None,
            stage_modifiers: self.stage_modifiers.clone(),
            stage_difficulty_choices: self.stage_difficulty_choices.clone(),
            ui_state: self.ui_state.clone(),
            dopamine: self.dopamine,
            treasure_tokens: self.treasure_tokens,
            pending_next_stage_offer: self.pending_next_stage_offer,
            shop_panel_mode: self.shop_panel_mode,
            status_effect_particle_generator: StatusEffectParticleGenerator::new(self.game_now),
            black_smoke_sources: Default::default(),

            hand_panel_forced_open: self.hand_panel_forced_open,
            shop_panel_forced_open: self.shop_panel_forced_open,
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
    // Every 5th stage, plus the last 5 final stages.
    stage.is_multiple_of(5) || (stage >= 46)
}

/// Make sure that the tower can be placed at the given coord.
pub fn place_tower(tower: Tower, placing_tower_slot_id: HandSlotId) {
    crate::game_state::mutate_game_state(move |game_state| {
        game_state.place_tower(tower);
        game_state.hand.delete_slots(&[placing_tower_slot_id]);

        // Auto-select the first card (tower or barricade) if available
        if let Some(first_slot_id) = game_state.hand.get_slot_id_by_index(0)
            && game_state
                .hand
                .get_item(first_slot_id)
                .and_then(|item| item.as_tower())
                .is_some()
        {
            game_state.hand.select_slot(first_slot_id);
        }
    });
}

// Unit tests that exercise panel toggle behavior.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_panels_basic_scenarios() {
        let mut gs = create_initial_game_state();

        // initially flow is SelectingTower (round 0 default shop pick)
        assert!(gs.can_open_hand_panel());
        assert!(!gs.can_open_shop_panel()); // shop_panel_mode starts as None per new behavior

        // selecting tower flow: hand panel is allowed, shop is disabled unless mode set
        assert!(gs.hand_panel_forced_open);
        assert!(gs.shop_panel_forced_open);

        gs.toggle_panels();
        // toggling when one or both allowed should close both
        assert!(!gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // reopening when none open should open hand panel again, shop remains disabled
        gs.toggle_panels();
        assert!(gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // enter selecting tower flow - hand is allowed; shop is disabled until offer is set
        gs.goto_selecting_tower();
        assert!(gs.can_open_hand_panel());
        assert!(!gs.can_open_shop_panel());
        // forced flags were true already; hand panel open
        assert!(gs.hand_panel_forced_open && gs.can_open_hand_panel());
        assert!(gs.shop_panel_forced_open); // still true by state but not open because cannot open

        // space should close hand
        gs.toggle_panels();
        assert!(!gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // closing again should reopen only hand panel, shop remains disabled (forced state may stay false)
        gs.toggle_panels();
        assert!(gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // go to placing tower flow: hand allowed, shop not
        gs.goto_placing_tower(crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::Barricade,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        ));
        assert!(gs.can_open_hand_panel());
        assert!(!gs.can_open_shop_panel());

        // forced flags remain whatever they were; toggle logic should respect permissions
        // shop is not allowed in placing flow, so forced flag can be false
        assert!(gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // space when hand is open should close both (shop will be closed but can't open anyway)
        gs.toggle_panels();
        assert!(!gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // toggle again should reopen only the hand panel since shop is not allowed
        gs.toggle_panels();
        assert!(gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);
    }

    #[test]
    fn selecting_tower_allows_shop_panel() {
        let mut gs = create_initial_game_state();
        gs.flow = GameFlow::SelectingTower(crate::game_state::flow::SelectingTowerFlow::new(&gs));
        gs.shop_panel_mode = crate::game_state::poker_action::NextStageOffer::Shop;
        assert!(gs.can_open_shop_panel());
    }

    #[test]
    fn shop_panel_mode_none_disables_shop_panel() {
        let mut gs = create_initial_game_state();
        gs.flow = GameFlow::SelectingTower(crate::game_state::flow::SelectingTowerFlow::new(&gs));
        gs.shop_panel_mode = crate::game_state::poker_action::NextStageOffer::None;
        assert!(!gs.can_open_shop_panel());
    }

    #[test]
    fn dopamine_helpers_clamp_and_subtract() {
        let mut gs = create_initial_game_state();

        // deterministic start
        gs.dopamine = 0;

        gs.apply_dopamine_delta(2);
        assert_eq!(gs.dopamine, 2);
        assert!(!gs.is_dopamine_depleted());

        gs.apply_dopamine_delta(10);
        assert_eq!(gs.dopamine, MAX_DOPAMINE);
        assert!(!gs.is_dopamine_depleted());

        gs.apply_dopamine_delta(-3);
        assert_eq!(gs.dopamine, MAX_DOPAMINE.saturating_sub(3));

        gs.apply_dopamine_delta(-10);
        assert_eq!(gs.dopamine, 0);
        assert!(gs.is_dopamine_depleted());
    }

    #[test]
    fn treasure_token_helpers_respect_cap_and_spend() {
        let mut gs = create_initial_game_state();

        assert_eq!(gs.treasure_tokens, 0);
        gs.add_treasure_token(1);
        assert_eq!(gs.treasure_tokens, 1);

        gs.add_treasure_token(10);
        assert_eq!(gs.treasure_tokens, gs.upgrade_state.max_treasure_tokens);

        assert!(gs.spend_treasure_token(1));
        assert_eq!(gs.treasure_tokens, gs.upgrade_state.max_treasure_tokens - 1);

        let current = gs.treasure_tokens;
        assert!(!gs.spend_treasure_token(current + 1));
        assert_eq!(gs.treasure_tokens, current);
    }
}

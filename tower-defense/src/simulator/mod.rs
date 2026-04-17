//! Headless game simulator for balance testing.
//!
//! Runs game simulations without rendering, collects statistics into SQLite.

pub mod events;
pub mod recording;
pub mod stats;
pub mod strategies;

use crate::card::Deck;
use crate::config::GameConfig;
use crate::game_state::flow::GameFlow;
use crate::game_state::monster_spawn::MonsterSpawnState;
use crate::game_state::play_history::HistoryEventType;
use crate::game_state::stage_modifiers::StageModifiers;
use crate::game_state::tick::{TICK_MAX_DURATION, tick_headless};
use crate::game_state::{
    EffectEventQueue, GameState, MAP_SIZE, TRAVEL_POINTS, play_history::PlayHistory,
};
use crate::hand::{Hand, HandItem};
use crate::route::calculate_routes;
use std::sync::Arc;

use events::SimEvent;
use namui::Instant;
use strategies::{
    CardRerollStrategy, ItemUseStrategy, ShopStrategy, TowerPlacementStrategy, TreasureStrategy,
};

/// A single headless game simulation.
pub struct HeadlessGame {
    pub game_state: GameState,
    pub events: Vec<SimEvent>,
    pub total_towers_placed: usize,
    pub total_items_used: usize,
    pub total_damage_taken: f32,
    pub stage_damage: Vec<f32>,
    pub total_gold_earned: usize,
}

impl HeadlessGame {
    /// Create a new headless game with default initial state.
    pub fn new() -> Self {
        Self::new_with_config(Arc::new(GameConfig::default_config()))
    }

    /// Create a new headless game using a supplied game configuration.
    pub fn new_with_config(config: Arc<GameConfig>) -> Self {
        let game_state = create_headless_game_state(config.clone());
        Self {
            game_state,
            events: Vec::new(),
            total_towers_placed: 0,
            total_items_used: 0,
            total_damage_taken: 0.0,
            stage_damage: Vec::new(),
            total_gold_earned: 0,
        }
    }

    /// Run a full game simulation with the given strategies.
    #[allow(clippy::too_many_arguments)]
    pub fn run<F>(
        &mut self,
        shop_strategy: &dyn ShopStrategy,
        card_reroll_strategy: &dyn CardRerollStrategy,
        tower_placement_strategy: &dyn TowerPlacementStrategy,
        item_use_strategy: &dyn ItemUseStrategy,
        treasure_strategy: &dyn TreasureStrategy,
        rng: &mut impl rand::Rng,
        mut on_clear_rate_update: F,
    ) -> SimResult
    where
        F: FnMut(f32) -> bool,
    {
        self.events.push(SimEvent::GameStart);

        // Initialize: go to selecting tower for stage 1
        self.game_state.goto_selecting_tower();
        self.events.push(SimEvent::StageStart {
            stage: self.game_state.stage,
        });

        let mut last_history_event_index = 0;

        loop {
            match self.game_state.flow.clone() {
                GameFlow::Initializing => {
                    self.game_state.goto_selecting_tower();
                }
                GameFlow::SelectingTower(_) => {
                    let stage = self.game_state.stage;
                    let _gold_before = self.game_state.gold;
                    let _hp_before = self.game_state.hp;

                    // Execute shop strategy
                    shop_strategy.execute_shop(&mut self.game_state, rng);
                    self.drain_play_history_events(&mut last_history_event_index);

                    // Execute item use strategy before card selection
                    item_use_strategy.on_before_defense(&mut self.game_state);
                    self.drain_play_history_events(&mut last_history_event_index);

                    // Execute card reroll and tower selection
                    let rerolls_before = self.game_state.rerolled_count;
                    card_reroll_strategy.execute_card_selection(&mut self.game_state, rng);
                    let rerolls_used = self.game_state.rerolled_count - rerolls_before;
                    self.drain_play_history_events(&mut last_history_event_index);

                    self.events.push(SimEvent::CardReroll {
                        stage,
                        reroll_number: rerolls_used,
                    });
                }
                GameFlow::PlacingTower => {
                    let towers_before = self.game_state.towers.iter().count();

                    // Execute tower placement strategy
                    tower_placement_strategy.execute_placement(&mut self.game_state);

                    let towers_after = self.game_state.towers.iter().count();
                    self.total_towers_placed += towers_after.saturating_sub(towers_before);

                    // If still in PlacingTower, force defense
                    if matches!(self.game_state.flow, GameFlow::PlacingTower) {
                        self.game_state.goto_defense();
                    }

                    self.events.push(SimEvent::DefenseStart {
                        stage: self.game_state.stage,
                    });
                }
                GameFlow::Defense(_) => {
                    let clear_rate = self.game_state.calculate_clear_rate();
                    if !on_clear_rate_update(clear_rate) {
                        return SimResult {
                            victory: false,
                            final_stage: self.game_state.stage,
                            clear_rate,
                            final_hp: self.game_state.hp,
                            final_gold: self.game_state.gold,
                            total_towers_placed: self.total_towers_placed,
                            total_items_used: self.total_items_used,
                            total_damage_taken: self.total_damage_taken,
                            stage_damage: self.stage_damage.clone(),
                            total_gold_earned: self.total_gold_earned,
                        };
                    }

                    let hp_before = self.game_state.hp;

                    // Run defense simulation with fixed time ticks
                    let continue_sim = self.simulate_defense(
                        item_use_strategy,
                        &mut last_history_event_index,
                        &mut on_clear_rate_update,
                    );
                    self.drain_play_history_events(&mut last_history_event_index);

                    let damage_this_stage = (hp_before - self.game_state.hp).max(0.0);
                    self.total_damage_taken += damage_this_stage;
                    self.stage_damage.push(damage_this_stage);

                    if !continue_sim {
                        return SimResult {
                            victory: false,
                            final_stage: self.game_state.stage,
                            clear_rate: self.game_state.calculate_clear_rate(),
                            final_hp: self.game_state.hp,
                            final_gold: self.game_state.gold,
                            total_towers_placed: self.total_towers_placed,
                            total_items_used: self.total_items_used,
                            total_damage_taken: self.total_damage_taken,
                            stage_damage: self.stage_damage.clone(),
                            total_gold_earned: self.total_gold_earned,
                        };
                    }
                }
                GameFlow::TreasureSelection(ref flow) => {
                    let options = flow.options.clone();
                    if !options.is_empty() {
                        let choice =
                            treasure_strategy.select_treasure(&self.game_state, &options, rng);
                        let upgrade_kind =
                            Self::canonicalize_debug_name(format!("{:?}", options[choice].kind));
                        self.events.push(SimEvent::TreasureSelected {
                            stage: self.game_state.stage,
                            upgrade_kind,
                        });
                        self.game_state.select_treasure(choice);
                    } else {
                        self.game_state.goto_next_stage();
                    }

                    self.events.push(SimEvent::StageStart {
                        stage: self.game_state.stage,
                    });
                }
                GameFlow::Result { clear_rate } => {
                    on_clear_rate_update(clear_rate);
                    let victory = clear_rate >= 100.0;
                    self.events.push(SimEvent::GameEnd {
                        final_stage: self.game_state.stage,
                        victory,
                        clear_rate,
                    });

                    return SimResult {
                        victory,
                        final_stage: self.game_state.stage,
                        clear_rate,
                        final_hp: self.game_state.hp,
                        final_gold: self.game_state.gold,
                        total_towers_placed: self.total_towers_placed,
                        total_items_used: self.total_items_used,
                        total_damage_taken: self.total_damage_taken,
                        stage_damage: self.stage_damage.clone(),
                        total_gold_earned: self.total_gold_earned,
                    };
                }
            }
        }
    }

    /// Simulate the defense phase with fixed time ticks until completion.
    fn simulate_defense<F>(
        &mut self,
        item_use_strategy: &dyn ItemUseStrategy,
        last_history_event_index: &mut usize,
        on_clear_rate_update: &mut F,
    ) -> bool
    where
        F: FnMut(f32) -> bool,
    {
        let tick_dt = TICK_MAX_DURATION;
        let max_ticks = 60 * 60 * 5; // 5 minutes at 60fps as safety limit
        let mut tick_count = 0;

        let hp_before = self.game_state.hp;

        while matches!(self.game_state.flow, GameFlow::Defense(_)) && tick_count < max_ticks {
            self.game_state.advance_time(tick_dt);
            tick_headless(&mut self.game_state, tick_dt);
            tick_count += 1;

            let clear_rate = self.game_state.calculate_clear_rate();
            if !on_clear_rate_update(clear_rate) {
                return false;
            }

            // Check if damage was taken and invoke item strategy
            let current_hp = self.game_state.hp;
            if current_hp < hp_before {
                let damage = hp_before - current_hp;
                item_use_strategy.on_damage_taken(&mut self.game_state, damage);
                self.drain_play_history_events(last_history_event_index);
            }
        }

        true
    }

    fn drain_play_history_events(&mut self, last_history_event_index: &mut usize) {
        while *last_history_event_index < self.game_state.play_history.events.len() {
            let event = &self.game_state.play_history.events[*last_history_event_index];
            match &event.event_type {
                HistoryEventType::ItemPurchased { item, cost } => {
                    self.events.push(SimEvent::ShopPurchase {
                        stage: event.stage,
                        cost: *cost,
                        item_kind: Self::canonicalize_debug_name(format!("{:?}", item.kind)),
                    });
                }
                HistoryEventType::UpgradePurchased { upgrade, cost } => {
                    self.events.push(SimEvent::ShopPurchase {
                        stage: event.stage,
                        cost: *cost,
                        item_kind: Self::canonicalize_debug_name(format!("{:?}", upgrade.kind)),
                    });
                }
                HistoryEventType::ItemUsed { item } => {
                    self.events.push(SimEvent::ItemUsed {
                        stage: event.stage,
                        item_kind: Self::canonicalize_debug_name(format!("{:?}", item.kind)),
                    });
                }
                _ => {}
            }
            *last_history_event_index += 1;
        }
    }

    fn canonicalize_debug_name(name: String) -> String {
        name.split(' ').next().unwrap_or(&name).to_owned()
    }
}

impl Default for HeadlessGame {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a single simulation run.
#[derive(Clone, Debug)]
pub struct SimResult {
    pub victory: bool,
    pub final_stage: usize,
    pub clear_rate: f32,
    pub final_hp: f32,
    pub final_gold: usize,
    pub total_towers_placed: usize,
    pub total_items_used: usize,
    pub total_damage_taken: f32,
    pub stage_damage: Vec<f32>,
    pub total_gold_earned: usize,
}

/// Create a GameState suitable for headless simulation.
fn create_headless_game_state(config: Arc<GameConfig>) -> GameState {
    use crate::game_state::BaseAnimationState;
    use crate::game_state::Camera;
    use crate::game_state::StatusEffectParticleGenerator;
    use crate::game_state::UIState;

    let now = Instant::now();
    GameState {
        monsters: Default::default(),
        towers: Default::default(),
        camera: Camera::new(),
        route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
        backgrounds: crate::game_state::background::generate_backgrounds(),
        upgrade_state: Default::default(),
        flow: GameFlow::Initializing,
        hand: Hand::new(std::iter::empty::<HandItem>()),
        stage: 1,
        left_dice: config.player.base_dice_chance,
        monster_spawn_state: MonsterSpawnState::idle(),
        projectiles: Default::default(),
        delayed_hits: Default::default(),
        items: vec![],
        gold: config.player.starting_gold,
        cursor_preview: Default::default(),
        hp: config.player.starting_hp,
        shield: 0.0,
        user_status_effects: Default::default(),
        left_quest_board_refresh_chance: 0,
        item_used: false,
        game_now: now,
        fast_forward_multiplier: Default::default(),
        rerolled_count: 0,
        locale: crate::l10n::Locale::KOREAN,
        deck: Deck::new(0),
        play_history: PlayHistory::new(),
        opened_modal: None,
        stage_modifiers: StageModifiers::new(),
        ui_state: UIState::new(),
        status_effect_particle_generator: StatusEffectParticleGenerator::new(now),
        black_smoke_sources: Default::default(),
        effect_events: EffectEventQueue::default(),
        base_animation_state: BaseAnimationState::new(now),
        config: config.clone(),
        hand_panel_forced_open: false,
        shop_panel_forced_open: false,
    }
}

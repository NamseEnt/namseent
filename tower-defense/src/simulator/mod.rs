//! Headless game simulator for balance testing.
//!
//! Runs game simulations without rendering, collects statistics into SQLite.

pub mod events;
pub mod game_config;
pub mod recording;
pub mod strategies;

use crate::card::Deck;
use crate::game_state::flow::GameFlow;
use crate::game_state::monster_spawn::MonsterSpawnState;
use crate::game_state::stage_modifiers::StageModifiers;
use crate::game_state::tick::{TICK_MAX_DURATION, tick_headless};
use crate::game_state::{GameState, MAP_SIZE, MAX_HP, TRAVEL_POINTS, play_history::PlayHistory};
use crate::hand::{Hand, HandItem};
use crate::route::calculate_routes;

use events::SimEvent;
use namui::Instant;
use strategies::{CardRerollStrategy, ItemUseStrategy, ShopStrategy, TowerPlacementStrategy};

/// A single headless game simulation.
pub struct HeadlessGame {
    pub game_state: GameState,
    pub events: Vec<SimEvent>,
    pub total_towers_placed: usize,
    pub total_items_used: usize,
    pub total_damage_taken: f32,
    pub total_gold_earned: usize,
}

impl HeadlessGame {
    /// Create a new headless game with default initial state.
    pub fn new() -> Self {
        let game_state = create_headless_game_state();
        Self {
            game_state,
            events: Vec::new(),
            total_towers_placed: 0,
            total_items_used: 0,
            total_damage_taken: 0.0,
            total_gold_earned: 0,
        }
    }

    /// Run a full game simulation with the given strategies.
    pub fn run<F>(
        &mut self,
        shop_strategy: &dyn ShopStrategy,
        card_reroll_strategy: &dyn CardRerollStrategy,
        tower_placement_strategy: &dyn TowerPlacementStrategy,
        item_use_strategy: &dyn ItemUseStrategy,
        rng: &mut impl rand::Rng,
        mut on_clear_rate_update: F,
    ) -> SimResult
    where
        F: FnMut(f32),
    {
        self.events.push(SimEvent::GameStart);

        // Initialize: go to selecting tower for stage 1
        self.game_state.goto_selecting_tower();
        self.events.push(SimEvent::StageStart {
            stage: self.game_state.stage,
        });

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

                    // Execute item use strategy before card selection
                    item_use_strategy.on_before_defense(&mut self.game_state);

                    // Execute card reroll and tower selection
                    let rerolls_before = self.game_state.rerolled_count;
                    card_reroll_strategy.execute_card_selection(&mut self.game_state, rng);
                    let rerolls_used = self.game_state.rerolled_count - rerolls_before;

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
                    on_clear_rate_update(clear_rate);

                    let hp_before = self.game_state.hp;

                    // Run defense simulation with fixed time ticks
                    self.simulate_defense(item_use_strategy);

                    let damage_this_stage = (hp_before - self.game_state.hp).max(0.0);
                    self.total_damage_taken += damage_this_stage;
                }
                GameFlow::TreasureSelection(ref flow) => {
                    // Select the first treasure option
                    let options = flow.options.clone();
                    if !options.is_empty() {
                        let choice = rng.gen_range(0..options.len());
                        let upgrade_kind = format!("{:?}", options[choice].kind);
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
                        total_gold_earned: self.total_gold_earned,
                    };
                }
            }
        }
    }

    /// Simulate the defense phase with fixed time ticks until completion.
    fn simulate_defense(&mut self, item_use_strategy: &dyn ItemUseStrategy) {
        let tick_dt = TICK_MAX_DURATION;
        let max_ticks = 60 * 60 * 5; // 5 minutes at 60fps as safety limit
        let mut tick_count = 0;

        let hp_before = self.game_state.hp;

        while matches!(self.game_state.flow, GameFlow::Defense(_)) && tick_count < max_ticks {
            self.game_state.advance_time(tick_dt);
            tick_headless(&mut self.game_state, tick_dt);
            tick_count += 1;

            // Check if damage was taken and invoke item strategy
            let current_hp = self.game_state.hp;
            if current_hp < hp_before {
                let damage = hp_before - current_hp;
                item_use_strategy.on_damage_taken(&mut self.game_state, damage);
            }
        }
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
    pub total_gold_earned: usize,
}

/// Create a GameState suitable for headless simulation.
fn create_headless_game_state() -> GameState {
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
        left_dice: 1,
        monster_spawn_state: MonsterSpawnState::idle(),
        projectiles: Default::default(),
        delayed_hits: Default::default(),
        items: vec![],
        gold: 0,
        cursor_preview: Default::default(),
        hp: MAX_HP,
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
        base_animation_state: BaseAnimationState::new(now),
        hand_panel_forced_open: false,
        shop_panel_forced_open: false,
    }
}

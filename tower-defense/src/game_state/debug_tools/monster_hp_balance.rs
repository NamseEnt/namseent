use crate::card::{Rank, Suit};
use crate::game_state::{
    debug_tools::{
        add_tower_card::get_expected_tower_for_stage,
        add_upgrade::{UpgradeCategory, get_expected_upgrade_for_stage},
        spiral_place::place_selected_tower_in_spiral,
        state_snapshot,
    },
    mutate_game_state,
    tower::TowerTemplate,
    upgrade::Upgrade,
};
use crate::theme::{
    button::{Button, ButtonVariant},
    typography::memoized_text,
};
use namui::*;
use rand::{Rng, thread_rng};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, State)]
pub struct BalanceState {
    pub hp_offset: f32,
    pub increment: f32,
    pub all_killed_last_time: Option<bool>,
    pub display_text: String,
    pub snapshot_index: usize,
    pub defense_start_hp: f32,
    pub needs_adjustment: bool,
    pub base_max_hp: f32,
}

impl Default for BalanceState {
    fn default() -> Self {
        Self {
            hp_offset: 0.0,
            increment: 0.0,
            all_killed_last_time: None,
            display_text: "HP Balance: Ready".to_string(),
            snapshot_index: 0,
            defense_start_hp: 100.0,
            needs_adjustment: false,
            base_max_hp: 1.0,
        }
    }
}

fn balance_state_storage() -> &'static Mutex<Option<BalanceState>> {
    static STORAGE: OnceLock<Mutex<Option<BalanceState>>> = OnceLock::new();
    STORAGE.get_or_init(|| Mutex::new(None))
}

pub fn get_hp_offset() -> f32 {
    balance_state_storage()
        .lock()
        .expect("balance state mutex poisoned")
        .as_ref()
        .map(|s| s.hp_offset)
        .unwrap_or(0.0)
}

pub fn get_balance_state() -> Option<BalanceState> {
    balance_state_storage()
        .lock()
        .expect("balance state mutex poisoned")
        .clone()
}

fn set_balance_state(state: Option<BalanceState>) {
    let mut guard = balance_state_storage()
        .lock()
        .expect("balance state mutex poisoned");
    *guard = state;
}

fn get_first_monster_kind_from_spawn_table(
    gs: &crate::game_state::GameState,
) -> Option<crate::game_state::monster::MonsterKind> {
    let health_multiplier = gs.stage_modifiers.get_enemy_health_multiplier();
    let (monster_queue, _) = super::super::monster_spawn::monster_queue_table(
        gs.stage,
        gs.route.clone(),
        gs.now(),
        health_multiplier,
    );
    monster_queue.front().map(|monster| monster.kind)
}

/// Runs: snapshot -> place expected tower -> expected upgrade -> spiral place -> defense
/// Then checks if all monsters are killed and adjusts hp_offset accordingly
pub struct MonsterHpBalanceButton {
    pub width: Px,
}

impl Component for MonsterHpBalanceButton {
    fn render(self, ctx: &RenderCtx) {
        let display_text = get_balance_state()
            .map(|s| s.display_text.clone())
            .unwrap_or_else(|| "HP Balance".to_string());

        ctx.add(
            Button::new(
                Wh::new(self.width, 44.px()),
                &|| {
                    mutate_game_state(|gs| {
                        set_balance_state(None);
                        run_hp_balance_procedure(gs);
                    });
                },
                &|wh, text_color, ctx| {
                    ctx.add(memoized_text(
                        (&text_color, &display_text, &wh),
                        |mut builder| {
                            builder
                                .paragraph()
                                .color(text_color)
                                .text(display_text.clone())
                                .render_center(wh)
                        },
                    ));
                },
            )
            .variant(ButtonVariant::Contained),
        );
    }
}

fn run_hp_balance_procedure(gs: &mut crate::game_state::GameState) {
    // Initialize balance state if needed
    if get_balance_state().is_none() {
        // Get first monster kind from spawn table
        let first_monster_kind = get_first_monster_kind_from_spawn_table(gs);
        if let Some(kind) = first_monster_kind {
            let base_max_hp = crate::game_state::monster::MonsterTemplate::get_base_max_hp(kind);
            let increment = base_max_hp * 0.1; // 10% of base max_hp
            set_balance_state(Some(BalanceState {
                hp_offset: 0.0,
                increment,
                all_killed_last_time: None,
                display_text: format!("HP Balance: offset={}, increment={:.2}", 0.0, increment),
                snapshot_index: 0,
                defense_start_hp: 100.0,
                needs_adjustment: false,
                base_max_hp,
            }));
        } else {
            return;
        }
    }

    let state = get_balance_state().unwrap();

    // Check if we should stop
    if state.increment <= state.base_max_hp * 0.01 {
        return;
    }

    // Step 1: Save snapshot and track index
    state_snapshot::save_snapshot_from_state(gs);
    let snapshot_idx = state_snapshot::list_snapshots().len() - 1;

    // Step 2: Place expected tower
    let expected_tower_kind = get_expected_tower_for_stage(gs.stage);
    let template = TowerTemplate::new(expected_tower_kind, Suit::Spades, Rank::Ace);
    gs.goto_placing_tower(template);

    // Step 3: Apply expected upgrade
    let (expected_rarity, expected_category) = get_expected_upgrade_for_stage(gs.stage);
    let upgrade = if expected_category == UpgradeCategory::Random {
        crate::game_state::upgrade::generate_upgrade(gs, expected_rarity)
    } else {
        let kind = expected_category.generate_upgrade_kind(expected_rarity);
        let value = thread_rng().gen_range(0.0..=1.0);
        Upgrade {
            kind,
            rarity: expected_rarity,
            value: value.into(),
        }
    };
    gs.upgrade_state.upgrade(upgrade);

    // Step 4: Place towers in spiral
    place_selected_tower_in_spiral(gs);

    // Step 5: Go to defense
    gs.goto_defense();

    // Store the hp before defense starts for later comparison
    let mut state = get_balance_state().unwrap();
    state.defense_start_hp = gs.hp;
    state.snapshot_index = snapshot_idx;
    state.needs_adjustment = true;
    state.display_text = format!(
        "HP Balance: Running... offset={:.2}, increment={:.2}",
        state.hp_offset, state.increment
    );
    set_balance_state(Some(state));
}

pub fn check_and_adjust_hp_balance(gs: &mut crate::game_state::GameState) {
    let Some(mut state) = get_balance_state() else {
        return;
    };

    // Prevent duplicate execution in the same round
    if !state.needs_adjustment {
        return;
    }
    state.needs_adjustment = false;
    set_balance_state(Some(state.clone()));

    if state.increment <= state.base_max_hp * 0.01 {
        // Done
        let final_hp = state.base_max_hp + state.hp_offset;
        state.display_text = format!("HP Balance: DONE - final_hp={:.2}", final_hp);
        set_balance_state(Some(state));
        return;
    }

    let all_monsters_killed = gs.hp == state.defense_start_hp;

    match state.all_killed_last_time {
        None => {
            // First run
            state.all_killed_last_time = Some(all_monsters_killed);
            if all_monsters_killed {
                state.hp_offset += state.increment;
            } else {
                state.hp_offset -= state.increment;
            }
        }
        Some(was_all_killed) => {
            // Check if outcome changed
            if was_all_killed != all_monsters_killed {
                // Outcome changed, halve the increment
                state.increment *= 0.5;
                state.all_killed_last_time = Some(all_monsters_killed);
            }

            // Adjust offset based on current outcome
            if all_monsters_killed {
                state.hp_offset += state.increment;
            } else {
                state.hp_offset -= state.increment;
            }
        }
    }

    state.display_text = format!(
        "HP Balance: offset={:.2}, increment={:.2}",
        state.hp_offset, state.increment
    );

    // Restore hp to 100
    gs.hp = 100.0;

    // Continue procedure if increment is still large enough
    if state.increment > state.base_max_hp * 0.01 {
        let snapshot_index = state.snapshot_index;
        set_balance_state(Some(state));

        // Restore to the saved snapshot before continuing
        state_snapshot::restore_snapshot(snapshot_index);
        mutate_game_state(|gs| {
            run_hp_balance_procedure(gs);
        });
    } else {
        let final_hp = state.base_max_hp + state.hp_offset;
        state.display_text = format!("HP Balance: DONE - final_hp={:.2}", final_hp);
        set_balance_state(Some(state));
        gs.stage += 1;
        gs.goto_next_stage();
    }
}

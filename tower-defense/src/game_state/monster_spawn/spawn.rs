use crate::game_state::*;
use crate::route::Route;
use std::collections::VecDeque;
use std::sync::Arc;

pub fn start_spawn(game_state: &mut GameState) {
    if game_state.monster_spawn_state.is_spawning() {
        return;
    }

    let health_multipliers = game_state.stage_modifiers.enemy_health_multipliers();
    let sim_tick = game_state.sim_tick();
    let (monster_queue, spawn_interval) = monster_queue_table(
        game_state.stage,
        game_state.route.clone(),
        sim_tick,
        health_multipliers,
        &game_state.config,
    );

    game_state.monster_spawn_state.monster_queue = monster_queue;
    game_state.monster_spawn_state.spawn_interval = spawn_interval;
    game_state.monster_spawn_state.next_spawn_tick = Some(sim_tick);
}

pub fn tick(game_state: &mut GameState, sim_tick: SimTick) {
    if let Some(next_time) = game_state.monster_spawn_state.next_spawn_tick
        && sim_tick < next_time
    {
        return;
    }

    let Some(mut next_monster) = game_state.monster_spawn_state.monster_queue.pop_front() else {
        game_state.monster_spawn_state.next_spawn_tick = None;
        return;
    };

    for skill in next_monster.skills.iter_mut() {
        skill.last_used_at = sim_tick;
    }

    #[cfg(feature = "debug-tools")]
    {
        let hp_offset = crate::game_state::debug_tools::monster_hp_balance::get_hp_offset();
        let hp_offset = crate::Health::from_f64(hp_offset as f64).unwrap_or(crate::Health::ZERO);
        next_monster.max_hp = next_monster.max_hp.saturating_add(hp_offset);
        next_monster.hp = next_monster.max_hp;
    }

    game_state.monsters.push(next_monster);
    game_state.on_enemy_spawned();

    game_state.monster_spawn_state.next_spawn_tick =
        Some(sim_tick + game_state.monster_spawn_state.spawn_interval);
}

pub fn monster_queue_table(
    stage: usize,
    route: Arc<Route>,
    sim_tick: SimTick,
    health_multipliers: &crate::RatioProduct,
    config: &crate::config::GameConfig,
) -> (VecDeque<Monster>, SimTickSpan) {
    let (template_queue, spawn_interval) = monster_template_queue_table(stage, config);

    let monster_queue = template_queue
        .into_iter()
        .map(|template| Monster::new(&template, route.clone(), sim_tick, health_multipliers))
        .collect();

    (monster_queue, spawn_interval)
}

pub fn monster_template_queue_table(
    stage: usize,
    config: &crate::config::GameConfig,
) -> (VecDeque<MonsterTemplate>, SimTickSpan) {
    let spawn_interval =
        SimTickSpan::from_millis_ceil((10000.0 / (26.0 * (stage as f32 / 50.0) + 4.0)) as u64);

    let stage_wave = config
        .monsters
        .stage_waves
        .iter()
        .find(|wave| wave.stage == stage)
        .expect("missing stage wave for stage");

    let template_queue = stage_wave
        .entries
        .iter()
        .flat_map(|entry| std::iter::repeat_n(entry.kind, entry.count))
        .map(|kind| MonsterTemplate::new_with_config(kind, config))
        .collect::<VecDeque<_>>();

    (template_queue, spawn_interval)
}

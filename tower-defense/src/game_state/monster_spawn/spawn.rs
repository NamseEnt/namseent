use crate::game_state::*;
use crate::route::Route;
use std::collections::VecDeque;
use std::sync::Arc;

pub fn start_spawn(game_state: &mut GameState) {
    if game_state.monster_spawn_state.is_spawning() {
        return;
    }

    let health_multiplier = game_state.stage_modifiers.get_enemy_health_multiplier();
    let now = game_state.now();
    let (monster_queue, spawn_interval) = monster_queue_table(
        game_state.stage,
        game_state.route.clone(),
        now,
        health_multiplier,
        &game_state.config,
    );

    game_state.monster_spawn_state.monster_queue = monster_queue;
    game_state.monster_spawn_state.spawn_interval = spawn_interval;
    game_state.monster_spawn_state.next_spawn_time = Some(now);
}

pub fn tick(game_state: &mut GameState, now: namui::Instant) {
    if let Some(next_time) = game_state.monster_spawn_state.next_spawn_time
        && now < next_time
    {
        return;
    }

    let Some(mut next_monster) = game_state.monster_spawn_state.monster_queue.pop_front() else {
        game_state.monster_spawn_state.next_spawn_time = None;
        return;
    };

    for skill in next_monster.skills.iter_mut() {
        skill.last_used_at = now;
    }

    #[cfg(feature = "debug-tools")]
    {
        let hp_offset = crate::game_state::debug_tools::monster_hp_balance::get_hp_offset();
        next_monster.max_hp += hp_offset;
        next_monster.hp = next_monster.max_hp;
    }

    game_state.monsters.push(next_monster);
    game_state.on_enemy_spawned();

    game_state.monster_spawn_state.next_spawn_time =
        Some(now + game_state.monster_spawn_state.spawn_interval);
}

pub fn monster_queue_table(
    stage: usize,
    route: Arc<Route>,
    now: namui::Instant,
    health_multiplier: f32,
    config: &crate::config::GameConfig,
) -> (VecDeque<Monster>, namui::Duration) {
    let (template_queue, spawn_interval) = monster_template_queue_table(stage, config);

    let monster_queue = template_queue
        .into_iter()
        .map(|template| Monster::new(&template, route.clone(), now, health_multiplier))
        .collect();

    (monster_queue, spawn_interval)
}

pub fn monster_template_queue_table(
    stage: usize,
    config: &crate::config::GameConfig,
) -> (VecDeque<MonsterTemplate>, namui::Duration) {
    let spawn_interval =
        namui::Duration::from_millis((10000.0 / (26.0 * (stage as f32 / 50.0) + 4.0)) as i64);

    let stage_wave = config
        .monsters
        .stage_waves
        .get(&stage)
        .expect("missing stage wave for stage");

    let template_queue = stage_wave
        .entries
        .iter()
        .flat_map(|(kind, count)| std::iter::repeat_n(*kind, *count))
        .map(|kind| MonsterTemplate::new_with_config(kind, config))
        .collect::<VecDeque<_>>();

    (template_queue, spawn_interval)
}

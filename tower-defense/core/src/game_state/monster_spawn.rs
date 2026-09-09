use crate::{CoreState, MonsterState, MoveOnRouteState, SimTickSpan, apply_ratio_product_raw};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterSpawnState {
    pub monster_queue: Vec<MonsterState>,
    pub next_spawn_tick: Option<u64>,
    pub spawn_interval_ticks: u64,
}

pub fn calculate_stage_total_hp_raw(
    stage: usize,
    config: &crate::GameConfig,
    enemy_health_multipliers_raw: &[i64],
) -> i64 {
    let wave = config
        .monsters
        .stage_waves
        .iter()
        .find(|wave| wave.stage == stage)
        .expect("missing stage wave for stage");

    wave.entries
        .iter()
        .map(|entry| {
            let stats = config
                .monsters
                .stats
                .iter()
                .find(|stats| stats.kind == entry.kind)
                .expect("missing monster stats for kind");
            apply_ratio_product_raw(stats.base_hp_raw, enemy_health_multipliers_raw)
                .saturating_mul(entry.count.min(i64::MAX as usize) as i64)
        })
        .fold(0, i64::saturating_add)
}

pub(crate) fn start_spawn(state: &mut CoreState) {
    if state.monster_spawn.next_spawn_tick.is_some() {
        return;
    }

    let wave = state
        .config
        .monsters
        .stage_waves
        .iter()
        .find(|wave| wave.stage == state.progress.stage)
        .expect("missing stage wave for stage");
    let spawn_interval_ticks = SimTickSpan::from_millis_ceil(
        (10_000.0 / (26.0 * (state.progress.stage as f32 / 50.0) + 4.0)) as u64,
    )
    .ticks();
    let enemy_health_multipliers_raw = &state.stage_modifiers.enemy_health_multipliers_raw;
    let mut monster_queue = Vec::new();

    for entry in &wave.entries {
        let stats = state
            .config
            .monsters
            .stats
            .iter()
            .find(|stats| stats.kind == entry.kind)
            .expect("missing monster stats for kind");
        let max_hp_raw = apply_ratio_product_raw(stats.base_hp_raw, enemy_health_multipliers_raw);
        let velocity_raw = apply_ratio_product_raw(
            5 * crate::WORLD_UNITS_PER_TILE,
            std::slice::from_ref(&stats.velocity_mul_raw),
        );
        for _ in 0..entry.count {
            let id = state.next_entity_id.allocate_raw();
            let start = *state
                .route
                .world_coords
                .first()
                .expect("monster route must have a start point");
            monster_queue.push(MonsterState {
                id,
                move_on_route: MoveOnRouteState {
                    route: state.route.clone(),
                    route_index: 0,
                    route_progress_raw: 0,
                    map_coord: start,
                    velocity_raw,
                    movement_remainder: 0,
                    motion_revision: 0,
                },
                kind: entry.kind,
                hp_raw: max_hp_raw,
                max_hp_raw,
                stage_progress_counted: false,
                skills: Vec::new(),
                status_effects: Vec::new(),
                damage_raw: stats.damage_raw,
                reward: stats.reward,
            });
        }
    }

    state.monster_spawn = MonsterSpawnState {
        monster_queue,
        next_spawn_tick: Some(state.sim_tick.ticks()),
        spawn_interval_ticks,
    };
}

pub(crate) fn spawn_due(state: &mut CoreState) -> bool {
    if let Some(next_spawn_tick) = state.monster_spawn.next_spawn_tick
        && state.sim_tick.ticks() < next_spawn_tick
    {
        return false;
    }

    let Some(mut monster) = state.monster_spawn.monster_queue.first().cloned() else {
        state.monster_spawn.next_spawn_tick = None;
        return false;
    };
    state.monster_spawn.monster_queue.remove(0);
    for skill in &mut monster.skills {
        skill.last_used_at = state.sim_tick.ticks();
    }
    state.push_event(crate::CoreEvent::MonsterSpawned {
        monster_id: monster.id,
        monster_kind: monster.kind,
        position: monster.move_on_route.map_coord,
    });
    state.monsters.push(monster);
    state.monster_spawn.next_spawn_tick = Some(
        state
            .sim_tick
            .saturating_add(SimTickSpan::from_ticks(
                state.monster_spawn.spawn_interval_ticks,
            ))
            .ticks(),
    );
    true
}

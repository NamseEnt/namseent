use crate::SimTickSpan;
#[cfg(feature = "debug-tools")]
use crate::game_state::Monster;
use crate::game_state::MonsterTemplate;
#[cfg(feature = "debug-tools")]
use crate::route::Route;
use std::collections::VecDeque;
#[cfg(feature = "debug-tools")]
use std::sync::Arc;

#[cfg(feature = "debug-tools")]
pub(crate) fn monster_queue_table(
    stage: usize,
    route: Arc<Route>,
    sim_tick: crate::SimTick,
    health_multipliers: &crate::RatioProduct,
    config: &crate::config::GameConfig,
    allocator: &mut super::super::entity_id::EntityIdAllocator,
) -> (VecDeque<Monster>, SimTickSpan) {
    let (template_queue, spawn_interval) = monster_template_queue_table(stage, config);

    let monster_queue = template_queue
        .into_iter()
        .map(|template| {
            let id = crate::MonsterId::from_raw(allocator.allocate_raw());
            Monster::new_with_id(&template, route.clone(), sim_tick, health_multipliers, id)
        })
        .collect::<VecDeque<_>>();

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
        .map(|kind| MonsterTemplate::new(kind, config))
        .collect::<VecDeque<_>>();

    (template_queue, spawn_interval)
}

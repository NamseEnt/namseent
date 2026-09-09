use crate::SimTickSpan;
use crate::game_state::MonsterTemplate;
use std::collections::VecDeque;

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
        .filter_map(crate::game_state::monster::MonsterKind::from_core_raw)
        .map(|kind| MonsterTemplate::new(kind, config))
        .collect::<VecDeque<_>>();

    (template_queue, spawn_interval)
}

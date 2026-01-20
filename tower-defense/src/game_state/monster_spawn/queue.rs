use crate::game_state::MonsterKind;
use crate::game_state::monster_spawn::NamedMonsterConfig;
use std::collections::VecDeque;

pub fn append_named_to_queue(
    monster_queue: &mut VecDeque<MonsterKind>,
    named_queue: &mut VecDeque<NamedMonsterConfig>,
    extras: &[NamedMonsterConfig],
) {
    if extras.is_empty() {
        return;
    }

    let mut seen = monster_queue
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    for config in extras {
        if seen.insert(config.kind) {
            monster_queue.push_back(config.kind);
            named_queue.push_back(config.clone());
        }
    }
}

use crate::game_state::MonsterKind;
use std::collections::{HashSet, VecDeque};

pub fn append_named_to_queue(queue: &mut VecDeque<MonsterKind>, extras: &[MonsterKind]) {
    if extras.is_empty() {
        return;
    }

    let mut seen: HashSet<MonsterKind> = queue.iter().copied().collect();
    for kind in extras {
        if seen.insert(*kind) {
            queue.push_back(*kind);
        }
    }
}

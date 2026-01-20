use crate::game_state::{Monster, MonsterTemplate};
use crate::route::Route;
use namui::Instant;
use std::collections::VecDeque;
use std::sync::Arc;

pub fn append_named_to_queue(
    monster_queue: &mut VecDeque<Monster>,
    extras: &[MonsterTemplate],
    route: Arc<Route>,
    now: Instant,
    health_multiplier: f32,
) {
    if extras.is_empty() {
        return;
    }

    let mut seen = monster_queue
        .iter()
        .map(|monster| monster.kind)
        .collect::<std::collections::HashSet<_>>();
    for template in extras {
        if seen.insert(template.kind) {
            let monster = Monster::new(template, route.clone(), now, health_multiplier);
            monster_queue.push_back(monster);
        }
    }
}

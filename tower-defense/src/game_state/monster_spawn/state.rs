use crate::game_state::*;
use std::collections::VecDeque;

#[derive(State, Clone)]
pub struct MonsterSpawnState {
    pub monster_queue: VecDeque<Monster>,
    pub next_spawn_time: Option<Instant>,
    pub spawn_interval: namui::Duration,
}

impl MonsterSpawnState {
    pub fn idle() -> Self {
        Self {
            monster_queue: VecDeque::new(),
            next_spawn_time: None,
            spawn_interval: namui::Duration::from_millis(0),
        }
    }

    pub fn is_spawning(&self) -> bool {
        self.next_spawn_time.is_some()
    }

    pub fn is_idle(&self) -> bool {
        self.next_spawn_time.is_none() && self.monster_queue.is_empty()
    }
}

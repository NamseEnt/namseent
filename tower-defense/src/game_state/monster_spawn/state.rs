use crate::game_state::*;
use std::collections::VecDeque;

#[derive(State, Clone)]
pub struct MonsterSpawnState {
    pub monster_queue: VecDeque<Monster>,
    pub next_spawn_tick: Option<SimTick>,
    pub spawn_interval: SimTickSpan,
}

impl MonsterSpawnState {
    pub fn idle() -> Self {
        Self {
            monster_queue: VecDeque::new(),
            next_spawn_tick: None,
            spawn_interval: SimTickSpan::ZERO,
        }
    }

    pub fn is_spawning(&self) -> bool {
        self.next_spawn_tick.is_some()
    }

    pub fn is_idle(&self) -> bool {
        self.next_spawn_tick.is_none() && self.monster_queue.is_empty()
    }
}

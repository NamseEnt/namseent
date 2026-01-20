use crate::game_state::*;
use std::collections::VecDeque;

#[derive(State, Clone)]
pub struct MonsterSpawnState {
    pub monster_queue: VecDeque<MonsterKind>,
    pub next_spawn_time: Option<Instant>,
    pub spawn_interval: namui::Duration,
    pub challenge_choices: [MonsterKind; 3],
    pub challenge_selected: [bool; 3],
}

impl MonsterSpawnState {
    pub fn idle() -> Self {
        Self {
            monster_queue: VecDeque::new(),
            next_spawn_time: None,
            spawn_interval: namui::Duration::from_millis(0),
            challenge_choices: [MonsterKind::Named01; 3],
            challenge_selected: [false; 3],
        }
    }

    pub fn is_spawning(&self) -> bool {
        self.next_spawn_time.is_some()
    }

    pub fn is_idle(&self) -> bool {
        self.next_spawn_time.is_none() && self.monster_queue.is_empty()
    }

    pub fn reset_challenge_selection(&mut self) {
        self.challenge_selected = [false; 3];
    }

    pub fn toggle_challenge_selection(&mut self, index: usize) {
        if index < 3 {
            self.challenge_selected[index] = !self.challenge_selected[index];
        }
    }
}

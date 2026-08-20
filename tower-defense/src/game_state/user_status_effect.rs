use crate::{FixedRatio, SimTick};
use namui::*;

use super::GameState;

#[derive(Debug, State, Clone)]
pub struct UserStatusEffect {
    pub kind: UserStatusEffectKind,
    pub end_at: SimTick,
}

#[derive(Debug, State, Clone)]
pub enum UserStatusEffectKind {
    DamageReduction { damage_multiply: FixedRatio },
}

pub fn remove_user_finished_status_effects(game_state: &mut GameState, sim_tick: SimTick) {
    game_state
        .user_status_effects
        .retain(|e| sim_tick < e.end_at);
}

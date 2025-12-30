use namui::*;

use super::GameState;

#[derive(State, Clone)]
pub struct UserStatusEffect {
    pub kind: UserStatusEffectKind,
    pub end_at: Instant,
}

#[derive(State, Clone)]
pub enum UserStatusEffectKind {
    DamageReduction { damage_multiply: f32 },
}

pub fn remove_user_finished_status_effects(game_state: &mut GameState, now: Instant) {
    game_state.user_status_effects.retain(|e| now < e.end_at);
}

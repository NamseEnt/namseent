use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{HighProjectile, NewAttackParticle},
};
use namui::Instant;

pub enum AttackVisualEffect {
    HighProjectile {
        start_xy: (f32, f32),
        end_xy: (f32, f32),
    },
}

pub fn spawn_tower_attack_visual_effect(effect: AttackVisualEffect, created_at: Instant) {
    match effect {
        AttackVisualEffect::HighProjectile { start_xy, end_xy } => {
            ATTACK_VISUAL.spawn(NewAttackParticle::HighProjectile(HighProjectile::new(
                start_xy, end_xy, created_at,
            )));
        }
    }
}

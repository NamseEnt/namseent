mod high;
mod one_pair;
mod two_pair;

use namui::Instant;

pub enum AttackVisualEffect {
    High {
        start_xy: (f32, f32),
        end_xy: (f32, f32),
    },
    OnePair {
        start_xy: (f32, f32),
        end_xy: (f32, f32),
    },
    TwoPair {
        start_xy: (f32, f32),
        end_xy: (f32, f32),
    },
}

pub fn spawn_tower_attack_visual_effect(effect: AttackVisualEffect, created_at: Instant) {
    match effect {
        AttackVisualEffect::High { start_xy, end_xy } => {
            high::spawn(start_xy, end_xy, created_at);
        }
        AttackVisualEffect::OnePair { start_xy, end_xy } => {
            one_pair::spawn(start_xy, end_xy, created_at);
        }
        AttackVisualEffect::TwoPair { start_xy, end_xy } => {
            two_pair::spawn(start_xy, end_xy, created_at);
        }
    }
}

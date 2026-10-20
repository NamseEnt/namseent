mod common;
mod high;
mod one_pair;
mod three_of_a_kind;
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
    ThreeOfAKind {
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        attack_range_radius: f32,
    },
    ThreeOfAKindHit {
        start_xy: (f32, f32),
        impact_xy: (f32, f32),
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
        AttackVisualEffect::ThreeOfAKind {
            start_xy,
            end_xy,
            attack_range_radius,
        } => {
            three_of_a_kind::spawn(start_xy, end_xy, attack_range_radius, created_at);
        }
        AttackVisualEffect::ThreeOfAKindHit {
            start_xy,
            impact_xy,
        } => three_of_a_kind::spawn_hit_effect(start_xy, impact_xy, created_at),
    }
}

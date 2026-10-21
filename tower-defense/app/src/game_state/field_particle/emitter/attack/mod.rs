mod common;
mod flush;
mod full_house;
mod high;
mod one_pair;
mod straight;
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
    Straight {
        start_xy: (f32, f32),
        end_xy: (f32, f32),
    },
    StraightHit {
        start_xy: (f32, f32),
        impact_xy: (f32, f32),
    },
    Flush {
        start_xy: (f32, f32),
        target_xy: (f32, f32),
    },
    FlushHit {
        start_xy: (f32, f32),
        impact_xy: (f32, f32),
    },
    FullHouse {
        start_xy: (f32, f32),
        center_xy: (f32, f32),
        attack_range_radius: f32,
    },
    FullHouseHit {
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
        AttackVisualEffect::Straight { start_xy, end_xy } => {
            straight::spawn(start_xy, end_xy, created_at);
        }
        AttackVisualEffect::StraightHit {
            start_xy,
            impact_xy,
        } => straight::spawn_hit_effect(start_xy, impact_xy, created_at),
        AttackVisualEffect::Flush {
            start_xy,
            target_xy,
        } => flush::spawn(start_xy, target_xy, created_at),
        AttackVisualEffect::FlushHit {
            start_xy,
            impact_xy,
        } => flush::spawn_hit_effect(start_xy, impact_xy, created_at),
        AttackVisualEffect::FullHouse {
            start_xy,
            center_xy,
            attack_range_radius,
        } => full_house::spawn(start_xy, center_xy, attack_range_radius, created_at),
        AttackVisualEffect::FullHouseHit { impact_xy } => {
            full_house::spawn_hit_effect(impact_xy, created_at)
        }
    }
}

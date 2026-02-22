pub mod instant_effect;
pub mod laser;

use super::projectile::{ProjectileKind, ProjectileTrail};
use instant_effect::{TargetHitEffect, TowerEmitEffect};
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileHitEffect {
    TrashBounce,
    CardBurst,
    SparkleBurst,
    HeartBurst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectileGroup {
    Trash,
    Girl,
    Cards,
    Heart,
}

impl ProjectileGroup {
    pub fn random_kind(&self) -> ProjectileKind {
        match self {
            Self::Trash => ProjectileKind::random_trash(),
            Self::Girl => ProjectileKind::random_girl(),
            Self::Cards => ProjectileKind::random_cards(),
            Self::Heart => ProjectileKind::random_heart(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttackType {
    Projectile {
        speed: Per<f32, Duration>,
        trail: ProjectileTrail,
        projectile_group: ProjectileGroup,
        hit_effect: ProjectileHitEffect,
    },
    Laser,
    InstantEffect {
        emit_effect: TowerEmitEffect,
        hit_effect: TargetHitEffect,
    },
    FullHouseRain {
        tower_xy: (f32, f32),
        target_xy: (f32, f32),
    },
}

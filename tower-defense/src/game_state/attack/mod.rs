pub mod instant_effect;
pub mod laser;

use super::projectile::{ProjectileKind, ProjectileTrail};
use instant_effect::{TargetHitEffect, TowerEmitEffect};
use namui::*;

/// 투사체 피격 시 효과
#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileHitEffect {
    TrashBounce,
    CardBurst,
    SparkleBurst,
}

/// 투사체 그룹 - 투사체 종류를 분류
#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum ProjectileGroup {
    Trash,
    Girl,
    Cards,
}

impl ProjectileGroup {
    pub fn random_kind(&self) -> ProjectileKind {
        match self {
            Self::Trash => ProjectileKind::random_trash(),
            Self::Girl => ProjectileKind::random_girl(),
            Self::Cards => ProjectileKind::random_cards(),
        }
    }
}

/// 타워가 사용할 수 있는 공격 방식
#[derive(Debug, Clone, PartialEq, State)]
pub enum AttackType {
    /// 투사체: 발사 후 적에게 날아가서 데미지
    Projectile {
        speed: Per<f32, Duration>,
        trail: ProjectileTrail,
        projectile_group: ProjectileGroup,
        hit_effect: ProjectileHitEffect,
    },
    /// 레이저 광선: 즉시 데미지 + 잔상 이펙트
    Laser,
    /// 즉시 이펙트: 타워 위치 → 적 위치에 이펙트 생성 + 즉시 데미지
    InstantEffect {
        emit_effect: TowerEmitEffect,
        hit_effect: TargetHitEffect,
    },
    /// FullHouse 이펙트: 하늘에서 trash가 떨어짐
    FullHouseRain {
        tower_xy: (f32, f32),
        target_xy: (f32, f32),
    },
}

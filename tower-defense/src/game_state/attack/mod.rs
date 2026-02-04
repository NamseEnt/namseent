pub mod instant_effect;
pub mod laser;

use namui::*;

/// 타워가 사용할 수 있는 공격 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, State, Default)]
pub enum AttackType {
    /// 투사체: 발사 후 적에게 날아가서 데미지
    #[default]
    Projectile,
    /// 빠른 투사체: 고속 이동 + 불타는 잔상 이펙트
    BurningProjectile,
    /// 레이저 광선: 즉시 데미지 + 잔상 이펙트
    Laser,
    /// 즉시 이펙트: 타워 위치 → 적 위치에 이펙트 생성 + 즉시 데미지
    InstantEffect,
}

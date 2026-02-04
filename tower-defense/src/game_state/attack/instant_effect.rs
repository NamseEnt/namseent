use namui::*;

/// 이펙트 수명
pub const EFFECT_LIFETIME: Duration = Duration::from_millis(400);

/// 이펙트 종류
#[derive(Clone, Copy, PartialEq, Eq, Debug, State)]
pub enum InstantEffectKind {
    /// 폭발 이펙트
    Explosion,
    /// 번개 이펙트
    Lightning,
    /// 마법진 이펙트
    MagicCircle,
    /// FullHouse 이펙트 - 하늘에서 trash가 떨어짐
    FullHouseRain,
    /// FullHouse 이펙트 - 하늘로 trash가 솟구침
    FullHouseBurst,
}

/// 타워 위치에서 시작되는 발사 이펙트
#[derive(Clone, Debug, PartialEq, State)]
pub struct TowerEmitEffect {
    /// 타워 위치
    pub tower_xy: (f32, f32),
    /// 대상 위치
    pub target_xy: (f32, f32),
    /// 생성 시간
    pub created_at: Instant,
    /// 이펙트 종류
    pub kind: InstantEffectKind,
}

/// 적 위치에 생성되는 히트 이펙트
#[derive(Clone, Debug, PartialEq, State)]
pub struct TargetHitEffect {
    /// 이펙트 위치
    pub xy: (f32, f32),
    /// 생성 시간
    pub created_at: Instant,
    /// 이펙트 종류
    pub kind: InstantEffectKind,
    /// 이펙트 크기 배율
    pub scale: f32,
}

impl TowerEmitEffect {
    pub fn new(
        tower_xy: (f32, f32),
        target_xy: (f32, f32),
        created_at: Instant,
        kind: InstantEffectKind,
    ) -> Self {
        Self {
            tower_xy,
            target_xy,
            created_at,
            kind,
        }
    }

    /// 이펙트 진행도 (0.0 ~ 1.0)
    pub fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / EFFECT_LIFETIME.as_secs_f32()).min(1.0)
    }

    pub fn is_expired(&self, now: Instant) -> bool {
        now - self.created_at >= EFFECT_LIFETIME
    }
}

impl TargetHitEffect {
    pub fn new(xy: (f32, f32), created_at: Instant, kind: InstantEffectKind, scale: f32) -> Self {
        Self {
            xy,
            created_at,
            kind,
            scale,
        }
    }

    /// 이펙트 진행도 (0.0 ~ 1.0)
    pub fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / EFFECT_LIFETIME.as_secs_f32()).min(1.0)
    }

    /// 현재 크기 배율 (시작 시 커졌다가 줄어듦)
    pub fn current_scale(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        // 처음에 빠르게 커지고, 이후 천천히 사라짐
        if progress < 0.2 {
            self.scale * (progress / 0.2)
        } else {
            self.scale * (1.0 - (progress - 0.2) / 0.8)
        }
    }

    /// 현재 투명도
    pub fn current_alpha(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        if progress < 0.1 {
            progress / 0.1
        } else {
            1.0 - (progress - 0.1) / 0.9
        }
    }

    pub fn is_expired(&self, now: Instant) -> bool {
        now - self.created_at >= EFFECT_LIFETIME
    }
}

use namui::*;

/// 레이저 광선의 수명
pub const LASER_LIFETIME: Duration = Duration::from_millis(500);

#[derive(Clone, State)]
pub struct LaserBeam {
    /// 레이저 시작점 (타워 위치)
    pub start_xy: (f32, f32),
    /// 레이저 끝점 (발사 시점의 적 위치)
    pub end_xy: (f32, f32),
    /// 레이저가 생성된 시간
    pub created_at: Instant,
    /// 데미지를 적용할 몬스터 ID. None이면 이미 사망한 것으로 간주.
    pub target_monster_id: usize,
}

impl LaserBeam {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        target_monster_id: usize,
    ) -> Self {
        Self {
            start_xy,
            end_xy,
            created_at,
            target_monster_id,
        }
    }

    /// 레이저의 현재 투명도 (페이드아웃)
    pub fn current_alpha(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        if elapsed >= LASER_LIFETIME {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / LASER_LIFETIME.as_secs_f32();
        1.0 - progress
    }

    /// 레이저가 만료되었는지 확인
    pub fn is_expired(&self, now: Instant) -> bool {
        now - self.created_at >= LASER_LIFETIME
    }
}

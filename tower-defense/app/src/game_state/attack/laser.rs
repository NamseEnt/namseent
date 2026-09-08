use crate::{MonsterId, SimTick, SimTickSpan, WorldCoord};
use namui::*;

/// 레이저 광선의 수명
pub const LASER_LIFETIME: SimTickSpan = SimTickSpan::from_millis_ceil(500);

#[derive(Clone, State)]
pub struct LaserBeam {
    /// 레이저 시작점 (타워 위치)
    pub start_xy: WorldCoord,
    /// 레이저 끝점 (발사 시점의 적 위치)
    pub end_xy: WorldCoord,
    /// 레이저가 생성된 시간
    pub created_at: SimTick,
    /// 데미지를 적용할 몬스터 ID.
    pub target_monster_id: MonsterId,
}

impl LaserBeam {
    pub fn new(
        start_xy: WorldCoord,
        end_xy: WorldCoord,
        created_at: SimTick,
        target_monster_id: MonsterId,
    ) -> Self {
        Self {
            start_xy,
            end_xy,
            created_at,
            target_monster_id,
        }
    }

    pub(crate) fn to_core_state(&self) -> td_core::LaserAttackState {
        td_core::LaserAttackState {
            start_xy: [self.start_xy.x, self.start_xy.y],
            end_xy: [self.end_xy.x, self.end_xy.y],
            created_at: self.created_at.ticks(),
            target_monster_id: self.target_monster_id.raw(),
        }
    }

    /// 레이저의 현재 투명도 (페이드아웃)
    pub fn current_alpha(&self, sim_tick: SimTick) -> f32 {
        let elapsed = sim_tick - self.created_at;
        if elapsed >= LASER_LIFETIME {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / LASER_LIFETIME.as_secs_f32();
        1.0 - progress
    }

    /// 레이저가 만료되었는지 확인
    pub fn is_expired(&self, sim_tick: SimTick) -> bool {
        sim_tick - self.created_at >= LASER_LIFETIME
    }
}

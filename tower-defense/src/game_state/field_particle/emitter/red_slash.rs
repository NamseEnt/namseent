use crate::game_state::field_particle::{RedSlashParticle, spawn_red_slash};
use namui::*;

const RED_SLASH_MARK_COUNT: usize = 5;
/// 타겟 위치 기준 앞뒤 배치 범위 (타일)
const RED_SLASH_MARK_SPREAD_TILE: f32 = 0.5;
const RED_SLASH_MARK_LIFETIME_MIN_MS: i64 = 100;
const RED_SLASH_MARK_LIFETIME_MAX_MS: i64 = 300;

/// 클론이 대시할 때 타겟 위치를 중심으로 슬래시 흔적을 생성한다.
///
/// - `spawn_xy`: 클론의 출발 위치 (방향 계산에 사용)
/// - `target_xy`: 클론이 관통하는 타겟 위치
/// - 진행 방향의 수직으로 슬래시 이미지를 표시
/// - `spawn_xy` 쪽에 가까운 파티클일수록 수명이 짧아 먼저 사라짐
pub fn spawn_red_slash_marks(spawn_xy: (f32, f32), target_xy: (f32, f32), now: Instant) {
    let dx = target_xy.0 - spawn_xy.0;
    let dy = target_xy.1 - spawn_xy.1;
    let length = (dx * dx + dy * dy).sqrt();
    if length < 1e-6 {
        return;
    }

    let dir_x = dx / length;
    let dir_y = dy / length;
    let dash_angle_rad = dy.atan2(dx);

    for i in 0..RED_SLASH_MARK_COUNT {
        // t: -1.0 (start쪽) ~ +1.0 (end쪽)
        let t = -RED_SLASH_MARK_SPREAD_TILE
            + (2.0 * RED_SLASH_MARK_SPREAD_TILE) * (i as f32) / (RED_SLASH_MARK_COUNT - 1) as f32;

        let xy = (target_xy.0 + dir_x * t, target_xy.1 + dir_y * t);

        // t가 -1 (spawn쪽)이면 짧은 수명, +1 (end쪽)이면 긴 수명
        let lifetime_ms = RED_SLASH_MARK_LIFETIME_MIN_MS
            + ((t + RED_SLASH_MARK_SPREAD_TILE) / (2.0 * RED_SLASH_MARK_SPREAD_TILE)
                * (RED_SLASH_MARK_LIFETIME_MAX_MS - RED_SLASH_MARK_LIFETIME_MIN_MS) as f32)
                .round() as i64;
        let lifetime = Duration::from_millis(lifetime_ms);

        spawn_red_slash(RedSlashParticle::new(xy, dash_angle_rad, now, lifetime));
    }
}

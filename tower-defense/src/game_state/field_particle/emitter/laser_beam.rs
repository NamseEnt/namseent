use crate::game_state::field_particle::FieldParticle;
use namui::*;
use rand::Rng;

// 상수 정의
const LASER_LINE_COUNT: usize = 8; // 생성할 직선 개수
const LINE_THICKNESS_MIN: f32 = 0.1; // 최소 두께 (타일)
const LINE_THICKNESS_MAX: f32 = 0.25; // 최대 두께 (타일)
const LASER_LIFETIME_MS: i64 = 120; // 레이저 수명
const START_OFFSET_RANGE: f32 = 0.9; // start 점 오프셋 범위 (직선 길이 비율)
const END_OFFSET_RANGE: f32 = 0.9; // end 점 오프셋 범위 (직선 길이 비율)
const MOVEMENT_SPEED: f32 = 32.0; // 초당 target 방향으로 이동하는 거리 (타일 단위)

#[derive(Clone, State)]
pub struct LaserBeamEmitter {
    pub start_xy: (f32, f32), // 타워 위치
    pub end_xy: (f32, f32),   // 타겟 위치
    pub created_at: Instant,
    pub emitted: bool,
}

impl LaserBeamEmitter {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Self {
        Self {
            start_xy,
            end_xy,
            created_at,
            emitted: false,
        }
    }
}

impl namui::particle::Emitter<crate::game_state::field_particle::FieldParticle>
    for LaserBeamEmitter
{
    fn emit(
        &mut self,
        now: Instant,
        _dt: Duration,
    ) -> Vec<crate::game_state::field_particle::FieldParticle> {
        if self.emitted {
            return vec![];
        }

        self.emitted = true;
        let mut rng = rand::thread_rng();
        let mut out = Vec::with_capacity(LASER_LINE_COUNT);

        // 직선의 방향 벡터 계산
        let dx = self.end_xy.0 - self.start_xy.0;
        let dy = self.end_xy.1 - self.start_xy.1;

        for i in 0..LASER_LINE_COUNT {
            let (start_t, end_t, thickness) = if i == 0 {
                // 첫 번째 직선은 최대 두께로 시작점과 끝점을 완전히 연결
                (0.0, 1.0, LINE_THICKNESS_MAX)
            } else {
                // 나머지는 랜덤
                let start_t = rng.gen_range(0.0..START_OFFSET_RANGE);
                let end_t = rng.gen_range((1.0 - END_OFFSET_RANGE)..1.0);
                let thickness = rng.gen_range(LINE_THICKNESS_MIN..LINE_THICKNESS_MAX);
                (start_t, end_t, thickness)
            };

            let line_start = (
                self.start_xy.0 + dx * start_t,
                self.start_xy.1 + dy * start_t,
            );
            let line_end = (self.start_xy.0 + dx * end_t, self.start_xy.1 + dy * end_t);

            let particle = crate::game_state::field_particle::LaserLineParticle::new(
                line_start,
                line_end,
                self.end_xy, // clamp용 target 위치
                now,
                Duration::from_millis(LASER_LIFETIME_MS),
                thickness,
                MOVEMENT_SPEED,
            );

            out.push(FieldParticle::LaserLine { particle });
        }

        out
    }

    fn is_done(&self, _now: Instant) -> bool {
        self.emitted
    }
}

use crate::game_state::MonsterKind;
use namui::*;
use rand::{Rng, thread_rng};

const CORPSE_DURATION_SEC: f32 = 2.0;
const INITIAL_SPEED_PX_PER_SEC: f32 = 420.0; // 초기 발사 속도
const GRAVITY_PX_PER_SEC2: f32 = 128.0; // 아래 방향 +Y
const LINEAR_DRAG_PER_SEC: f32 = 5.0; // 강한 공기저항
const ANGULAR_VELOCITY_INIT_DEG_PER_SEC: f32 = 540.0; // 빠른 시작 회전 속도
const ANGULAR_DRAG_PER_SEC: f32 = 3.0;

#[derive(Clone, State)]
pub struct MonsterCorpseParticle {
    pub position: Xy<Px>,
    pub created_at: Instant,
    pub duration: Duration,
    pub rotation: Angle,
    pub angular_velocity: Per<Angle, Duration>,
    pub velocity: Per<Xy<Px>, Duration>,
    pub monster_kind: MonsterKind,
    pub wh: Wh<Px>,
    pub scale: f32,
}

impl MonsterCorpseParticle {
    pub fn new(
        position: Xy<Px>,
        now: Instant,
        rotation: Angle,
        monster_kind: MonsterKind,
        wh: Wh<Px>,
    ) -> Self {
        // 초기 방향: 수직 상단(위쪽, -90deg)을 기준으로 좌/우 45도 랜덤
        let mut rng = thread_rng();
        let offset_deg: f32 = rng.gen_range(-45.0..=45.0);
        let launch_deg: f32 = -90.0 + offset_deg;
        let launch_rad = launch_deg.to_radians();
        let vx = INITIAL_SPEED_PX_PER_SEC * launch_rad.cos();
        let vy = INITIAL_SPEED_PX_PER_SEC * launch_rad.sin();

        // 각속도는 빠르게 시작
        let angular_velocity_deg_per_sec =
            ANGULAR_VELOCITY_INIT_DEG_PER_SEC * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        Self {
            position,
            created_at: now,
            duration: Duration::from_secs_f32(CORPSE_DURATION_SEC),
            rotation,
            angular_velocity: Per::new(angular_velocity_deg_per_sec.deg(), 1.sec()),
            velocity: Per::new(Xy::new(px(vx), px(vy)), 1.sec()),
            monster_kind,
            wh,
            scale: 1.0,
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    pub fn tick(&mut self, now: Instant, delta_time: Duration) {
        let dt = delta_time.as_secs_f32();

        // Scale: 1.0 -> 0.0 (지속시간 동안 선형 감소)
        let elapsed = now - self.created_at;
        let progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.scale = 1.0 - progress;

        // 현재 속도 (px/sec) 를 Px로 환산하여 가감할 값 준비
        let mut delta_position_per_second = self.velocity * 1.sec(); // Xy<Px>

        // 중력 적용 (+Y 방향)
        delta_position_per_second.y += px(GRAVITY_PX_PER_SEC2 * dt);

        // 강한 공기저항 (속도 지수 감쇠)
        let lin_drag = (-LINEAR_DRAG_PER_SEC * dt).exp();
        delta_position_per_second *= lin_drag;

        // 속도 갱신 (다시 Per 로 환원)
        self.velocity = Per::new(delta_position_per_second, 1.sec());

        // 위치 업데이트
        let v = self.velocity * delta_time; // Xy<Px>
        self.position.x += v.x;
        self.position.y += v.y;

        // 각속도 감쇠 및 회전 업데이트
        let ang_drag = (-ANGULAR_DRAG_PER_SEC * dt).exp();
        let mut delta_rotation_per_second = self.angular_velocity * 1.sec(); // Angle
        delta_rotation_per_second *= ang_drag;
        self.angular_velocity = Per::new(delta_rotation_per_second, 1.sec());
        self.rotation += self.angular_velocity * delta_time;
    }

    pub fn render(&self) -> RenderingTree {
        let Self {
            rotation,
            monster_kind,
            wh,
            scale,
            ..
        } = self;

        let image = monster_kind.image();

        namui::translate(
            self.position.x,
            self.position.y,
            namui::rotate(
                *rotation,
                namui::scale(
                    *scale,
                    *scale,
                    namui::image(ImageParam {
                        rect: Rect::from_xy_wh(wh.to_xy() * -0.5, *wh),
                        image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: None,
                        },
                    }),
                ),
            ),
        )
    }
}

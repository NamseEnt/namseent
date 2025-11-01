use namui::*;
use std::ops::{Add, Mul, Sub};

// 상수들을 컴파일 타임에 계산
const SPRING_STRENGTH: f32 = 350.0;
const DAMPING: f32 = 25.0;
const SNAP_THRESHOLD: f32 = 0.01;
const VELOCITY_THRESHOLD: f32 = 0.5;
const MAX_DELTA_TIME: f32 = 1.0 / 30.0; // 최대 30fps에 해당하는 델타 타임으로 제한
const MIN_DELTA_TIME: f32 = 0.001; // 최소 델타 타임

// 제곱값들을 미리 계산하여 런타임 계산 최소화
const SNAP_THRESHOLD_SQ: f32 = SNAP_THRESHOLD * SNAP_THRESHOLD;
const VELOCITY_THRESHOLD_SQ: f32 = VELOCITY_THRESHOLD * VELOCITY_THRESHOLD;
const NEG_DAMPING: f32 = -DAMPING;

pub fn xy_with_spring<T>(ctx: &RenderCtx, target_xy: Xy<T>, initial_xy: Xy<T>) -> Xy<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<f32, Output = T>
        + PartialEq
        + std::fmt::Debug
        + Send
        + Sync
        + 'static
        + State,
    T: From<f32> + Into<f32>,
{
    let now = Instant::now();
    let (context, set_context) = ctx.state(|| SpringAnimationContext {
        last_tick_at: now,
        velocity: Xy::new(0.0.into(), 0.0.into()),
        position: initial_xy,
    });

    // 델타 타임 계산 및 제한
    let mut delta_time = (now - context.last_tick_at).as_secs_f32();

    // 매우 작은 델타 타임이면 애니메이션 건너뛰기
    if delta_time < MIN_DELTA_TIME {
        return context.position;
    }

    // 프레임드랍으로 인한 과도한 이동을 방지하기 위해 델타 타임 제한
    delta_time = delta_time.min(MAX_DELTA_TIME);

    // 변위 계산
    let displacement = target_xy - context.position;

    // 변위와 속도를 f32로 미리 변환하여 중복 변환 방지
    let displacement_x_f32: f32 = displacement.x.into();
    let displacement_y_f32: f32 = displacement.y.into();
    let velocity_x_f32: f32 = context.velocity.x.into();
    let velocity_y_f32: f32 = context.velocity.y.into();

    // 변위가 매우 작으면 목표 위치로 스냅
    let displacement_magnitude_sq =
        displacement_x_f32 * displacement_x_f32 + displacement_y_f32 * displacement_y_f32;

    if displacement_magnitude_sq < SNAP_THRESHOLD_SQ {
        let velocity_magnitude_sq =
            velocity_x_f32 * velocity_x_f32 + velocity_y_f32 * velocity_y_f32;

        if velocity_magnitude_sq < VELOCITY_THRESHOLD_SQ {
            // 목표에 도달했으므로 상태 업데이트하고 반환
            set_context.mutate(move |ctx| {
                ctx.last_tick_at = now;
                ctx.velocity = Xy::new(0.0.into(), 0.0.into());
                ctx.position = target_xy;
            });
            return target_xy;
        }
    }

    // 물리 계산 (한 번에 수행)
    let spring_force = displacement * SPRING_STRENGTH;
    let damping_force = context.velocity * NEG_DAMPING;
    let total_force = spring_force + damping_force;

    // 속도와 위치 업데이트
    let new_velocity = context.velocity + total_force * delta_time;
    let new_position = context.position + new_velocity * delta_time;

    // 상태 업데이트 (한 번에)
    set_context.mutate(move |ctx| {
        ctx.last_tick_at = now;
        ctx.velocity = new_velocity;
        ctx.position = new_position;
    });

    new_position
}

#[derive(State)]
struct SpringAnimationContext<T: std::fmt::Debug + State> {
    last_tick_at: Instant,
    velocity: Xy<T>,
    position: Xy<T>,
}

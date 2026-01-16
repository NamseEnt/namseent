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

/// 일반적인 spring 애니메이션을 수행하는 함수
/// T가 스프링 물리 연산을 지원하는 모든 타입에 대해 작동함
pub fn with_spring<T>(
    ctx: &RenderCtx,
    target: T,
    initial: T,
    magnitude_sq: impl Fn(&T) -> f32,
    zero: impl Fn() -> T + Send + 'static,
) -> T
where
    T: Copy + PartialEq + std::fmt::Debug + Send + Sync + 'static + State,
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>,
{
    let now = Instant::now();
    let (context, set_context) = ctx.state(|| SpringAnimationContext {
        last_tick_at: now,
        velocity: zero(),
        position: initial,
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
    let displacement = target - context.position;

    // 변위가 매우 작으면 목표로 스냅
    let displacement_magnitude_sq = magnitude_sq(&displacement);

    if displacement_magnitude_sq < SNAP_THRESHOLD_SQ {
        let velocity_magnitude_sq = magnitude_sq(&context.velocity);

        if velocity_magnitude_sq < VELOCITY_THRESHOLD_SQ {
            // 목표에 도달했으므로 상태 업데이트하고 반환
            set_context.mutate(move |ctx| {
                ctx.last_tick_at = now;
                ctx.velocity = zero();
                ctx.position = target;
            });
            return target;
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
    velocity: T,
    position: T,
}

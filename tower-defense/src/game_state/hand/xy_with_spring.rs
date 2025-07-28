use crate::game_state::use_game_state;
use namui::*;

pub(super) fn xy_with_spring(ctx: &RenderCtx, target_xy: Xy<Px>, initial_xy: Xy<Px>) -> Xy<Px> {
    let now = use_game_state(ctx).now();
    let (context, set_context) = ctx.state(|| SpringAnimationContext {
        last_tick_at: now,
        velocity: Xy::zero(),
        position: initial_xy,
    });

    // 스프링 애니메이션 상수
    const SPRING_STRENGTH: f32 = 250.0; // 스프링 강도
    const DAMPING: f32 = 15.0; // 감쇠

    // 델타 타임 계산
    let delta_time = (now - context.last_tick_at).as_secs_f32();
    let delta_time = delta_time.min(1.0 / 30.0); // 최대 30fps로 제한

    // 목표 위치와의 차이 계산
    let displacement = Xy {
        x: target_xy.x - context.position.x,
        y: target_xy.y - context.position.y,
    };

    // 스프링 힘 계산 (Hooke's law: F = -kx)
    let spring_force = Xy {
        x: displacement.x * SPRING_STRENGTH,
        y: displacement.y * SPRING_STRENGTH,
    };

    // 감쇠 힘 계산
    let damping_force = Xy {
        x: -context.velocity.x * DAMPING,
        y: -context.velocity.y * DAMPING,
    };

    // 전체 힘 계산
    let total_force = Xy {
        x: spring_force.x + damping_force.x,
        y: spring_force.y + damping_force.y,
    };

    // 속도 업데이트 (F = ma, a = F/m, m = 1 가정)
    let new_velocity = Xy {
        x: context.velocity.x + total_force.x * delta_time,
        y: context.velocity.y + total_force.y * delta_time,
    };

    // 위치 업데이트
    let new_position = Xy {
        x: context.position.x + new_velocity.x * delta_time,
        y: context.position.y + new_velocity.y * delta_time,
    };

    // 매우 작은 움직임일 때는 목표 위치로 스냅
    let threshold = px(0.5);
    let final_position = if displacement.x.abs() < threshold
        && displacement.y.abs() < threshold
        && new_velocity.x.abs() < threshold
        && new_velocity.y.abs() < threshold
    {
        target_xy
    } else {
        new_position
    };

    // 상태 업데이트
    let new_velocity = if final_position == target_xy {
        Xy::zero()
    } else {
        new_velocity
    };
    set_context.mutate(move |ctx| {
        ctx.last_tick_at = now;
        ctx.velocity = new_velocity;
        ctx.position = final_position;
    });

    final_position
}

struct SpringAnimationContext {
    last_tick_at: Instant,
    velocity: Xy<Px>,
    position: Xy<Px>,
}

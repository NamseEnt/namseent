use super::{
    cannon_ball::{CannonBall, CANNON_BALLS_ATOM},
    ship::SHIP_KINETICS_ATOM,
};
use crate::app::{
    ballistics::AimMovingTarget,
    mechanics::{Meter, SpeedExt},
    PX_PER_METER_ATOM,
};
use namui::prelude::*;

pub const FORTRESS_RADIUS: Meter = Meter(10.0);
pub static FORTRESS_STATE_ATOM: Atom<FortressState> = Atom::uninitialized_new();

#[component]
pub struct Fortress {
    pub now: Instant,
}
impl Component for Fortress {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { now } = self;

        let (px_per_meter, _) = ctx.atom(&PX_PER_METER_ATOM);
        let (_, set_cannon_balls) = ctx.atom(&CANNON_BALLS_ATOM);
        let (ship_kinetic, _) = ctx.atom(&SHIP_KINETICS_ATOM);
        let (fortress_state, _) = ctx.atom(&FORTRESS_STATE_ATOM);

        let (last_fire_time, set_last_fire_time) = ctx.state(|| now);

        let FortressState {
            center_xy,
            impacted_at,
        } = *fortress_state;
        let fire_interval = Duration::from_secs(4);

        let color = {
            let elapsed = now - impacted_at;
            if elapsed < 3.sec() {
                match ((elapsed.as_secs_f32() * 4.0) as isize) & 2 {
                    0 => Color::RED,
                    _ => Color::GREEN,
                }
            } else {
                Color::GREEN
            }
        };
        let dt = now - *last_fire_time;
        let fire_cannon = || {
            let projectile_speed = 100.mps();

            let start_xy = center_xy;

            let (xy_vector, xz_angle) = AimMovingTarget {
                projectile_speed,
                start_xy,
                target_xy: ship_kinetic.center_xy,
                target_speed: ship_kinetic.front_velocity,
                target_yaw: ship_kinetic.yaw,
            }
            .aim();

            let cannon_ball = CannonBall {
                start_xy,
                xy_vector,
                xz_angle,
                start_speed: projectile_speed,
                start_at: now,
            };

            set_cannon_balls.mutate(move |cannon_balls| {
                cannon_balls.push(cannon_ball);
            });
        };

        if dt > fire_interval {
            set_last_fire_time.set(now);
            fire_cannon();
        }

        let center_xy_px = Xy::single(*px_per_meter) * center_xy;
        let fortress_radius = *px_per_meter * FORTRESS_RADIUS;
        ctx.component(path(
            Path::new().add_oval(Rect::from_xy_wh(
                center_xy_px - Xy::single(fortress_radius),
                Wh::single(fortress_radius * 2),
            )),
            Paint::new(color),
        ));

        ctx.done()
    }
}

#[derive(Debug, Clone)]
pub struct FortressState {
    pub center_xy: Xy<Meter>,
    pub impacted_at: Instant,
}
pub trait MutateFortressState {
    fn impact(self, now: Instant);
}
impl MutateFortressState for SetState<FortressState> {
    fn impact(self, now: Instant) {
        self.mutate(move |fortress| {
            fortress.impacted_at = now;
        });
    }
}

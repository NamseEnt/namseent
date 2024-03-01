use crate::app::{
    ballistics::GRAVITY,
    camera::CAMERA_STATE_ATOM,
    mechanics::{Meter, MeterExt, Speed},
};
use namui::prelude::*;
use num_traits::Zero;

pub static CANNON_BALLS_ATOM: Atom<Vec<CannonBall>> = Atom::uninitialized_new();

#[derive(Debug, PartialEq, Clone)]
pub struct CannonBall {
    pub start_xy: Xy<Meter>,
    pub xy_vector: Xy<f32>,
    pub xz_angle: Angle,
    pub start_speed: Speed,
    pub start_at: Instant,
}

pub struct Xyz<T> {
    pub xy: Xy<T>,
    pub z: T,
}

impl CannonBall {
    pub fn xyz(&self, time: Instant) -> Xyz<Meter> {
        let Self {
            start_xy,
            xy_vector,
            xz_angle,
            start_speed,
            start_at,
        } = self;

        let xy_start_speed = *start_speed * xz_angle.cos();
        let z_speed = *start_speed * xz_angle.sin();

        let t = time - *start_at;

        let xy_length = xy_start_speed * t;
        let xy_at_t = start_xy + Xy::single(xy_length) * xy_vector;
        let z_at_t = z_speed * t - 0.5 * GRAVITY * t * t;

        Xyz {
            xy: xy_at_t,
            z: z_at_t,
        }
    }
}

#[component]
pub struct CannonBalls {
    pub now: Instant,
}
impl Component for CannonBalls {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { now } = self;

        let (camera_state, _) = ctx.atom(&CAMERA_STATE_ATOM);
        let (cannon_balls, _) = ctx.atom(&CANNON_BALLS_ATOM);

        let px_per_meter = camera_state.px_per_meter();
        let screen_left_top_xy = camera_state.screen_left_top_xy();

        ctx.compose(|ctx| {
            for cannon_ball in cannon_balls.as_ref() {
                let xyz = cannon_ball.xyz(now);
                let shadow = xyz.xy - screen_left_top_xy;
                let bullet = shadow + Xy::new(Meter::zero(), -xyz.z);

                ctx.add(path(
                    Path::new().add_oval(Rect::from_xy_wh(
                        Xy::single(px_per_meter) * (bullet - Xy::single(2.meter())),
                        Wh::single(px_per_meter * 4.meter()),
                    )),
                    Paint::new(Color::RED),
                ));

                ctx.add(path(
                    Path::new().add_oval(Rect::from_xy_wh(
                        Xy::single(px_per_meter) * (shadow - Xy::single(2.meter())),
                        Wh::single(px_per_meter * 4.meter()),
                    )),
                    Paint::new(Color::BLACK),
                ));
            }
        });

        ctx.done()
    }
}

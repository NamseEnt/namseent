mod ballistics;
mod mechanics;

use self::{
    ballistics::GRAVITY,
    mechanics::{Meter, MeterExt, Speed, SpeedExt},
};
use namui::prelude::*;
use num_traits::{One, Zero};

static PX_PER_METER_ATOM: Atom<Per<Px, Meter>> = Atom::uninitialized_new();

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        ctx.atom_init(&PX_PER_METER_ATOM, || Per::new(1.px(), Meter::one()));

        ctx.component(Ship {});

        ctx.done()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct CannonBall {
    start_xy: Xy<Meter>,
    xy_vector: Xy<f32>,
    xz_angle: Angle,
    start_speed: Speed,
    start_at: Instant,
}

struct Xyz<T> {
    xy: Xy<T>,
    z: T,
}

impl CannonBall {
    fn xyz(&self, time: Instant) -> Xyz<Meter> {
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

#[namui::component]
pub struct Ship {}
impl namui::Component for Ship {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let now = namui::time::now();

        let (px_per_meter, _) =
            ctx.atom_init(&PX_PER_METER_ATOM, || Per::new(1.px(), Meter::one()));
        let (center_xy, set_center_xy) = ctx.state(|| Xy::new(100.meter(), 100.meter()));
        let (yaw, set_yaw) = ctx.state(|| 0.rad());
        let (front_velocity, set_front_velocity) = ctx.state(Speed::zero);
        let (move_ship_last_time, set_move_ship_last_time) = ctx.state(|| now);
        let (cannon_balls, set_cannon_balls) = ctx.state::<Vec<CannonBall>>(Vec::new);

        let ship_radius = *px_per_meter * 10.meter();

        let dt = now - *move_ship_last_time;

        let update_ship_xy = || {
            set_move_ship_last_time.set(now);
            let dl = *front_velocity * dt;
            let dxy = Xy::single(dl) * yaw.as_xy();
            set_center_xy.set(*center_xy + dxy);
        };

        let update_cannon_balls = || {
            if cannon_balls
                .iter()
                .any(|cannon_ball| cannon_ball.xyz(now).z < 0.meter())
            {
                set_cannon_balls.set(
                    cannon_balls
                        .iter()
                        .filter(|cannon_ball| cannon_ball.xyz(now).z >= 0.meter())
                        .cloned()
                        .collect(),
                );
            }
        };

        if dt > (1.0 / 60.0).sec() {
            update_ship_xy();
            update_cannon_balls();
        }

        ctx.on_raw_event(|event| match event {
            RawEvent::KeyDown { event } => match event.code {
                Code::ArrowLeft => {
                    set_yaw.mutate(|angle| *angle -= 10.deg());
                }
                Code::ArrowRight => {
                    set_yaw.mutate(|angle| *angle += 10.deg());
                }
                Code::ArrowUp => {
                    set_front_velocity.mutate(|v| *v = *v + 10.mps());
                }
                Code::ArrowDown => {
                    set_front_velocity.mutate(|v| *v = *v - 10.mps());
                }
                _ => {}
            },
            RawEvent::MouseDown { event } => {
                let start_speed = 100.mps();

                let start_xy = *center_xy;
                let target_xy = (event.xy.into_type::<f32>()
                    / (*px_per_meter * Meter::one()).as_f32())
                    * Xy::single(Meter::one());
                let xy_diff = target_xy - start_xy;
                let xy_vector = xy_diff.normalize_f32();
                let distance = xy_diff.length();

                let max_range = ballistics::range(start_speed, 45.deg());
                let xz_angle = match max_range <= distance {
                    true => 45.deg(),
                    false => ballistics::calculate_launch_angle(start_speed, distance),
                };

                let cannon_ball = CannonBall {
                    start_xy,
                    xy_vector,
                    xz_angle,
                    start_speed,
                    start_at: now,
                };

                set_cannon_balls.mutate(move |cannon_balls| {
                    cannon_balls.push(cannon_ball);
                });
            }
            _ => (),
        });

        ctx.compose(|ctx| {
            for cannon_ball in cannon_balls.as_ref() {
                let xyz = cannon_ball.xyz(now);
                let shadow = xyz.xy;
                let bullet = shadow + Xy::new(Meter::zero(), -xyz.z);

                ctx.add(path(
                    Path::new().add_oval(Rect::from_xy_wh(
                        Xy::single(*px_per_meter) * (shadow - Xy::single(2.meter())),
                        Wh::single(*px_per_meter * 4.meter()),
                    )),
                    Paint::new(Color::BLACK),
                ));

                ctx.add(path(
                    Path::new().add_oval(Rect::from_xy_wh(
                        Xy::single(*px_per_meter) * (bullet - Xy::single(2.meter())),
                        Wh::single(*px_per_meter * 4.meter()),
                    )),
                    Paint::new(Color::RED),
                ));
            }
        });

        let head_radius = *px_per_meter * 5.meter();
        let center_xy_px = Xy::single(*px_per_meter) * *center_xy;
        ctx.component(path(
            Path::new().move_to(center_xy_px.x, center_xy_px.y).line_to(
                center_xy_px.x + (ship_radius + head_radius) * yaw.cos(),
                center_xy_px.y + (ship_radius + head_radius) * yaw.sin(),
            ),
            Paint::new(Color::BLUE)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(5.px()),
        ));

        ctx.component(path(
            Path::new().add_oval(Rect::from_xy_wh(
                center_xy_px - Xy::single(ship_radius),
                Wh::single(ship_radius * 2),
            )),
            Paint::new(Color::RED),
        ));

        ctx.done()
    }
}

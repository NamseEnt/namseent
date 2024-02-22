mod ballistics;

use namui::prelude::*;

use self::ballistics::GRAVITY;

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        ctx.component(Ship {});

        ctx.done()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct CannonBall {
    start_xy: Xy<Px>,
    xy_vector: Xy<f32>,
    xz_angle: Angle,
    start_velocity: Per<Px, Duration>,
    start_at: Instant,
}

struct Xyz {
    xy: Xy<Px>,
    z: Px,
}

impl CannonBall {
    fn xyz(&self, time: Instant) -> Xyz {
        let Self {
            start_xy,
            xy_vector,
            xz_angle,
            start_velocity,
            start_at,
        } = self;

        let xy_start_velocity = start_velocity.mul_to_numerator(xz_angle.cos());
        let z_velocity = start_velocity.mul_to_numerator(xz_angle.sin());

        let t = time - *start_at;

        let xy_length = xy_start_velocity * t;
        let xy_at_t = start_xy + Xy::single(xy_length) * xy_vector;
        let z_at_t = z_velocity * t - GRAVITY * (t / 1.sec()) * (t / 1.sec()) / 2.0;

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

        let (center_xy, set_center_xy) = ctx.state(|| Xy::new(100.px(), 100.px()));
        let (yaw, set_yaw) = ctx.state(|| 0.rad());
        let (front_velocity, set_front_velocity) = ctx.state(|| Per::new(0.px(), 1.sec()));
        let (move_ship_last_time, set_move_ship_last_time) = ctx.state(|| now);
        let (cannon_balls, set_cannon_balls) = ctx.state::<Vec<CannonBall>>(Vec::new);

        let ship_radius = 10.px();

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
                .any(|cannon_ball| cannon_ball.xyz(now).z < 0.px())
            {
                set_cannon_balls.set(
                    cannon_balls
                        .iter()
                        .filter(|cannon_ball| cannon_ball.xyz(now).z >= 0.px())
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
                    set_front_velocity.mutate(|v| *v += Per::new(10.px(), 1.sec()));
                }
                Code::ArrowDown => {
                    set_front_velocity.mutate(|v| *v -= Per::new(10.px(), 1.sec()));
                }
                _ => {}
            },
            RawEvent::MouseDown { event } => {
                let start_velocity = Per::new(100.px(), 1.sec());

                let start_xy = *center_xy;
                let xy_diff = event.xy - start_xy;
                let xy_vector = xy_diff.normalize_f32();
                let distance = xy_diff.length();

                let max_range = ballistics::range(start_velocity, 45.deg());
                let xz_angle = match max_range <= distance {
                    true => 45.deg(),
                    false => ballistics::calculate_launch_angle(start_velocity, distance),
                };

                let cannon_ball = CannonBall {
                    start_xy,
                    xy_vector,
                    xz_angle,
                    start_velocity,
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
                let bullet = shadow + Xy::new(0.px(), -xyz.z);

                ctx.add(path(
                    Path::new().add_oval(Rect::from_xy_wh(
                        shadow - Xy::single(2.px()),
                        Wh::single(4.px()),
                    )),
                    Paint::new(Color::BLACK),
                ));

                ctx.add(path(
                    Path::new().add_oval(Rect::from_xy_wh(
                        bullet - Xy::single(2.px()),
                        Wh::single(4.px()),
                    )),
                    Paint::new(Color::RED),
                ));
            }
        });

        let head_radius = 5.px();
        ctx.component(path(
            Path::new().move_to(center_xy.x, center_xy.y).line_to(
                center_xy.x + (ship_radius + head_radius) * yaw.cos(),
                center_xy.y + (ship_radius + head_radius) * yaw.sin(),
            ),
            Paint::new(Color::BLUE)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(5.px()),
        ));

        ctx.component(path(
            Path::new().add_oval(Rect::from_xy_wh(
                *center_xy - Xy::single(ship_radius),
                Wh::single(ship_radius * 2),
            )),
            Paint::new(Color::RED),
        ));

        ctx.done()
    }
}

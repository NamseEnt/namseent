use super::cannon_ball::{CannonBall, CANNON_BALLS_ATOM};
use crate::app::{
    ballistics,
    camera::CAMERA_STATE_ATOM,
    mechanics::{Acceleration, AccelerationExt, Meter, MeterExt, Speed, SpeedExt},
};
use namui::{network::http::IntoUrl, prelude::*};
use num_traits::{AsPrimitive, Float, One, Signed};
use std::ops::Neg;

const SHIP_RADIUS: Meter = Meter(10.0);

pub static SHIP_KINETICS_ATOM: Atom<ShipKinetics> = Atom::uninitialized_new();

#[namui::component]
pub struct Ship {
    pub now: Instant,
}
impl namui::Component for Ship {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { now } = self;

        let (camera_state, _) = ctx.atom(&CAMERA_STATE_ATOM);
        let (ship_kinetics, set_ship_kinetics) = ctx.atom(&SHIP_KINETICS_ATOM);
        let (_, set_cannon_balls) = ctx.atom(&CANNON_BALLS_ATOM);

        let px_per_meter = camera_state.px_per_meter();
        let screen_left_top_xy = camera_state.screen_left_top_xy();
        let ShipKinetics { center_xy, yaw, .. } = *ship_kinetics;
        let ship_radius = px_per_meter * SHIP_RADIUS;
        let start_speed = 100.mps();

        let start_xy = center_xy;
        let target_xy = screen_left_top_xy
            + ((mouse::position()).into_type::<f32>() / (px_per_meter * Meter::one()).as_f32())
                * Xy::single(Meter::one());
        let xy_diff = target_xy - start_xy;
        let xy_vector = xy_diff.normalize_f32();
        let distance = xy_diff.length();
        let max_range = ballistics::calculate_range_with_xz_angle(start_speed, 45.deg());

        ctx.on_raw_event(|event| match event {
            RawEvent::KeyDown { event } => match event.code {
                Code::ArrowUp => {
                    set_ship_kinetics.mutate_throttle_up();
                }
                Code::ArrowDown => {
                    set_ship_kinetics.mutate_throttle_down();
                }
                _ => {}
            },
            RawEvent::MouseDown { .. } => {
                let xz_angle = match max_range <= distance {
                    true => 45.deg(),
                    false => {
                        ballistics::calculate_launch_angle_with_distance(start_speed, distance)
                    }
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

        ctx.component(ExpectedTrajectory {
            start_xy,
            target_xy,
            max_range,
        });

        let head_radius = px_per_meter * 5.meter();
        let center_xy_px = Xy::single(px_per_meter) * (center_xy - screen_left_top_xy);
        ctx.component(path(
            Path::new().move_to(center_xy_px.x, center_xy_px.y).line_to(
                center_xy_px.x + (ship_radius + head_radius) * yaw.cos(),
                center_xy_px.y + (ship_radius + head_radius) * yaw.sin(),
            ),
            Paint::new(Color::BLUE)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(5.px()),
        ));

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate(center_xy_px);
            let mut ctx = if yaw.cos().is_positive() {
                ctx.scale(Xy::new(-1.0, 1.0))
            } else {
                ctx
            };
            ctx.add(ImageDrawCommand {
                rect: Rect::Ltrb {
                    left: -ship_radius,
                    top: -ship_radius,
                    right: ship_radius,
                    bottom: ship_radius,
                },
                source: ImageSource::Url {
                    url: "bundle:resources/ship.png".into_url().unwrap(),
                },
                fit: ImageFit::Cover,
                paint: None,
            });
        });

        ctx.done()
    }
}

#[derive(Debug, Clone)]
pub struct ShipKinetics {
    pub center_xy: Xy<Meter>,
    pub yaw: Angle,
    pub velocity: Xy<Speed>,
    pub throttle: ShipThrottle,
    pub rudder: Angle,
}
pub trait MutateShipKinetics {
    fn mutate_throttle_up(self);
    fn mutate_throttle_down(self);
    fn mutate_tick(self, dt: Duration, left: bool, right: bool);
}
impl MutateShipKinetics for SetState<ShipKinetics> {
    fn mutate_throttle_up(self) {
        self.mutate(move |ship_kinetics| {
            ship_kinetics.throttle = ship_kinetics.throttle.up();
        });
    }

    fn mutate_throttle_down(self) {
        self.mutate(move |ship_kinetics| {
            ship_kinetics.throttle = ship_kinetics.throttle.down();
        });
    }

    fn mutate_tick(self, dt: Duration, left: bool, right: bool) {
        self.mutate(move |ship_kinetics| {
            ship_kinetics.drag(dt);
            ship_kinetics.handle_rudder(dt, left, right);
            ship_kinetics.rotate(dt);
            ship_kinetics.accelerate(dt);
            ship_kinetics.move_xy(dt);
        });
    }
}
impl ShipKinetics {
    pub fn drag(&mut self, dt: Duration) {
        const DRAG: f32 = 0.05;
        let drag_acc = |speed: Speed| {
            let sign = speed.signum().neg();
            (sign * speed * speed * DRAG).as_f32().mpsps()
        };
        self.velocity.x = self.velocity.x + drag_acc(self.velocity.x) * dt;
        self.velocity.y = self.velocity.y + drag_acc(self.velocity.y) * dt;
    }
    pub fn handle_rudder(&mut self, dt: Duration, left: bool, right: bool) {
        let amount = 45.deg() * dt.as_secs_f32();
        if left {
            self.rudder = Angle::Degree((self.rudder - amount).as_degrees().clamp(-45.0, 45.0));
        }
        if right {
            self.rudder = Angle::Degree((self.rudder + amount).as_degrees().clamp(-45.0, 45.0));
        }
        if !left && !right {
            let rudder = self.rudder.as_degrees();
            let sign = rudder.signum();
            self.rudder =
                Angle::Degree(sign * (rudder.abs() - amount.as_degrees()).clamp(0.0, 45.0));
        }
    }
    pub fn rotate(&mut self, dt: Duration) {
        let direction_weight = (self.velocity.atan2() - self.yaw).cos();
        let speed_weight = (self.velocity.length() / 10.mps())
            .as_f32()
            .clamp(-1.0, 1.0);
        self.yaw += self.rudder * direction_weight * speed_weight * dt.as_secs_f32();
    }
    pub fn accelerate(&mut self, dt: Duration) {
        const ACCELERATION: Acceleration = Acceleration(20.0);
        let amount = self.throttle.mps() * (ACCELERATION * dt);
        self.velocity = self.velocity + self.yaw.as_xy() * Xy::single(amount);
        namui::log!("speed {:?}", self.velocity.length());
    }
    pub fn move_xy(&mut self, dt: Duration) {
        self.center_xy = self.center_xy + self.velocity * Xy::single(dt);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShipThrottle {
    Full,
    ThreeQuarter,
    Half,
    Quarter,
    Idle,
    Reverse,
}
impl ShipThrottle {
    pub fn up(self) -> Self {
        match self {
            Self::Full => Self::Full,
            Self::ThreeQuarter => Self::Full,
            Self::Half => Self::ThreeQuarter,
            Self::Quarter => Self::Half,
            Self::Idle => Self::Quarter,
            Self::Reverse => Self::Idle,
        }
    }
    pub fn down(self) -> Self {
        match self {
            Self::Full => Self::ThreeQuarter,
            Self::ThreeQuarter => Self::Half,
            Self::Half => Self::Quarter,
            Self::Quarter => Self::Idle,
            Self::Idle => Self::Reverse,
            Self::Reverse => Self::Reverse,
        }
    }
}
impl<F> AsPrimitive<F> for ShipThrottle
where
    F: Float + 'static,
{
    fn as_(self) -> F {
        match self {
            Self::Full => F::one(),
            Self::ThreeQuarter => F::from(0.75).unwrap(),
            Self::Half => F::from(0.5).unwrap(),
            Self::Quarter => F::from(0.25).unwrap(),
            Self::Idle => F::zero(),
            Self::Reverse => F::one().neg(),
        }
    }
}

#[component]
struct ExpectedTrajectory {
    start_xy: Xy<Meter>,
    target_xy: Xy<Meter>,
    max_range: Meter,
}
impl Component for ExpectedTrajectory {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            start_xy,
            target_xy,
            max_range,
        } = self;

        let (camera_state, _) = ctx.atom(&CAMERA_STATE_ATOM);
        let screen_left_top_xy = camera_state.screen_left_top_xy();
        let px_per_meter = camera_state.px_per_meter();

        let diff = target_xy - start_xy;
        let vector = diff.normalize_f32();
        let length = diff.length();
        let opacity = ((max_range - length / 1.meter()).as_f32().clamp(0.0, 1.0) * 255.0) as u8;

        ctx.compose(|ctx| {
            if opacity == 0 {
                return;
            }
            let gradient_end_xy = {
                let in_meter = start_xy + vector * Xy::single(Meter::min(length, 10.meter()))
                    - screen_left_top_xy;
                Xy::single(px_per_meter) * in_meter
            };
            let curve_control_xy = {
                let mut middle = start_xy + diff / 2.0;
                let z_offset = (length / max_range) * (max_range / 4.0);
                middle.y = middle.y - z_offset;
                Xy::single(px_per_meter) * (middle - screen_left_top_xy)
            };
            let start_xy = Xy::single(px_per_meter) * (start_xy - screen_left_top_xy);
            let target_xy = Xy::single(px_per_meter) * (target_xy - screen_left_top_xy);

            let paint = Paint::new(Color::BLACK)
                .set_shader(Shader::LinearGradient {
                    start_xy,
                    end_xy: gradient_end_xy,
                    colors: vec![Color::RED.with_alpha(0), Color::RED.with_alpha(opacity)],
                    tile_mode: TileMode::Clamp,
                })
                .set_style(PaintStyle::Stroke)
                .set_stroke_cap(StrokeCap::Round)
                .set_stroke_width(4.px());
            let path = Path::new().move_to(start_xy.x, start_xy.y).quad_to(
                curve_control_xy.x,
                curve_control_xy.y,
                target_xy.x,
                target_xy.y,
            );
            ctx.add(namui::path(path, paint));
        });

        ctx.done()
    }
}

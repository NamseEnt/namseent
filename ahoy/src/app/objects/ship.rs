use super::cannon_ball::{CannonBall, CANNON_BALLS_ATOM};
use crate::app::{
    ballistics,
    mechanics::{Meter, MeterExt, Speed, SpeedExt},
    PX_PER_METER_ATOM,
};
use namui::prelude::*;
use num_traits::One;

const SHIP_RADIUS: Meter = Meter(10.0);

pub static SHIP_KINETICS_ATOM: Atom<ShipKinetics> = Atom::uninitialized_new();

#[namui::component]
pub struct Ship {
    pub now: Instant,
}
impl namui::Component for Ship {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { now } = self;

        let (px_per_meter, _) = ctx.atom(&PX_PER_METER_ATOM);
        let (ship_kinetics, set_ship_kinetics) = ctx.atom(&SHIP_KINETICS_ATOM);
        let (_, set_cannon_balls) = ctx.atom(&CANNON_BALLS_ATOM);

        let ShipKinetics { center_xy, yaw, .. } = *ship_kinetics;
        let ship_radius = *px_per_meter * SHIP_RADIUS;

        ctx.on_raw_event(|event| match event {
            RawEvent::KeyDown { event } => match event.code {
                Code::ArrowLeft => {
                    set_ship_kinetics.mutate_yaw(-(10.deg()));
                }
                Code::ArrowRight => {
                    set_ship_kinetics.mutate_yaw(10.deg());
                }
                Code::ArrowUp => {
                    set_ship_kinetics.mutate_velocity(10.mps());
                }
                Code::ArrowDown => {
                    set_ship_kinetics.mutate_velocity(-(10.mps()));
                }
                _ => {}
            },
            RawEvent::MouseDown { event } => {
                let start_speed = 100.mps();

                let start_xy = center_xy;
                let target_xy = (event.xy.into_type::<f32>()
                    / (*px_per_meter * Meter::one()).as_f32())
                    * Xy::single(Meter::one());
                let xy_diff = target_xy - start_xy;
                let xy_vector = xy_diff.normalize_f32();
                let distance = xy_diff.length();

                let max_range = ballistics::calculate_range_with_xz_angle(start_speed, 45.deg());
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

        let head_radius = *px_per_meter * 5.meter();
        let center_xy_px = Xy::single(*px_per_meter) * center_xy;
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

#[derive(Debug, Clone)]
pub struct ShipKinetics {
    pub center_xy: Xy<Meter>,
    pub yaw: Angle,
    pub front_velocity: Speed,
}
pub trait MutateShipKinetics {
    fn mutate_center_xy(self, delta: Xy<Meter>);
    fn mutate_yaw(self, delta: Angle);
    fn mutate_velocity(self, delta: Speed);
}
impl MutateShipKinetics for SetState<ShipKinetics> {
    fn mutate_center_xy(self, delta: Xy<Meter>) {
        self.mutate(move |ship_kinetics| {
            ship_kinetics.center_xy = ship_kinetics.center_xy + delta;
        });
    }

    fn mutate_yaw(self, delta: Angle) {
        self.mutate(move |ship_kinetics| {
            ship_kinetics.yaw += delta;
        });
    }

    fn mutate_velocity(self, delta: Speed) {
        self.mutate(move |ship_kinetics| {
            ship_kinetics.front_velocity = ship_kinetics.front_velocity + delta;
        });
    }
}

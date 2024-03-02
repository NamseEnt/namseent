mod ballistics;
mod camera;
mod mechanics;
mod objects;

use self::{
    camera::{Camera, CameraState, MutateCameraState, CAMERA_STATE_ATOM},
    mechanics::{Meter, MeterExt},
    objects::{
        cannon_ball::{CannonBalls, CANNON_BALLS_ATOM},
        fortress::{
            Fortress, FortressState, MutateFortressState, FORTRESS_RADIUS, FORTRESS_STATE_ATOM,
        },
        ship::{MutateShipKinetics, Ship, ShipKinetics, SHIP_KINETICS_ATOM},
    },
};
use namui::prelude::*;
use num_traits::One;
use std::vec;

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let now = namui::time::now();

        ctx.atom_init(&CAMERA_STATE_ATOM, || {
            CameraState::new(Per::new(3.px(), Meter::one()), Xy::single(100.meter()))
        });
        ctx.atom_init(&CANNON_BALLS_ATOM, Vec::new);
        ctx.atom_init(&SHIP_KINETICS_ATOM, || ShipKinetics {
            center_xy: Xy::single(100.meter()),
            yaw: 0.rad(),
            velocity: Xy::zero(),
            throttle: objects::ship::ShipThrottle::Idle,
            rudder: 0.rad(),
        });
        ctx.atom_init(&FORTRESS_STATE_ATOM, || FortressState {
            center_xy: Xy::single(200.meter()),
            impacted_at: now,
        });

        ctx.component(Tick { now });
        ctx.component(CannonBalls { now });
        ctx.component(Ship { now });
        ctx.component(Fortress { now });
        ctx.component(Camera { now });

        ctx.done()
    }
}

#[namui::component]
pub struct Tick {
    pub now: Instant,
}
impl Component for Tick {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { now } = self;

        let (_, set_camera_state) = ctx.atom(&CAMERA_STATE_ATOM);
        let (cannon_balls, set_cannon_balls) = ctx.atom(&CANNON_BALLS_ATOM);
        let (ship_kinetics_atom, set_ship_kinetics_atom) = ctx.atom(&SHIP_KINETICS_ATOM);
        let (fortress_state, set_fortress_state) = ctx.atom(&FORTRESS_STATE_ATOM);

        let (last_tick_time, set_last_tick_time) = ctx.state(|| now);

        let ShipKinetics {
            center_xy: ship_center_xy,
            ..
        } = *ship_kinetics_atom;

        let dt = now - *last_tick_time;
        let update_ship_xy = || {
            let left = keyboard::any_code_press([Code::ArrowLeft]);
            let right = keyboard::any_code_press([Code::ArrowRight]);
            set_ship_kinetics_atom.mutate_tick(dt, left, right);
        };

        let update_cannon_balls = || {
            if cannon_balls
                .iter()
                .any(|cannon_ball| cannon_ball.xyz(now).z < 0.meter())
            {
                let mut impact_xys = vec![];
                set_cannon_balls.set(
                    cannon_balls
                        .iter()
                        .filter(|cannon_ball| {
                            let xyz = cannon_ball.xyz(now);
                            let not_impacted = cannon_ball.xyz(now).z >= 0.meter();
                            if !not_impacted {
                                impact_xys.push(xyz.xy);
                            }
                            not_impacted
                        })
                        .cloned()
                        .collect(),
                );

                for impact_xy in impact_xys {
                    if (fortress_state.center_xy - impact_xy).length() < FORTRESS_RADIUS {
                        set_fortress_state.impact(now);
                    }
                }
            }
        };

        let tick_camera = || {
            set_camera_state.mutate_tick(now);
            set_camera_state.mutate_center_xy(now, ship_center_xy);
        };

        if dt > (1.0 / 60.0).sec() {
            set_last_tick_time.set(now);
            update_ship_xy();
            update_cannon_balls();
            tick_camera();
        }

        ctx.done()
    }
}

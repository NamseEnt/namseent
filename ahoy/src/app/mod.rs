mod ballistics;
mod mechanics;
mod objects;

use self::{
    mechanics::{Meter, MeterExt, Speed},
    objects::{
        cannon_ball::{CannonBalls, CANNON_BALLS_ATOM},
        fortress::Fortress,
        ship::{MutateShipKinetics, Ship, ShipKinetics, SHIP_KINETICS_ATOM},
    },
};
use namui::prelude::*;
use num_traits::{One, Zero};

static PX_PER_METER_ATOM: Atom<Per<Px, Meter>> = Atom::uninitialized_new();

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let now = namui::time::now();

        ctx.atom_init(&PX_PER_METER_ATOM, || Per::new(1.px(), Meter::one()));
        ctx.atom_init(&CANNON_BALLS_ATOM, Vec::new);
        ctx.atom_init(&SHIP_KINETICS_ATOM, || ShipKinetics {
            center_xy: Xy::single(100.meter()),
            yaw: 0.rad(),
            front_velocity: Speed::zero(),
        });

        ctx.component(Tick { now });
        ctx.component(Ship { now });
        ctx.component(Fortress {
            now,
            center_xy: Xy::single(200.meter()),
        });
        ctx.component(CannonBalls { now });

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

        let (cannon_balls, set_cannon_balls) = ctx.atom(&CANNON_BALLS_ATOM);
        let (ship_kinetics_atom, set_ship_kinetics_atom) = ctx.atom(&SHIP_KINETICS_ATOM);

        let (last_tick_time, set_last_tick_time) = ctx.state(|| now);

        let ShipKinetics {
            yaw,
            front_velocity,
            ..
        } = *ship_kinetics_atom;

        let dt = now - *last_tick_time;
        let update_ship_xy = || {
            let dl = front_velocity * dt;
            let dxy = Xy::single(dl) * yaw.as_xy();
            set_ship_kinetics_atom.mutate_center_xy(dxy);
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
            set_last_tick_time.set(now);
            update_ship_xy();
            update_cannon_balls();
        }

        ctx.done()
    }
}

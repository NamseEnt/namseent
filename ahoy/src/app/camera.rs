use super::mechanics::{Meter, MeterExt};
use keyframe::{ease, functions::EaseOutCubic};
use namui::prelude::*;
use num_traits::One;

pub static CAMERA_STATE_ATOM: Atom<CameraState> = Atom::uninitialized_new();

#[derive(Debug, Clone, Copy)]
pub struct CameraState {
    px_per_meter_at_tick: Per<Px, Meter>,
    start_px_per_meter: Per<Px, Meter>,
    end_px_per_meter: Per<Px, Meter>,
    px_per_meter_changed_at: Instant,
    screen_left_top_at_tick: Xy<Meter>,
    center_xy_at_tick: Xy<Meter>,
    start_center_xy: Xy<Meter>,
    end_center_xy: Xy<Meter>,
    center_xy_changed_at: Instant,
}
impl CameraState {
    pub fn new(px_per_meter: Per<Px, Meter>, center_xy: Xy<Meter>) -> Self {
        let half_screen = half_screen(px_per_meter);
        Self {
            px_per_meter_at_tick: px_per_meter,
            start_px_per_meter: px_per_meter,
            end_px_per_meter: px_per_meter,
            px_per_meter_changed_at: Instant::new(0.sec()),
            screen_left_top_at_tick: center_xy - half_screen,
            center_xy_at_tick: center_xy,
            start_center_xy: center_xy,
            end_center_xy: center_xy,
            center_xy_changed_at: Instant::new(0.sec()),
        }
    }
    pub fn px_per_meter(&self) -> Per<Px, Meter> {
        self.px_per_meter_at_tick
    }
    pub fn end_px_per_meter(&self) -> Per<Px, Meter> {
        self.end_px_per_meter
    }
    pub fn screen_left_top_xy(&self) -> Xy<Meter> {
        self.screen_left_top_at_tick
    }
    fn calculate_tween(&mut self, now: Instant) {
        let Self {
            start_px_per_meter,
            end_px_per_meter,
            px_per_meter_changed_at,
            start_center_xy,
            end_center_xy,
            center_xy_changed_at,
            ..
        } = self;

        let px_per_meter = ease(
            EaseOutCubic,
            (*start_px_per_meter * Meter::one()).as_f32(),
            (*end_px_per_meter * Meter::one()).as_f32(),
            (now - *px_per_meter_changed_at).as_secs_f32(),
        )
        .px();
        let px_per_meter = Per::new(px_per_meter, Meter::one());

        let center_xy = {
            let time = (now - *center_xy_changed_at).as_secs_f32();
            let start_x = start_center_xy.x.as_f32();
            let end_x = end_center_xy.x.as_f32();
            let x = ease(EaseOutCubic, start_x, end_x, time).meter();
            let start_y = start_center_xy.y.as_f32();
            let end_y = end_center_xy.y.as_f32();
            let y = ease(EaseOutCubic, start_y, end_y, time).meter();
            Xy::new(x, y)
        };

        let half_screen = half_screen(px_per_meter);

        self.px_per_meter_at_tick = px_per_meter;
        self.center_xy_at_tick = center_xy;
        self.screen_left_top_at_tick = center_xy - half_screen;
    }
}
pub trait MutateCameraState {
    fn mutate_px_per_meter(self, now: Instant, px_per_meter: Per<Px, Meter>);
    fn mutate_center_xy(self, now: Instant, center_xy: Xy<Meter>);
    fn mutate_tick(self, now: Instant);
}
impl MutateCameraState for SetState<CameraState> {
    fn mutate_px_per_meter(self, now: Instant, px_per_meter: Per<Px, Meter>) {
        self.mutate(move |state| {
            state.start_px_per_meter = state.px_per_meter_at_tick;
            state.end_px_per_meter = px_per_meter;
            state.px_per_meter_changed_at = now;
        });
    }
    fn mutate_center_xy(self, now: Instant, center_xy: Xy<Meter>) {
        self.mutate(move |state| {
            state.start_center_xy = state.center_xy_at_tick;
            state.end_center_xy = center_xy;
            state.center_xy_changed_at = now;
        });
    }
    fn mutate_tick(self, now: Instant) {
        self.mutate(move |state| {
            state.calculate_tween(now);
        });
    }
}
#[namui::component]
pub struct Camera {
    pub now: Instant,
}
impl Component for Camera {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { now } = self;

        let (camera_state, set_camera_state) = ctx.atom(&CAMERA_STATE_ATOM);

        ctx.on_raw_event(|event| {
            let RawEvent::Wheel { event } = event else {
                return;
            };

            let next_px_per_meter = {
                let prev = camera_state.end_px_per_meter() * Meter::one();
                Per::new(
                    (prev + (event.delta_xy.y / prev.as_f32()).px()).clamp(2.px(), 8.px()),
                    Meter::one(),
                )
            };
            set_camera_state.mutate_px_per_meter(now, next_px_per_meter);
        });

        ctx.done()
    }
}

fn half_screen(px_per_meter: Per<Px, Meter>) -> Xy<Meter> {
    let screen_wh = (screen::size().into_type::<Px>().into_type::<f32>()
        / (px_per_meter * Meter::one()).as_f32())
    .into_type::<Meter>();
    (screen_wh / 2.0).as_xy()
    // Xy::single(5.meter())
}

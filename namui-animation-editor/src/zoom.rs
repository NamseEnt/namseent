use namui::types::{Px, Time, TimePerPx};

pub(crate) fn zoom_time_per_px(target: TimePerPx, delta: f32) -> TimePerPx {
    const STEP: f32 = 400.0;
    const MIN: f32 = 10.0;
    const MAX: f32 = 1000.0;

    let ms_per_px = (target * Px::from(1.0_f32)).as_millis();

    let wheel = STEP * (ms_per_px / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed_ms = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
    Time::Ms(zoomed_ms) / Px::from(1.0_f32)
}

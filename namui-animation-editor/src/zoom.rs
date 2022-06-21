use namui::types::TimePerPixel;

pub(crate) fn zoom_time_per_pixel(target: TimePerPixel, delta: f32) -> TimePerPixel {
    const STEP: f32 = 400.0;
    const MIN: f32 = 10.0;
    const MAX: f32 = 1000.0;

    let ms_per_pixel = target.ms_per_pixel();

    let wheel = STEP * (ms_per_pixel / 10.0).log2();

    let next_wheel = wheel + delta;

    let zoomed = namui::math::num::clamp(10.0 * 2.0f32.powf(next_wheel / STEP), MIN, MAX);
    TimePerPixel::from_ms_per_pixel(zoomed)
}

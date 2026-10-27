mod beam;
mod charge_lines;
mod magic_stars;
mod muzzle_flash;

pub use beam::Beam;
pub use charge_lines::ChargeLines;
pub use magic_stars::MagicStars;
pub use muzzle_flash::MuzzleFlash;

use namui::*;

pub(super) const MAX_LIFETIME: Duration = Duration::from_millis(650);
pub(crate) const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
pub(crate) const ROYAL_PURPLE: [f32; 3] = [0.92, 0.7, 1.0];
pub(crate) const ROYAL_VIOLET: [f32; 3] = [0.78, 0.62, 1.0];
pub(crate) const ROYAL_PINK: [f32; 3] = [1.0, 0.58, 0.85];
pub(crate) const ROYAL_RED: [f32; 3] = [1.0, 0.58, 0.25];
pub(crate) const ROYAL_GOLD: [f32; 3] = [1.0, 0.7, 0.2];
pub(crate) const ROYAL_VAPOR: [f32; 3] = [0.86, 0.88, 0.9];
pub(crate) const STAR_COLORS: [[f32; 3]; 3] = [ROYAL_VIOLET, ROYAL_PINK, ROYAL_RED];

pub(super) fn progress_after_delay(
    elapsed: Duration,
    delay: Duration,
    duration: Duration,
) -> Option<f32> {
    if elapsed < delay || elapsed >= delay + duration {
        return None;
    }
    Some(((elapsed - delay).as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0))
}

pub(super) fn frame_index(progress: f32, frame_count: usize) -> usize {
    ((progress.clamp(0.0, 1.0) * frame_count as f32) as usize).min(frame_count - 1)
}

pub(super) fn point_at(start_xy: (f32, f32), direction: (f32, f32), distance: f32) -> (f32, f32) {
    (
        start_xy.0 + direction.0 * distance,
        start_xy.1 + direction.1 * distance,
    )
}

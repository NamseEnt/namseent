use super::{FAB_SIZE, FAB_WIDTH, FabSurface};
use crate::icon::IconKind;
use crate::sound::{self, EmitSoundParams, SoundGroup, SpatialMode, VolumePreset};
use crate::theme::palette;
use namui::*;

const FAB_LONG_PRESS_MIN_RELEASE_SECONDS: f32 = 0.18;
const FAB_LONG_PRESS_MIN_VISIBLE_PROGRESS: f32 = 0.08;
const FAB_LONG_PRESS_INITIAL_SOUND_INTERVAL: f32 = 0.24;
const FAB_LONG_PRESS_FINAL_SOUND_INTERVAL: f32 = 0.06;
const FAB_LONG_PRESS_INDICATOR_MIN_RADIUS: Px = px(56.0);
const FAB_LONG_PRESS_INDICATOR_RADIUS: Px = px(84.0);

#[derive(Clone, Copy, State)]
struct FabLongPressState {
    press_start_time: Option<Instant>,
    accumulated_seconds: f32,
    release_time: Option<Instant>,
    release_duration_seconds: f32,
    last_sound_time: Option<Instant>,
    completed: bool,
}

impl FabLongPressState {
    fn new() -> Self {
        Self {
            press_start_time: None,
            accumulated_seconds: 0.0,
            release_time: None,
            release_duration_seconds: 0.0,
            last_sound_time: None,
            completed: false,
        }
    }

    fn progress(self, now: Instant, duration_seconds: f32) -> f32 {
        if self.completed {
            return 1.0;
        }

        let seconds = if let Some(start_time) = self.press_start_time {
            self.accumulated_seconds + (now - start_time).as_secs_f32()
        } else if let Some(release_time) = self.release_time {
            let elapsed = (now - release_time).as_secs_f32();
            self.accumulated_seconds * (1.0 - elapsed / self.release_duration_seconds.max(0.001))
        } else {
            self.accumulated_seconds
        };

        (seconds / duration_seconds.max(0.001)).clamp(0.0, 1.0)
    }

    fn start_press(&mut self, now: Instant, duration_seconds: f32) {
        self.accumulated_seconds = self.progress(now, duration_seconds) * duration_seconds;
        self.press_start_time = Some(now);
        self.release_time = None;
        self.release_duration_seconds = 0.0;
        self.last_sound_time = Some(now);
        self.completed = false;
    }

    fn release_press(&mut self, now: Instant, duration_seconds: f32) {
        self.accumulated_seconds = (self.progress(now, duration_seconds) * duration_seconds)
            .max(duration_seconds * FAB_LONG_PRESS_MIN_VISIBLE_PROGRESS);
        self.press_start_time = None;
        self.release_time = Some(now);
        self.release_duration_seconds = self
            .accumulated_seconds
            .max(FAB_LONG_PRESS_MIN_RELEASE_SECONDS);
        self.last_sound_time = None;
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

pub(super) struct FabLongPressButton<'a> {
    pub(super) visible_content_x: Px,
    pub(super) icon: IconKind,
    pub(super) disabled: bool,
    pub(super) duration: Duration,
    pub(super) on_click: &'a dyn Fn(),
}

impl Component for FabLongPressButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            visible_content_x,
            icon,
            disabled,
            duration,
            on_click,
        } = self;
        let wh = Wh::new(FAB_WIDTH, FAB_SIZE);
        let duration_seconds = duration.as_secs_f32().max(0.001);
        let now = Instant::now();
        let (state, set_state) = ctx.state(FabLongPressState::new);
        let mut state_value = *state;
        let has_active_state = state_value.press_start_time.is_some()
            || state_value.release_time.is_some()
            || state_value.accumulated_seconds > 0.0
            || state_value.completed;
        if disabled && has_active_state {
            state_value.reset();
            set_state.set(state_value);
        }
        let linear_progress = if disabled {
            0.0
        } else {
            state_value.progress(now, duration_seconds)
        };

        if !disabled && state_value.press_start_time.is_some() {
            if !state_value.completed {
                if linear_progress >= 1.0 {
                    play_fab_long_press_sound();
                    on_click();
                    state_value.completed = true;
                    state_value.press_start_time = None;
                    state_value.release_time = None;
                    state_value.accumulated_seconds = duration_seconds;
                    state_value.last_sound_time = None;
                } else if let Some(last_sound_time) = state_value.last_sound_time {
                    let elapsed = (now - last_sound_time).as_secs_f32();
                    if elapsed >= fab_long_press_sound_interval(linear_progress) {
                        play_fab_long_press_sound();
                        state_value.last_sound_time = Some(now);
                    }
                }
            }

            set_state.set(state_value);
        } else if state_value.release_time.is_some() && linear_progress <= 0.0 {
            state_value.reset();
            set_state.set(state_value);
        }

        let eased_progress = ease_out_cubic(linear_progress);
        let indicator_presence = crate::animation::with_spring(
            ctx,
            if linear_progress > 0.0 { 1.0 } else { 0.0 },
            0.0,
            |value| value * value,
            || 0.0,
        );
        let center = Xy::new(visible_content_x + FAB_SIZE / 2.0, FAB_SIZE / 2.0);
        let indicator_radius = FAB_LONG_PRESS_INDICATOR_MIN_RADIUS
            + (FAB_LONG_PRESS_INDICATOR_RADIUS - FAB_LONG_PRESS_INDICATOR_MIN_RADIUS)
                * indicator_presence;
        let indicator_alpha = (40.0 + 40.0 * indicator_presence) as u8;

        let ctx = ctx.mouse_cursor(if disabled {
            MouseCursor::Standard(StandardCursor::NotAllowed)
        } else {
            MouseCursor::Standard(StandardCursor::Pointer)
        });

        ctx.add(FabSurface {
            wh,
            visible_content_x,
            icon,
            disabled,
        });

        if indicator_presence > 0.001 {
            ctx.add(namui::path(
                Path::new().add_oval(Rect::Ltrb {
                    left: center.x - indicator_radius,
                    top: center.y - indicator_radius,
                    right: center.x + indicator_radius,
                    bottom: center.y + indicator_radius,
                }),
                Paint::new(palette::BLUE.with_alpha(indicator_alpha)).set_style(PaintStyle::Fill),
            ));

            if eased_progress > 0.001 {
                let progress_radius = indicator_radius - 4.px();
                let oval = Rect::Ltrb {
                    left: center.x - progress_radius,
                    top: center.y - progress_radius,
                    right: center.x + progress_radius,
                    bottom: center.y + progress_radius,
                };
                let progress_path = if eased_progress >= 0.999 {
                    Path::new().add_oval(oval)
                } else {
                    Path::new()
                        .move_to(center.x, center.y)
                        .arc_to(oval, (-90.0).deg(), (360.0 * eased_progress).deg())
                        .close()
                };
                ctx.add(namui::path(
                    progress_path,
                    Paint::new(palette::BLUE.with_alpha(190)).set_style(PaintStyle::Fill),
                ));
            }
        }

        ctx.attach_event(move |event| {
            if disabled {
                return;
            }

            match event {
                Event::MouseDown { event } if event.is_local_xy_in() => {
                    event.stop_propagation();
                    let mut next = *state;
                    next.start_press(Instant::now(), duration_seconds);
                    play_fab_long_press_sound();
                    set_state.set(next);
                }
                Event::MouseUp { event } => {
                    let mut next = *state;
                    if next.completed {
                        next.reset();
                        set_state.set(next);
                    } else if next.press_start_time.is_some() {
                        next.release_press(Instant::now(), duration_seconds);
                        set_state.set(next);
                    }
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                    }
                }
                _ => {}
            }
        });
    }
}

fn fab_long_press_sound_interval(progress: f32) -> f32 {
    FAB_LONG_PRESS_INITIAL_SOUND_INTERVAL
        + (FAB_LONG_PRESS_FINAL_SOUND_INTERVAL - FAB_LONG_PRESS_INITIAL_SOUND_INTERVAL)
            * progress.clamp(0.0, 1.0)
}

fn ease_out_cubic(value: f32) -> f32 {
    1.0 - (1.0 - value.clamp(0.0, 1.0)).powi(3)
}

fn play_fab_long_press_sound() {
    sound::emit_sound(EmitSoundParams::one_shot(
        sound::random_small_button(),
        SoundGroup::Ui,
        VolumePreset::Medium,
        SpatialMode::NonSpatial,
    ));
}

#[cfg(test)]
mod tests {
    use super::{ease_out_cubic, fab_long_press_sound_interval};

    #[test]
    fn long_press_sound_interval_shortens_as_progress_increases() {
        assert!(fab_long_press_sound_interval(0.0) > fab_long_press_sound_interval(0.5));
        assert!(fab_long_press_sound_interval(0.5) > fab_long_press_sound_interval(1.0));
        assert!((fab_long_press_sound_interval(1.0) - 0.06).abs() < 0.001);
    }

    #[test]
    fn long_press_visual_progress_fills_quickly_at_the_start() {
        let early = ease_out_cubic(0.25);
        let late = ease_out_cubic(0.75) - ease_out_cubic(0.5);
        assert!(early > late);
    }
}

use super::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use crate::sound::{self, EmitSoundParams, SoundGroup, SpatialMode, VolumePreset};
use namui::*;

/// Long press 상태를 관리하는 구조체
#[derive(Clone, Copy, State)]
struct LongPressState {
    press_start_time: Option<Instant>,
    accumulated_time: Duration,
    release_time: Option<Instant>,
}

impl LongPressState {
    fn new() -> Self {
        Self {
            press_start_time: None,
            accumulated_time: Duration::from_secs(0),
            release_time: None,
        }
    }

    /// 현재 진행 시간을 계산 (음수가 되지 않도록 보장)
    fn current_progress(&self) -> Duration {
        if let Some(start_time) = self.press_start_time {
            // 버튼을 누르고 있는 중
            let pressing_duration = Instant::now() - start_time;
            self.accumulated_time + pressing_duration
        } else if let Some(rel_time) = self.release_time {
            // 버튼을 뗀 후 감소 중
            let elapsed_since_release = Instant::now() - rel_time;
            if elapsed_since_release > self.accumulated_time {
                Duration::from_secs(0)
            } else {
                self.accumulated_time - elapsed_since_release
            }
        } else {
            self.accumulated_time
        }
    }

    /// 버튼을 누르기 시작할 때 호출
    fn on_press_start(&mut self) {
        // 감소 중이었다면 현재 누적 시간을 고정
        if self.release_time.is_some() {
            self.accumulated_time = self.current_progress();
        }
        self.press_start_time = Some(Instant::now());
        self.release_time = None;
    }

    /// 버튼을 뗄 때 호출
    fn on_press_end(&mut self) {
        if let Some(start_time) = self.press_start_time {
            let pressing_duration = Instant::now() - start_time;
            self.accumulated_time += pressing_duration;
        }
        self.press_start_time = None;
        self.release_time = Some(Instant::now());
    }

    /// 트리거 완료 후 초기화
    fn reset(&mut self) {
        *self = Self::new();
    }

    /// 진행률이 0에 도달했는지 확인
    fn is_depleted(&self) -> bool {
        self.release_time.is_some() && self.current_progress().as_secs_f32() <= 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
#[allow(dead_code)]
pub enum ButtonVariant {
    Text,
    Contained,
    Outlined,
    Fab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
#[allow(dead_code)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

pub struct Button<'a> {
    pub wh: Wh<Px>,
    pub on_click: &'a dyn Fn(),
    pub content: &'a dyn Fn(Wh<Px>, Color, &RenderCtx),
    pub variant: ButtonVariant,
    pub color: ButtonColor,
    pub disabled: bool,
    pub long_press_time: Option<Duration>,
}

#[allow(dead_code)]
impl<'a> Button<'a> {
    pub fn new(
        wh: Wh<Px>,
        on_click: &'a dyn Fn(),
        content: &'a dyn Fn(Wh<Px>, Color, &RenderCtx),
    ) -> Self {
        Self {
            wh,
            on_click,
            content,
            variant: ButtonVariant::Contained,
            color: ButtonColor::Primary,
            disabled: false,
            long_press_time: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn long_press_time(mut self, duration: Duration) -> Self {
        self.long_press_time = Some(duration);
        self
    }
}

impl Component for Button<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Button {
            wh,
            on_click,
            content,
            variant,
            color,
            disabled,
            long_press_time,
        } = self;

        let (button_state, set_button_state) = ctx.state(|| {
            if disabled {
                ButtonState::Disabled
            } else {
                ButtonState::Normal
            }
        });

        let (long_press_state, set_long_press_state) = ctx.state(LongPressState::new);
        let (long_press_sound_started_at, set_long_press_sound_started_at) =
            ctx.state(|| None::<Instant>);
        let (last_long_press_sound_elapsed, set_last_long_press_sound_elapsed) =
            ctx.state(|| None::<f32>);

        let current_state = if disabled {
            ButtonState::Disabled
        } else {
            *button_state
        };

        let fill_color = get_button_fill_color(variant, color, current_state);

        let base_colors = get_base_colors(color);
        let text_color = if disabled {
            palette::ON_SURFACE.with_alpha(97) // 0.38 * 255 ≈ 97
        } else {
            match variant {
                ButtonVariant::Text | ButtonVariant::Outlined => base_colors.0,
                ButtonVariant::Contained | ButtonVariant::Fab => base_colors.1,
            }
        };

        content(wh, text_color, ctx);

        let cursor = if disabled {
            MouseCursor::Standard(StandardCursor::NotAllowed)
        } else {
            MouseCursor::Standard(StandardCursor::Pointer)
        };

        // Long press 프로그레스 오버레이 렌더링
        if let Some(duration) = long_press_time {
            let mut state = *long_press_state;
            let current_progress = state.current_progress();

            // 진행률이 0에 도달하면 상태 초기화
            if state.is_depleted() {
                state.reset();
                set_long_press_state.set(state);
            }

            let linear_progress =
                (current_progress.as_secs_f32() / duration.as_secs_f32()).min(1.0);
            let progress = apply_ease_out_cubic(linear_progress);

            if progress > 0.0 {
                const OVERLAY_DARKEN_FACTOR: f32 = 0.1;
                const OVERLAY_ALPHA: u8 = 128;

                let overlay_color =
                    darken_color(base_colors.0, OVERLAY_DARKEN_FACTOR).with_alpha(OVERLAY_ALPHA);

                ctx.add(rect(RectParam {
                    rect: Rect::Xywh {
                        x: px(0.0),
                        y: px(0.0),
                        width: wh.width * progress,
                        height: wh.height,
                    },
                    style: RectStyle {
                        stroke: None,
                        fill: Some(RectFill {
                            color: overlay_color,
                        }),
                        round: Some(RectRound {
                            radius: palette::ROUND,
                        }),
                    },
                }));
            }
        }

        let handle_button_event = move |event: Event<'_>| {
            if disabled {
                return;
            }

            match event {
                Event::MouseDown { event } => {
                    if event.is_local_xy_in() {
                        event.stop_propagation();
                        set_button_state.set(ButtonState::Pressed);

                        if long_press_time.is_some() {
                            play_random_button_click_sound();
                            let mut state = *long_press_state;
                            state.on_press_start();
                            set_long_press_state.set(state);
                            set_long_press_sound_started_at.set(Some(Instant::now()));
                            set_last_long_press_sound_elapsed.set(None);
                        }
                    }
                }
                Event::MouseUp { event } => {
                    let was_pressed = *button_state == ButtonState::Pressed;
                    let is_inside = event.is_local_xy_in();

                    set_button_state.set(if is_inside {
                        ButtonState::Hovered
                    } else {
                        ButtonState::Normal
                    });

                    if let Some(long_press_duration) = long_press_time {
                        let mut state = *long_press_state;
                        let total_progress = state.current_progress();

                        if is_inside && was_pressed && total_progress >= long_press_duration {
                            state.reset();
                        } else {
                            state.on_press_end();
                        }
                        set_long_press_state.set(state);
                        set_long_press_sound_started_at.set(None);
                        set_last_long_press_sound_elapsed.set(None);
                    } else if is_inside && was_pressed {
                        play_random_button_click_sound();
                        on_click();
                    }
                }
                Event::MouseMove { event } => {
                    let is_hovering = event.is_local_xy_in();
                    let new_state = match (*button_state, is_hovering) {
                        (ButtonState::Pressed, _) => ButtonState::Pressed,
                        (_, true) => ButtonState::Hovered,
                        (_, false) => ButtonState::Normal,
                    };
                    if new_state != *button_state {
                        set_button_state.set(new_state);
                    }
                }
                _ => {}
            }
        };

        let ctx = ctx.mouse_cursor(cursor);
        let ctx = if variant == ButtonVariant::Text {
            ctx.add(rect(RectParam {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: px(0.0),
                    width: wh.width,
                    height: wh.height,
                },
                style: RectStyle {
                    stroke: None,
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }))
        } else {
            ctx.add(PaperContainerBackground {
                width: wh.width,
                height: wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Tape,
                color: if fill_color.a > 0 {
                    fill_color
                } else {
                    Color::TRANSPARENT
                },
                shadow: variant != ButtonVariant::Text,
                arrow: None,
            })
        };

        ctx.attach_event(handle_button_event);

        if let Some(long_press_duration) = long_press_time
            && let ButtonState::Pressed = *button_state
        {
            if let Some(started_at) = *long_press_sound_started_at {
                let elapsed = (Instant::now() - started_at).as_secs_f32();
                let interval = long_press_repeat_interval(elapsed);
                let should_play = match *last_long_press_sound_elapsed {
                    Some(last_elapsed) => elapsed - last_elapsed >= interval,
                    None => elapsed >= interval,
                };

                if should_play {
                    play_random_button_click_sound();
                    set_last_long_press_sound_elapsed.set(Some(elapsed));
                }
            }

            let mut state = *long_press_state;
            let total_progress = state.current_progress();

            if total_progress >= long_press_duration {
                play_random_button_click_sound();
                on_click();
                state.reset();
                set_long_press_sound_started_at.set(Some(Instant::now()));
                set_last_long_press_sound_elapsed.set(None);
            }
        }
    }
}

fn long_press_repeat_interval(hold_elapsed_secs: f32) -> f32 {
    0.25 - 0.2 * hold_elapsed_secs.clamp(0.0, 1.0)
}

/// easeOutCubic 함수: 1 - (1-t)³
fn apply_ease_out_cubic(t: f32) -> f32 {
    let inverse = 1.0 - t;
    1.0 - (inverse * inverse * inverse)
}

fn get_button_fill_color(variant: ButtonVariant, color: ButtonColor, state: ButtonState) -> Color {
    let base_colors = get_base_colors(color);

    match (variant, state) {
        (ButtonVariant::Text, ButtonState::Normal) => Color::TRANSPARENT,
        (ButtonVariant::Text, ButtonState::Hovered) => lighten_color(base_colors.0, 0.1),
        (ButtonVariant::Text, ButtonState::Pressed) => lighten_color(base_colors.0, 0.2),
        (ButtonVariant::Text, ButtonState::Disabled) => Color::TRANSPARENT,

        (ButtonVariant::Contained, ButtonState::Normal) => base_colors.0,
        (ButtonVariant::Contained, ButtonState::Hovered) => lighten_color(base_colors.0, 0.1),
        (ButtonVariant::Contained, ButtonState::Pressed) => lighten_color(base_colors.0, 0.2),
        (ButtonVariant::Contained, ButtonState::Disabled) => palette::DISABLED_CONTAINER,

        (ButtonVariant::Outlined, ButtonState::Normal) => Color::TRANSPARENT,
        (ButtonVariant::Outlined, ButtonState::Hovered) => lighten_color(base_colors.0, 0.1),
        (ButtonVariant::Outlined, ButtonState::Pressed) => lighten_color(base_colors.0, 0.2),
        (ButtonVariant::Outlined, ButtonState::Disabled) => Color::TRANSPARENT,

        (ButtonVariant::Fab, ButtonState::Normal) => base_colors.0,
        (ButtonVariant::Fab, ButtonState::Hovered) => lighten_color(base_colors.0, 0.1),
        (ButtonVariant::Fab, ButtonState::Pressed) => lighten_color(base_colors.0, 0.2),
        (ButtonVariant::Fab, ButtonState::Disabled) => palette::DISABLED_CONTAINER,
    }
}

fn get_base_colors(color: ButtonColor) -> (Color, Color) {
    match color {
        ButtonColor::Primary => (palette::PRIMARY, palette::ON_PRIMARY),
        ButtonColor::Secondary => (palette::SECONDARY, palette::ON_SECONDARY),
        ButtonColor::Error => (palette::RED, palette::WHITE),
        ButtonColor::Warn => (palette::YELLOW, palette::BLACK),
        ButtonColor::Info => (palette::BLUE, palette::WHITE),
    }
}

fn lighten_color(color: Color, factor: f32) -> Color {
    let r = ((color.r as f32 / 255.0) + factor).min(1.0);
    let g = ((color.g as f32 / 255.0) + factor).min(1.0);
    let b = ((color.b as f32 / 255.0) + factor).min(1.0);

    Color::from_f01(r, g, b, color.a as f32 / 255.0)
}

fn darken_color(color: Color, factor: f32) -> Color {
    let r = ((color.r as f32 / 255.0) - factor).max(0.0);
    let g = ((color.g as f32 / 255.0) - factor).max(0.0);
    let b = ((color.b as f32 / 255.0) - factor).max(0.0);

    Color::from_f01(r, g, b, color.a as f32 / 255.0)
}

fn play_random_button_click_sound() {
    let asset = sound::random_bubble_pop();

    sound::emit_sound(EmitSoundParams::one_shot(
        asset,
        SoundGroup::Ui,
        VolumePreset::Medium,
        SpatialMode::NonSpatial,
    ));
}

use super::LightFrame;
use crate::app::theme::THEME;
use keyframe::{ease, functions::EaseOutCubic};
use namui::{prelude::*, time::since_start};
use namui_prebuilt::typography::{self, adjust_font_size};

#[component]
pub struct ButtonHoverEffect {
    pub wh: Wh<Px>,
    pub focused: bool,
}
impl Component for ButtonHoverEffect {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, focused } = self;

        let focused = ctx.track_eq(&focused);
        let (focus, set_selection) = ctx.state(DelayedFocus::default);

        ctx.effect("handle selection changed", || {
            let focused = *focused;
            if focused == focus.focused {
                return;
            }
            set_selection.mutate(move |selection| match focused {
                true => selection.focus(),
                false => selection.blur(),
            });
        });

        ctx.component(namui::path(
            Path::new().add_rect(Rect::zero_wh(wh)),
            Paint::new(
                THEME
                    .primary
                    .main
                    .with_alpha((128.0 * focus.intensity()) as u8),
            )
            .set_blend_mode(BlendMode::Plus)
            .set_mask_filter(MaskFilter::Blur {
                blur: Blur::Normal {
                    sigma: Blur::convert_radius_to_sigma(16.0),
                },
            }),
        ));

        ctx.done()
    }
}

#[component]
pub struct FilledButton<'a> {
    pub wh: Wh<Px>,
    pub text: String,
    pub on_click: &'a dyn Fn(),
    pub focused: bool,
}
impl Component for FilledButton<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            text,
            on_click,
            focused,
        } = self;

        let focused = ctx.track_eq(&focused);
        let (focus, set_selection) = ctx.state(DelayedFocus::default);
        let center_xy = wh / 2;

        ctx.effect("handle selection changed", || {
            let focused = *focused;
            if focused == focus.focused {
                return;
            }
            set_selection.mutate(move |selection| match focused {
                true => selection.focus(),
                false => selection.blur(),
            });
        });

        ctx.component(namui::path(
            Path::new().add_rect(Rect::zero_wh(wh)),
            Paint::new(
                THEME
                    .primary
                    .main
                    .with_alpha((128.0 * focus.intensity()) as u8),
            )
            .set_blend_mode(BlendMode::Plus)
            .set_mask_filter(MaskFilter::Blur {
                blur: Blur::Normal {
                    sigma: Blur::convert_radius_to_sigma(16.0),
                },
            }),
        ));

        ctx.component(namui::text(TextParam {
            text,
            x: center_xy.width,
            y: center_xy.height,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                size: typography::adjust_font_size(wh.height),
                name: THEME.font_name.to_string(),
            },
            style: TextStyle {
                color: THEME.text,
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.component(LightFrame { wh }.attach_event(|event| {
            let Event::MouseDown { event } = event else {
                return;
            };
            if !event.is_local_xy_in() {
                return;
            }
            on_click();
        }));

        ctx.done()
    }
}

#[component]
pub struct IconButton<'a> {
    pub wh: Wh<Px>,
    pub text: String,
    pub on_click: &'a dyn Fn(),
    pub focused: bool,
}
impl Component for IconButton<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            text,
            on_click,
            focused,
        } = self;

        ctx.component(TextButtonInner {
            wh,
            text,
            font: Font {
                size: adjust_font_size(wh.height),
                name: THEME.icon_font_name.to_string(),
            },
            on_click,
            focused,
        });

        ctx.done()
    }
}

#[component]
pub struct TextButton<'a> {
    pub wh: Wh<Px>,
    pub text: String,
    pub on_click: &'a dyn Fn(),
    pub focused: bool,
}
impl Component for TextButton<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            text,
            on_click,
            focused,
        } = self;

        ctx.component(TextButtonInner {
            wh,
            text,
            font: Font {
                size: adjust_font_size(wh.height),
                name: THEME.font_name.to_string(),
            },
            on_click,
            focused,
        });

        ctx.done()
    }
}

#[component]
struct TextButtonInner<'a> {
    wh: Wh<Px>,
    text: String,
    font: Font,
    on_click: &'a dyn Fn(),
    focused: bool,
}
impl Component for TextButtonInner<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            text,
            font,
            on_click,
            focused,
        } = self;

        let focused = ctx.track_eq(&focused);
        let (focus, set_selection) = ctx.state(DelayedFocus::default);

        ctx.effect("handle selection changed", || {
            let focused = *focused;
            if focused == focus.focused {
                return;
            }
            set_selection.mutate(move |selection| match focused {
                true => selection.focus(),
                false => selection.blur(),
            });
        });

        ctx.component(TextDrawCommand {
            text: text.clone(),
            font: font.clone(),
            x: wh.width / 2,
            y: wh.height / 2,
            paint: Paint::new(
                THEME
                    .primary
                    .main
                    .with_alpha((255.0 * focus.intensity()) as u8),
            )
            .set_blend_mode(BlendMode::Screen)
            .set_mask_filter(MaskFilter::Blur {
                blur: Blur::Normal {
                    sigma: Blur::convert_radius_to_sigma(16.0),
                },
            }),
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            max_width: None,
            line_height_percent: 100.percent(),
            underline: None,
        });

        ctx.component(
            TextDrawCommand {
                text,
                font,
                x: wh.width / 2,
                y: wh.height / 2,
                paint: Paint::new(
                    THEME
                        .text
                        .with_alpha(216 + (39.0 * focus.intensity()) as u8),
                ),
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                max_width: None,
                line_height_percent: 100.percent(),
                underline: None,
            }
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                on_click();
            }),
        );

        ctx.done()
    }
}

#[derive(Debug)]
struct DelayedFocus {
    focused: bool,
    last_intensity: (Duration, f32),
    speed: Per<f32, Duration>,
}
impl DelayedFocus {
    fn new(delay: Duration) -> Self {
        Self {
            focused: false,
            last_intensity: (Duration::default(), 0.0),
            speed: Per::new(1.0, delay),
        }
    }

    fn focus(&mut self) {
        let intensity = self.intensity();
        self.focused = true;
        self.last_intensity = (since_start(), intensity);
    }
    fn blur(&mut self) {
        let intensity = self.intensity();
        self.focused = false;
        self.last_intensity = (since_start(), intensity);
    }

    /// 0.0 ~ 1.0
    fn intensity(&self) -> f32 {
        let (last_changed_at, last_intensity) = self.last_intensity;
        let elapsed = since_start() - last_changed_at;
        let delta = self.speed
            * match self.focused {
                true => elapsed,
                false => -(elapsed / 4.0),
            };
        ease(EaseOutCubic, 0.0, 1.0, last_intensity + delta)
    }
}
impl Default for DelayedFocus {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}

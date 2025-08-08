use super::palette;
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonVariant {
    Text,
    Contained,
    Outlined,
    Fab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        } = self;

        let (button_state, set_button_state) = ctx.state(|| {
            if disabled {
                ButtonState::Disabled
            } else {
                ButtonState::Normal
            }
        });

        let current_state = if disabled {
            ButtonState::Disabled
        } else {
            *button_state
        };

        let (fill_color, stroke_color, stroke_width) =
            get_button_style(variant, color, current_state);

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

        ctx.mouse_cursor(cursor)
            .add(rect(RectParam {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: px(0.0),
                    width: wh.width,
                    height: wh.height,
                },
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: stroke_color,
                        width: stroke_width,
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill { color: fill_color }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            }))
            .attach_event({
                move |event| {
                    if disabled {
                        return;
                    }
                    match event {
                        Event::MouseDown { event } => {
                            if event.is_local_xy_in() {
                                event.stop_propagation();
                                set_button_state.set(ButtonState::Pressed);
                            }
                        }
                        Event::MouseUp { event } => {
                            if event.is_local_xy_in() && *button_state == ButtonState::Pressed {
                                on_click();
                            }
                            set_button_state.set(if event.is_local_xy_in() {
                                ButtonState::Hovered
                            } else {
                                ButtonState::Normal
                            });
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
                }
            });
    }
}

fn get_button_style(
    variant: ButtonVariant,
    color: ButtonColor,
    state: ButtonState,
) -> (Color, Color, Px) {
    let base_colors = get_base_colors(color);

    match (variant, state) {
        (ButtonVariant::Text, ButtonState::Normal) => {
            (Color::TRANSPARENT, Color::TRANSPARENT, 0.px())
        }
        (ButtonVariant::Text, ButtonState::Hovered) => (
            lighten_color(base_colors.0, 0.1),
            Color::TRANSPARENT,
            0.px(),
        ),
        (ButtonVariant::Text, ButtonState::Pressed) => (
            lighten_color(base_colors.0, 0.2),
            Color::TRANSPARENT,
            0.px(),
        ),
        (ButtonVariant::Text, ButtonState::Disabled) => {
            (Color::TRANSPARENT, Color::TRANSPARENT, 0.px())
        }

        (ButtonVariant::Contained, ButtonState::Normal) => (base_colors.0, base_colors.0, 0.px()),
        (ButtonVariant::Contained, ButtonState::Hovered) => (
            lighten_color(base_colors.0, 0.1),
            lighten_color(base_colors.0, 0.1),
            0.px(),
        ),
        (ButtonVariant::Contained, ButtonState::Pressed) => (
            lighten_color(base_colors.0, 0.2),
            lighten_color(base_colors.0, 0.2),
            0.px(),
        ),
        (ButtonVariant::Contained, ButtonState::Disabled) => (
            palette::DISABLED_CONTAINER,
            palette::DISABLED_CONTAINER,
            0.px(),
        ),

        (ButtonVariant::Outlined, ButtonState::Normal) => {
            (Color::TRANSPARENT, base_colors.0, 1.px())
        }
        (ButtonVariant::Outlined, ButtonState::Hovered) => (
            lighten_color(base_colors.0, 0.1),
            lighten_color(base_colors.0, 0.1),
            1.px(),
        ),
        (ButtonVariant::Outlined, ButtonState::Pressed) => (
            lighten_color(base_colors.0, 0.2),
            lighten_color(base_colors.0, 0.2),
            1.px(),
        ),
        (ButtonVariant::Outlined, ButtonState::Disabled) => {
            (Color::TRANSPARENT, palette::OUTLINE, 1.px())
        }

        (ButtonVariant::Fab, ButtonState::Normal) => {
            (base_colors.0, darken_color(base_colors.0, 0.3), 5.px())
        }
        (ButtonVariant::Fab, ButtonState::Hovered) => (
            lighten_color(base_colors.0, 0.1),
            darken_color(base_colors.0, 0.3),
            5.px(),
        ),
        (ButtonVariant::Fab, ButtonState::Pressed) => (
            lighten_color(base_colors.0, 0.2),
            darken_color(base_colors.0, 0.3),
            5.px(),
        ),
        (ButtonVariant::Fab, ButtonState::Disabled) => (
            palette::DISABLED_CONTAINER,
            darken_color(palette::DISABLED_CONTAINER, 0.2),
            5.px(),
        ),
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

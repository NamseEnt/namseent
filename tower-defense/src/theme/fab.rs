mod long_press;

use crate::animation::xy_with_spring;
use crate::icon::IconKind;

use crate::theme::button::{Button, ButtonVariant};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::{
    palette,
    typography::{FontSize, memoized_text},
};
use crate::tooltip::{TooltipContent, TooltipPlacement, WithHoverArea};
use long_press::FabLongPressButton;
use namui::*;

const FAB_SIZE: Px = px(96.0);
const FAB_WIDTH: Px = px(144.0);
const FAB_ICON_SIZE: Px = px(84.0);
const FAB_SCREEN_MARGIN: Px = px(36.0);
const FAB_STACK_PADDING: Px = px(12.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum FabSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum FabVerticalPosition {
    Top,
    Center,
    BottomPrimary,
    BottomSecondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct FabPosition {
    side: FabSide,
    vertical: FabVerticalPosition,
}

impl FabPosition {
    pub const fn new(side: FabSide, vertical: FabVerticalPosition) -> Self {
        Self { side, vertical }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FabLayout {
    target_xy: Xy<Px>,
    hidden_xy: Xy<Px>,
    visible_content_x: Px,
}

impl FabLayout {
    fn compute(screen_wh: Wh<Px>, position: FabPosition) -> Self {
        let visible_x = match position.side {
            FabSide::Left => -FAB_WIDTH + FAB_SIZE,
            FabSide::Right => screen_wh.width - FAB_SIZE,
        };
        let hidden_x = match position.side {
            FabSide::Left => -FAB_WIDTH - FAB_SIZE * 0.5,
            FabSide::Right => screen_wh.width + FAB_SIZE * 0.5,
        };
        let y = match position.vertical {
            FabVerticalPosition::Top => FAB_SCREEN_MARGIN,
            FabVerticalPosition::Center => (screen_wh.height - FAB_SIZE) * 0.5,
            FabVerticalPosition::BottomSecondary => screen_wh.height - FAB_SCREEN_MARGIN - FAB_SIZE,
            FabVerticalPosition::BottomPrimary => {
                screen_wh.height - FAB_SCREEN_MARGIN - FAB_SIZE - FAB_STACK_PADDING - FAB_SIZE
            }
        };
        let visible_content_x = match position.side {
            FabSide::Left => FAB_WIDTH - FAB_SIZE,
            FabSide::Right => 0.px(),
        };

        Self {
            target_xy: Xy::new(visible_x, y),
            hidden_xy: Xy::new(hidden_x, y),
            visible_content_x,
        }
    }

    pub fn bottom_reserved_height() -> Px {
        FAB_SCREEN_MARGIN + FAB_SIZE + FAB_STACK_PADDING + FAB_SIZE
    }

    const fn tooltip_placement(side: FabSide) -> TooltipPlacement {
        match side {
            FabSide::Left => TooltipPlacement::RightOf,
            FabSide::Right => TooltipPlacement::LeftOf,
        }
    }
}

pub struct FloatingActionButton<'a> {
    pub screen_wh: Wh<Px>,
    pub position: FabPosition,
    pub visible: bool,
    pub icon: IconKind,
    pub disabled: bool,
    pub long_press_time: Option<Duration>,
    pub on_click: &'a dyn Fn(),
    pub tooltip_content: Option<TooltipContent>,
}

struct FabSurface {
    wh: Wh<Px>,
    visible_content_x: Px,
    icon: IconKind,
    disabled: bool,
}

impl Component for FabSurface {
    fn render(self, ctx: &RenderCtx) {
        let visible_wh = Wh::single(FAB_SIZE);
        ctx.translate((self.visible_content_x, 0.px()))
            .add(memoized_text(&visible_wh, |mut builder| {
                builder
                    .headline()
                    .size(FontSize::Custom {
                        size: FAB_ICON_SIZE,
                    })
                    .color(if self.disabled {
                        palette::ON_SURFACE.with_alpha(96)
                    } else {
                        palette::ON_SURFACE
                    })
                    .stroke(3.px(), palette::OUTLINE)
                    .icon(self.icon)
                    .render_center(visible_wh)
            }));
        ctx.add(PaperContainerBackground {
            width: self.wh.width,
            height: self.wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::PaperSingleLayer,
            color: palette::SURFACE_CONTAINER_HIGH,
            outline_color: Some(palette::OUTLINE.with_alpha(180)),
            shadow: true,
            arrow: None,
        });
    }
}

impl Component for FloatingActionButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            position,
            visible,
            icon,
            disabled,
            long_press_time,
            on_click,
            tooltip_content,
        } = self;
        let layout = FabLayout::compute(screen_wh, position);
        let animated_xy = xy_with_spring(
            ctx,
            if visible {
                layout.target_xy
            } else {
                layout.hidden_xy
            },
            layout.hidden_xy,
        );
        let visible_content_x = layout.visible_content_x;
        let tooltip_placement = FabLayout::tooltip_placement(position.side);

        let content = move |wh: Wh<Px>, _color: Color, ctx: &RenderCtx| {
            ctx.add(FabSurface {
                wh,
                visible_content_x,
                icon,
                disabled,
            });
        };
        if let Some(duration) = long_press_time {
            ctx.absolute(animated_xy).add(WithHoverArea {
                component_key: format!(
                    "fab:long-press:{:?}:{:?}:{visible}",
                    position.side, position.vertical
                ),
                component: FabLongPressButton {
                    visible_content_x,
                    icon,
                    disabled: disabled || !visible,
                    duration,
                    on_click,
                },
                placement: tooltip_placement,
                on_enter: move || {
                    if visible {
                        tooltip_content.clone()
                    } else {
                        None
                    }
                },
                on_exit: || {},
            });
        } else {
            let button = Button::new(Wh::new(FAB_WIDTH, FAB_SIZE), on_click, &content)
                .disabled(disabled || !visible)
                .variant(ButtonVariant::Text);

            ctx.absolute(animated_xy).add(WithHoverArea {
                component_key: format!("fab:{:?}:{:?}:{visible}", position.side, position.vertical),
                component: button,
                placement: tooltip_placement,
                on_enter: move || {
                    if visible {
                        tooltip_content.clone()
                    } else {
                        None
                    }
                },
                on_exit: || {},
            });
        }
    }
}

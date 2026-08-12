use crate::animation::xy_with_spring;
use crate::icon::IconKind;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::{
    palette,
    typography::{FontSize, memoized_text},
};
use crate::tooltip::{TooltipContent, TooltipPlacement, WithHoverArea};
use namui::*;

pub const FAB_SIZE: Px = px(96.0);
pub const FAB_WIDTH: Px = px(144.0);
pub const FAB_ICON_SIZE: Px = px(84.0);
pub const FAB_SCREEN_MARGIN: Px = px(36.0);
pub const FAB_STACK_PADDING: Px = px(12.0);

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
    pub side: FabSide,
    pub vertical: FabVerticalPosition,
}

impl FabPosition {
    pub const fn new(side: FabSide, vertical: FabVerticalPosition) -> Self {
        Self { side, vertical }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FabLayout {
    pub target_xy: Xy<Px>,
    pub hidden_xy: Xy<Px>,
    pub visible_content_x: Px,
}

impl FabLayout {
    pub fn compute(screen_wh: Wh<Px>, position: FabPosition) -> Self {
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

    pub const fn tooltip_placement(side: FabSide) -> TooltipPlacement {
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
    pub on_click: &'a dyn Fn(),
    pub tooltip_content: Option<TooltipContent>,
}

impl Component for FloatingActionButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            screen_wh,
            position,
            visible,
            icon,
            disabled,
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

        ctx.absolute(animated_xy).add(WithHoverArea {
            component_key: format!("fab:{:?}:{:?}:{visible}", position.side, position.vertical),
            component: Button::new(
                Wh::new(FAB_WIDTH, FAB_SIZE),
                on_click,
                &move |wh, _color, ctx| {
                    let visible_wh = Wh::single(FAB_SIZE);
                    ctx.translate((visible_content_x, 0.px()))
                        .add(memoized_text(&visible_wh, |mut builder| {
                            builder
                                .headline()
                                .size(FontSize::Custom {
                                    size: FAB_ICON_SIZE,
                                })
                                .color(if disabled {
                                    palette::ON_SURFACE.with_alpha(96)
                                } else {
                                    palette::ON_SURFACE
                                })
                                .stroke(3.px(), palette::OUTLINE)
                                .icon(icon)
                                .render_center(visible_wh)
                        }));
                    ctx.add(PaperContainerBackground {
                        width: wh.width,
                        height: wh.height,
                        texture: PaperTexture::Rough,
                        variant: PaperVariant::PaperSingleLayer,
                        color: palette::SURFACE_CONTAINER_HIGH,
                        outline_color: Some(palette::OUTLINE.with_alpha(180)),
                        shadow: true,
                        arrow: None,
                    });
                },
            )
            .disabled(disabled || !visible)
            .variant(ButtonVariant::Text),
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

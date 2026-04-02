use super::PADDING;
use crate::game_state::upgrade::Upgrade;
use crate::game_state::{flow::GameFlow, mutate_game_state};
use crate::l10n;
use crate::theme::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
    typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

pub struct TreasureCard {
    pub wh: Wh<Px>,
    pub upgrade: Upgrade,
    pub locale: crate::l10n::Locale,
    pub index: usize,
}

struct TreasureCardContent {
    wh: Wh<Px>,
    upgrade: Upgrade,
    locale: crate::l10n::Locale,
}

impl Component for TreasureCardContent {
    fn render(self, ctx: &RenderCtx) {
        let TreasureCardContent {
            wh,
            upgrade,
            locale,
        } = self;

        ctx.compose(|ctx| {
            table::padding_no_clip(PADDING, |inner_wh, inner_ctx| {
                table::horizontal([
                    table::fixed_no_clip(wh.height, |thumb_wh, ctx| {
                        ctx.compose(|ctx| {
                            table::padding_no_clip(PADDING, |inner_wh, inner_ctx| {
                                inner_ctx.add(upgrade.kind.thumbnail(inner_wh));
                            })(thumb_wh, ctx);
                        });
                        ctx.add(PaperContainerBackground {
                            width: thumb_wh.width,
                            height: thumb_wh.height,
                            texture: PaperTexture::Rough,
                            variant: PaperVariant::PaperSingleLayer,
                            color: palette::SURFACE_CONTAINER_LOWEST,
                            shadow: false,
                            arrow: None,
                        });
                    }),
                    table::fixed_no_clip(PADDING, |_, _| {}),
                    table::ratio_no_clip(1, |text_wh, ctx| {
                        ctx.compose(|ctx| {
                            table::vertical([
                                table::fixed_no_clip(24.px(), |_, ctx| {
                                    ctx.add(memoized_text((), |mut builder| {
                                        builder
                                            .headline()
                                            .size(FontSize::Large)
                                            .l10n(
                                                l10n::upgrade::UpgradeKindText::Name(&upgrade.kind),
                                                &locale,
                                            )
                                            .render_left_top()
                                    }));
                                }),
                                table::fixed_no_clip(6.px(), |_, _| {}),
                                table::ratio_no_clip(1, |desc_wh, ctx| {
                                    ctx.add(AutoScrollViewWithCtx {
                                        wh: desc_wh,
                                        scroll_bar_width: PADDING,
                                        content: |ctx| {
                                            ctx.add(memoized_text((), |mut builder| {
                                                builder
                                                    .paragraph()
                                                    .size(FontSize::Medium)
                                                    .l10n(
                                                        l10n::upgrade::UpgradeKindText::Description(
                                                            &upgrade.kind,
                                                        ),
                                                        &locale,
                                                    )
                                                    .render_left_top()
                                            }));
                                        },
                                    });
                                }),
                            ])(text_wh, ctx);
                        });
                    }),
                ])(inner_wh, inner_ctx);
            })(wh, ctx);
        });
    }
}

impl Component for TreasureCard {
    fn render(self, ctx: &RenderCtx) {
        let TreasureCard {
            wh,
            upgrade,
            locale,
            index,
        } = self;

        let (hovered, set_hovered) = ctx.state(|| false);
        let target_scale = if *hovered {
            Xy::single(1.05)
        } else {
            Xy::single(1.0)
        };
        let animated_scale = crate::hand::xy_with_spring(ctx, target_scale, Xy::single(1.0));
        let half_wh = wh.to_xy() * 0.5;

        let ctx = ctx
            .mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
            .translate(half_wh)
            .scale(animated_scale)
            .translate(-half_wh);

        ctx.compose(|ctx| {
            ctx.add(TreasureCardContent {
                wh,
                upgrade,
                locale,
            });

            ctx.add(
                simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                    move |event| match event {
                        Event::MouseMove { event } => {
                            if event.is_local_xy_in() {
                                set_hovered.set(true);
                            } else if *hovered {
                                set_hovered.set(false);
                            }
                        }
                        Event::MouseUp { event } => {
                            if event.is_local_xy_in() {
                                event.stop_propagation();
                                mutate_game_state(move |gs| {
                                    if let GameFlow::TreasureSelection(flow) = &mut gs.flow
                                        && flow.pending_selection.is_none()
                                    {
                                        flow.pending_selection = Some(index);
                                    }
                                });
                            }
                        }
                        _ => {}
                    },
                ),
            );
        });

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Paper,
            color: palette::SURFACE_CONTAINER_LOW,
            shadow: true,
            arrow: None,
        });
    }
}

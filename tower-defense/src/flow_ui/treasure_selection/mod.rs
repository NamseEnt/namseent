use crate::game_state::upgrade::Upgrade;
use crate::game_state::{flow::GameFlow, mutate_game_state, use_game_state};
use crate::hand::xy_with_spring;
use crate::l10n;
use crate::theme::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
    typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const MODAL_WIDTH_RATIO: f32 = 0.6;
const MODAL_HEIGHT_RATIO: f32 = 0.375; // 전체 높이 절반으로 감소
const PADDING: Px = px(16.0);
const CARD_GAP: Px = px(24.0);

pub struct TreasureSelectionUi;

impl Component for TreasureSelectionUi {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let locale = game_state.text().locale();

        let options = if let GameFlow::TreasureSelection(flow) = &game_state.flow {
            &flow.options
        } else {
            return;
        };

        let screen_wh = screen::size().into_type::<Px>();
        let modal_wh = Wh::new(
            screen_wh.width * MODAL_WIDTH_RATIO,
            screen_wh.height * MODAL_HEIGHT_RATIO,
        );
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            let ctx = ctx.translate(modal_xy);

            ctx.compose(|ctx| {
                table::padding_no_clip(
                    PADDING,
                    table::vertical([
                        table::ratio(1, |_, _| {}),
                        table::fixed_no_clip(modal_wh.height - CARD_GAP * 2.0, |wh, ctx| {
                            ctx.compose(|ctx| {
                                let card_height = (wh.height - CARD_GAP * 2.0) / 3.0;
                                table::vertical([
                                    table::fixed_no_clip(card_height, |card_wh, ctx| {
                                        ctx.add(TreasureCard {
                                            wh: card_wh,
                                            upgrade: options[0],
                                            locale,
                                            index: 0,
                                        });
                                    }),
                                    table::fixed_no_clip(CARD_GAP, |_, _| {}),
                                    table::fixed_no_clip(card_height, |card_wh, ctx| {
                                        ctx.add(TreasureCard {
                                            wh: card_wh,
                                            upgrade: options[1],
                                            locale,
                                            index: 1,
                                        });
                                    }),
                                    table::fixed_no_clip(CARD_GAP, |_, _| {}),
                                    table::fixed_no_clip(card_height, |card_wh, ctx| {
                                        ctx.add(TreasureCard {
                                            wh: card_wh,
                                            upgrade: options[2],
                                            locale,
                                            index: 2,
                                        });
                                    }),
                                ])(wh, ctx);
                            });
                        }),
                        table::ratio(1, |_, _| {}),
                    ]),
                )(modal_wh, ctx);
            });
        });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK.with_alpha(180),
        ))
        .attach_event(|event| match event {
            Event::MouseDown { event } | Event::MouseUp { event } | Event::MouseMove { event } => {
                event.stop_propagation();
            }
            _ => {}
        });
    }
}

struct TreasureCard {
    wh: Wh<Px>,
    upgrade: Upgrade,
    locale: crate::l10n::Locale,
    index: usize,
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
                        ctx.add(upgrade.kind.thumbnail(thumb_wh));
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
        let animated_scale = xy_with_spring(ctx, target_scale, Xy::single(1.0));
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
                                    gs.select_treasure(index);
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

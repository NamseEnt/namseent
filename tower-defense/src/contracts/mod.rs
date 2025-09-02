use crate::{
    game_state::use_game_state,
    icon::{Icon, IconKind, IconSize},
    l10n::{TextManager, contract::ContractText},
    palette,
    theme::typography::{FontSize, HEADLINE_FONT_SIZE_LARGE, TextAlign, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, table};

const PANEL_WIDTH: Px = px(260.);
const PADDING: Px = px(4.);
const TITLE_HEIGHT: Px = px(36.);

pub struct ContractsPanel {
    pub screen_wh: Wh<Px>,
}

impl Component for ContractsPanel {
    fn render(self, render_ctx: &RenderCtx) {
        let game_state = use_game_state(render_ctx);
        let text_manager: TextManager = game_state.text();

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.clip(Path::new().add_rect(wh.to_rect()), ClipOp::Intersect)
                .add(AutoScrollViewWithCtx {
                    wh,
                    scroll_bar_width: PADDING,
                    content: |mut ctx| {
                        let content_width = wh.width;
                        for (index, c) in game_state.contracts.iter().enumerate() {
                            let content =
                                ctx.ghost_compose(format!("ContractItemContent {index}"), |ctx| {
                                    table::vertical([
                                        table::fixed(
                                            HEADLINE_FONT_SIZE_LARGE.into_px(),
                                            table::horizontal([
                                                // Rarity icon
                                                table::fixed(
                                                    HEADLINE_FONT_SIZE_LARGE.into_px(),
                                                    |wh, ctx| {
                                                        ctx.add(
                                                            Icon::new(IconKind::Rarity {
                                                                rarity: c.rarity,
                                                            })
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                        );
                                                    },
                                                ),
                                                table::ratio(1, move |wh, ctx| {
                                                    ctx.add(
                                                        headline("thumbnail".to_string())
                                                            .size(FontSize::Small)
                                                            .align(TextAlign::LeftCenter {
                                                                height: wh.height,
                                                            })
                                                            .max_width(wh.width)
                                                            .build(),
                                                    );
                                                }),
                                                table::fixed(
                                                    HEADLINE_FONT_SIZE_LARGE.into_px() * 2.0,
                                                    |wh, ctx| {
                                                        ctx.add(
                                                            headline(c.status.to_string())
                                                                .size(FontSize::Small)
                                                                .align(TextAlign::Center { wh })
                                                                .build(),
                                                        );
                                                    },
                                                ),
                                            ]),
                                        ),
                                        table::fixed(PADDING * 2.0, |_, _| {}),
                                        // Risk
                                        table::fit(table::FitAlign::LeftTop, move |compose_ctx| {
                                            let text =
                                                text_manager.contract(ContractText::Risk(&c.risk));
                                            compose_ctx.add(
                                                paragraph(text)
                                                    .size(FontSize::Medium)
                                                    .align(TextAlign::LeftTop)
                                                    .max_width(content_width)
                                                    .build_rich(),
                                            );
                                        }),
                                        table::fixed(PADDING, |_, _| {}),
                                        // Reward
                                        table::fit(table::FitAlign::LeftTop, move |compose_ctx| {
                                            let text = text_manager
                                                .contract(ContractText::Reward(&c.reward));
                                            compose_ctx.add(
                                                paragraph(text)
                                                    .size(FontSize::Medium)
                                                    .align(TextAlign::LeftTop)
                                                    .max_width(content_width)
                                                    .build_rich(),
                                            );
                                        }),
                                    ])(
                                        Wh::new(content_width, f32::MAX.px()), ctx
                                    );
                                });

                            let Some(content_wh) = bounding_box(&content).map(|rect| rect.wh())
                            else {
                                return;
                            };
                            let container_wh = content_wh + Wh::single(PADDING * 2.);
                            ctx.translate(Xy::single(PADDING)).add(content);
                            ctx.add(rect(RectParam {
                                rect: container_wh.to_rect(),
                                style: RectStyle {
                                    stroke: Some(RectStroke {
                                        color: palette::OUTLINE,
                                        width: 1.px(),
                                        border_position: BorderPosition::Inside,
                                    }),
                                    fill: Some(RectFill {
                                        color: palette::SURFACE_CONTAINER,
                                    }),
                                    round: Some(RectRound {
                                        radius: palette::ROUND,
                                    }),
                                },
                            }));

                            ctx = ctx.translate(Xy::new(0.px(), container_wh.height));
                        }
                    },
                });
        };

        render_ctx.compose(|ctx| {
            table::horizontal([
                table::fixed_no_clip(
                    PANEL_WIDTH,
                    table::padding(
                        PADDING,
                        table::vertical([
                            table::fixed(TITLE_HEIGHT, |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Quest).size(IconSize::Medium).wh(Wh {
                                    width: 32.px(),
                                    height: wh.height,
                                }));

                                ctx.add(rect(RectParam {
                                    rect: wh.to_rect(),
                                    style: RectStyle {
                                        stroke: Some(RectStroke {
                                            color: palette::OUTLINE,
                                            width: 1.px(),
                                            border_position: BorderPosition::Inside,
                                        }),
                                        fill: Some(RectFill {
                                            color: palette::SURFACE_CONTAINER,
                                        }),
                                        round: Some(RectRound {
                                            radius: palette::ROUND,
                                        }),
                                    },
                                }));
                            }),
                            table::fixed_no_clip(PADDING, |_, _| {}),
                            table::ratio(1, scroll_view),
                        ]),
                    ),
                ),
                table::ratio_no_clip(1, |_, _| {}),
            ])(self.screen_wh, ctx);
        });
    }
}

pub struct Contracts {
    pub screen_wh: Wh<Px>,
}
impl Component for Contracts {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(ContractsPanel {
            screen_wh: self.screen_wh,
        });
    }
}

use crate::{
    game_state::{item::use_item, mutate_game_state, use_game_state},
    icon::{Icon, IconKind, IconSize},
    l10n::ui::TopBarText,
    palette,
    theme::{
        button::Button,
        typography::{FontSize, HEADLINE_FONT_SIZE_LARGE, memoized_text},
    },
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, table};

const INVENTORY_WIDTH: Px = px(240.);
const PADDING: Px = px(4.);
const TITLE_HEIGHT: Px = px(36.);

pub struct Inventory {
    pub screen_wh: Wh<Px>,
}
impl Component for Inventory {
    fn render(self, render_ctx: &RenderCtx) {
        let game_state = use_game_state(render_ctx);

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.clip(Path::new().add_rect(wh.to_rect()), ClipOp::Intersect)
                .add(AutoScrollViewWithCtx {
                    wh,
                    scroll_bar_width: PADDING,
                    content: |mut ctx| {
                        let content_width = wh.width - PADDING * 2.;
                        for (item_index, item) in game_state.items.iter().enumerate() {
                            let locale = game_state.text().locale();
                            let name_text = item.name_text();
                            let desc_text = item.description_text();
                            let name_key = format!("{:?}:name", item.effect);
                            let desc_key = format!("{:?}:desc", item.effect);
                            let content = ctx.ghost_compose(
                                format!("InventoryItemContent {item_index}"),
                                |ctx| {
                                    table::vertical([
                                        table::fixed(
                                            HEADLINE_FONT_SIZE_LARGE.into_px(),
                                            table::horizontal([
                                                table::fixed(
                                                    HEADLINE_FONT_SIZE_LARGE.into_px(),
                                                    |wh, ctx| {
                                                        ctx.add(
                                                            Icon::new(IconKind::Rarity {
                                                                rarity: item.rarity,
                                                            })
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                        );
                                                    },
                                                ),
                                                table::fixed(
                                                    HEADLINE_FONT_SIZE_LARGE.into_px(),
                                                    |wh, ctx| {
                                                        ctx.add(item.effect.thumbnail(wh));
                                                    },
                                                ),
                                                table::ratio(1, |_, _| {}),
                                                table::fixed(
                                                    HEADLINE_FONT_SIZE_LARGE.into_px() * 3.0,
                                                    |wh, ctx| {
                                                        let can_use_result =
                                                            item.can_use(&game_state);
                                                        let is_disabled = can_use_result.is_err();

                                                        ctx.add(
                                                            Button::new(
                                                                wh,
                                                                &|| {
                                                                    mutate_game_state(
                                                                        move |game_state| {
                                                                            let item = game_state
                                                                                .items
                                                                                .remove(item_index);
                                                                            use_item(
                                                                                game_state, &item,
                                                                            );
                                                                        },
                                                                    );
                                                                },
                                                                &|wh, color, ctx| {
                                                                    ctx.add(memoized_text(
                                                                        (&color, &wh),
                                                                        |builder| {
                                                                            let use_text =
                                                                                game_state
                                                                                    .text()
                                                                                    .ui(
                                                                                    TopBarText::Use,
                                                                                );

                                                                            builder
                                                                                .headline()
                                                                                .size(
                                                                                    FontSize::Small,
                                                                                )
                                                                                .color(color)
                                                                                .text(use_text)
                                                                                .render_center(wh)
                                                                        },
                                                                    ));
                                                                },
                                                            )
                                                            .disabled(is_disabled),
                                                        );
                                                    },
                                                ),
                                            ]),
                                        ),
                                        table::fixed(PADDING * 2.0, |_, _| {}),
                                        table::fit(table::FitAlign::LeftTop, move |compose_ctx| {
                                            compose_ctx.add(memoized_text(
                                                (&name_key, &content_width, &locale.language),
                                                |builder| {
                                                    builder
                                                        .headline()
                                                        .size(FontSize::Small)
                                                        .max_width(content_width)
                                                        .l10n(name_text.clone(), &locale)
                                                        .render_left_top()
                                                },
                                            ));
                                        }),
                                        table::fixed(PADDING, |_, _| {}),
                                        table::fit(table::FitAlign::LeftTop, move |compose_ctx| {
                                            compose_ctx.add(memoized_text(
                                                (&content_width, &desc_key, &locale.language),
                                                |builder| {
                                                    builder
                                                        .paragraph()
                                                        .size(FontSize::Medium)
                                                        .max_width(content_width)
                                                        .l10n(desc_text.clone(), &locale)
                                                        .render_left_top()
                                                },
                                            ));
                                        }),
                                    ])(
                                        Wh::new(content_width, f32::MAX.px()), ctx
                                    );
                                },
                            );
                            let Some(content_wh) = content.bounding_box().map(|rect| rect.wh())
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
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(
                    INVENTORY_WIDTH,
                    table::padding(
                        PADDING,
                        table::vertical([
                            table::fixed(TITLE_HEIGHT, |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Item).size(IconSize::Medium).wh(Wh {
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
            ])(self.screen_wh, ctx);
        });
    }
}

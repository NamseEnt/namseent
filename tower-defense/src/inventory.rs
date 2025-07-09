use crate::{
    game_state::{
        MAX_INVENTORY_SLOT,
        cursor_preview::PreviewKind,
        item::{ ItemUsage, use_item},
        mutate_game_state, use_game_state,
    },
    l10n::ui::TopBarText,
    palette,
    theme::typography::{FontSize, HEADLINE_FONT_SIZE_LARGE, Headline, Paragraph, TextAlign},
};
use namui::*;
use namui_prebuilt::{button::TextButton, scroll_view::AutoScrollViewWithCtx, table};

const INVENTORY_WIDTH: Px = px(240.);
const PADDING: Px = px(4.);
const TITLE_HEIGHT: Px = px(36.);

pub struct Inventory {
    pub screen_wh: Wh<Px>,
}
impl Component for Inventory {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(
                    INVENTORY_WIDTH,
                    table::padding(
                        PADDING,
                        table::vertical([
                            table::fixed(TITLE_HEIGHT, |wh, ctx| {
                                ctx.add(Headline {
                                    text: format!("{} {}/{}", TopBarText::Inventory.to_korean(), game_state.items.len(), MAX_INVENTORY_SLOT),
                                    font_size: FontSize::Medium,
                                    text_align: TextAlign::Center { wh },
                                    max_width: wh.width.into(),
                                });
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
                            table::ratio(1, |wh, ctx| {
                                ctx.clip(Path::new().add_rect(wh.to_rect()), ClipOp::Intersect).add(AutoScrollViewWithCtx {
                                    wh,
                                    scroll_bar_width: PADDING,
                                    content: |mut ctx| {
                                        let content_width = wh.width - PADDING * 2.;

                                        for (item_index, item) in game_state.items.iter().enumerate() {
                                            let content = ctx.ghost_compose(format!("InventoryItemContent {item_index}"), |ctx| {
                                                table::vertical([
                                                    table::fixed(
                                                        HEADLINE_FONT_SIZE_LARGE.into_px(),
                                                        table::horizontal([
                                                            table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px(), |_, _| {
                                                                // TODO: Icons
                                                            }),
                                                            table::ratio(1, |_, _| {}),
                                                            table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px() * 3.0, |wh, ctx| {
                                                                ctx.add(TextButton {
                                                                    rect: wh.to_rect(),
                                                                    text: TopBarText::Use.to_korean().to_string(),
                                                                    text_color: palette::ON_SURFACE,
                                                                    stroke_color: palette::OUTLINE,
                                                                    stroke_width: 1.px(),
                                                                    fill_color: palette::SURFACE,
                                                                    mouse_buttons: vec![MouseButton::Left],
                                                                    on_mouse_up_in: |_| match item.kind.usage() {
                                                                        ItemUsage::Instant => {
                                                                            mutate_game_state(move |game_state| {
                                                                                let item = game_state.items.remove(item_index);
                                                                                use_item(game_state, &item, None);
                                                                            });
                                                                        }
                                                                        ItemUsage::CircularArea { .. }
                                                                        | ItemUsage::LinearArea { .. } => {
                                                                            let item = item.clone();
                                                                            mutate_game_state(move |game_state| {
                                                                                game_state.cursor_preview.kind =
                                                                                    PreviewKind::Item { item, item_index };
                                                                            });
                                                                        }
                                                                    },
                                                                });
                                                            }),
                                                            table::fixed(PADDING, |_, _| {}),
                                                            table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px(), |wh, ctx| {
                                                                ctx.add(TextButton {
                                                                    rect: wh.to_rect(),
                                                                    text: TopBarText::Remove.to_korean().to_string(),
                                                                    text_color: palette::ON_SURFACE,
                                                                    stroke_color: palette::OUTLINE,
                                                                    stroke_width: 1.px(),
                                                                    fill_color: palette::SURFACE,
                                                                    mouse_buttons: vec![MouseButton::Left],
                                                                    on_mouse_up_in: |_| {
                                                                        mutate_game_state(move |game_state| {
                                                                            game_state.items.remove(item_index);
                                                                        });
                                                                    },
                                                                });
                                                            }),
                                                        ]),
                                                    ),
                                                    table::fixed(PADDING * 2.0, |_, _| {}),
                                                    table::fit(table::FitAlign::LeftTop, |ctx| {
                                                        ctx.add(Headline {
                                                            text: item.kind.name().to_string(),
                                                            font_size: FontSize::Small,
                                                            text_align: TextAlign::LeftTop,
                                                            max_width: content_width.into(),
                                                        });
                                                    }),
                                                    table::fixed(PADDING, |_, _| {}),
                                                    table::fit(table::FitAlign::LeftTop, |ctx| {
                                                        ctx.add(Paragraph {
                                                            text: item.kind.description(),
                                                            font_size: FontSize::Medium,
                                                            text_align: TextAlign::LeftTop,
                                                            max_width: content_width.into(),
                                                        });
                                                    }),
                                                ])(Wh::new(content_width, f32::MAX.px()), ctx);
                                            });

                                            let Some(content_wh) = bounding_box(&content).map(|rect| rect.wh()) else {
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
                                                    fill: None,
                                                    round: Some(RectRound {
                                                        radius: palette::ROUND,
                                                    }),
                                                },
                                            }));
                                
                                            ctx.add(rect(RectParam {
                                                rect: Wh::new(
                                                    container_wh.width,
                                                    HEADLINE_FONT_SIZE_LARGE.into_px() + PADDING * 2.0,
                                                )
                                                .to_rect(),
                                                style: RectStyle {
                                                    stroke: None,
                                                    fill: Some(RectFill {
                                                        color: palette::SURFACE_CONTAINER,
                                                    }),
                                                    round: Some(RectRound {
                                                        radius: palette::ROUND,
                                                    }),
                                                },
                                            }));
                                
                                            ctx.add(rect(RectParam {
                                                rect: container_wh.to_rect(),
                                                style: RectStyle {
                                                    stroke: None,
                                                    fill: Some(RectFill {
                                                        color: palette::SURFACE,
                                                    }),
                                                    round: Some(RectRound {
                                                        radius: palette::ROUND,
                                                    }),
                                                },
                                            }));

                                            ctx = ctx.translate((0.px(), container_wh.height + PADDING));
                                        }
                                    },
                                });
                            }),
                        ]),
                    ),
                ),
            ])(screen_wh, ctx);
        });
    }
}

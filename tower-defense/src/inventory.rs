use std::iter;

use crate::{
    game_state::{
        item::{use_item, Item},
        use_game_state,
    },
    palette,
    upgrade::MAX_SHOP_SLOT_UPGRADE,
};
use namui::*;
use namui_prebuilt::{table, typography};

const INVENTORY_WIDTH: Px = px(240.);
const PADDING: Px = px(4.);
const ITEM_HEIGHT: Px = px(36.);

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
                    table::padding(PADDING, |wh, ctx| {
                        let height = ITEM_HEIGHT * (game_state.items.len() + 1) as f32;

                        ctx.compose(|ctx| {
                            table::vertical(
                                iter::once(table::fixed(
                                    ITEM_HEIGHT,
                                    table::padding(PADDING, |wh, ctx| {
                                        ctx.add(typography::body::center(
                                            wh,
                                            format!(
                                                "인벤토리 {}/{MAX_SHOP_SLOT_UPGRADE}",
                                                game_state.items.len()
                                            ),
                                            palette::ON_SURFACE,
                                        ));
                                    }),
                                ))
                                .chain(
                                    game_state.items.iter().enumerate().map(
                                        |(item_index, item)| {
                                            table::fixed(
                                                ITEM_HEIGHT,
                                                table::padding(PADDING, move |wh, ctx| {
                                                    ctx.add(InventoryItem {
                                                        wh,
                                                        item,
                                                        item_index,
                                                    });
                                                }),
                                            )
                                        },
                                    ),
                                ),
                            )(Wh::new(wh.width, height), ctx);
                        });

                        ctx.add(rect(RectParam {
                            rect: Rect::zero_wh(Wh::new(wh.width, height)),
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
                    }),
                ),
            ])(screen_wh, ctx);
        });
    }
}

struct InventoryItem<'a> {
    wh: Wh<Px>,
    item: &'a Item,
    item_index: usize,
}
impl Component for InventoryItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            item,
            item_index,
        } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(wh.height, |_, _| {
                    // TODO: Icons
                }),
                table::fixed(PADDING, |_, _| {}),
                table::ratio(1, |wh, ctx| {
                    ctx.add(typography::body::center(
                        wh,
                        item.kind.name(),
                        palette::ON_SURFACE,
                    ));
                }),
            ])(wh, ctx);
        });

        ctx.add(
            rect(RectParam {
                rect: wh.to_rect(),
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: palette::OUTLINE,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: palette::PRIMARY,
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            })
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                use_item(item_index);
            }),
        );
    }
}

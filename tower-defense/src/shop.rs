use crate::{
    game_state::{item::Item, mutate_game_state, use_game_state},
    palette,
};
use namui::*;
use namui_prebuilt::{
    button::{self, TextButton},
    simple_rect,
    table::{self, ratio},
    typography,
};

const PADDING: Px = px(4.0);
const SHOP_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const SHOP_BUTTON_WH: Wh<Px> = Wh {
    width: px(64.0),
    height: px(36.0),
};
const SOLD_OUT_HEIGHT: Px = px(36.0);

#[derive(Default, Clone)]
pub enum ShopSlot {
    #[default]
    Locked,
    Item {
        item: Item,
        cost: u32,
        purchased: bool,
    },
}

pub struct ShopModal {
    pub screen_wh: Wh<Px>,
}
impl Component for ShopModal {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;
        let game_state = use_game_state(ctx);

        let (opened, set_opened) = ctx.state(|| true);

        let toggle_open = || {
            set_opened.mutate(|opened| *opened = !*opened);
        };
        let shop_slots = &game_state.shop_slots;

        let purchase_item = |slot_index: usize| {
            mutate_game_state(move |state| {
                let slot = &mut state.shop_slots[slot_index];
                let ShopSlot::Item {
                    item,
                    cost,
                    purchased,
                } = slot
                else {
                    panic!("Invalid shop slot");
                };

                assert!(state.items.len() <= state.shop_slot);
                assert!(state.money >= *cost);
                assert!(!*purchased);

                state.items.push(item.clone());
                state.money -= *cost;
                *purchased = true;
            });
        };

        let offset = ((screen_wh - SHOP_WH) * 0.5).as_xy();

        ctx.compose(|ctx| {
            ctx.translate(offset).add(ShopOpenButton {
                opened: *opened,
                toggle_open: &toggle_open,
            });
        });

        ctx.compose(|ctx| {
            if !*opened {
                return;
            }
            ctx.translate(offset).add(Shop {
                shop_slots,
                purchase_item: &purchase_item,
            });
        });
    }
}

struct ShopOpenButton<'a> {
    opened: bool,
    toggle_open: &'a dyn Fn(),
}
impl Component for ShopOpenButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            opened,
            toggle_open,
        } = self;

        ctx.compose(|ctx| {
            ctx.translate((0.px(), SHOP_BUTTON_WH.height))
                .add(TextButton {
                    rect: SHOP_BUTTON_WH.to_rect(),
                    text: format!("상점 {}", if opened { "🔼" } else { "🔽" }),
                    text_color: palette::ON_SURFACE,
                    stroke_color: palette::OUTLINE,
                    stroke_width: 1.px(),
                    fill_color: palette::SURFACE_CONTAINER,
                    mouse_buttons: vec![MouseButton::Left],
                    on_mouse_up_in: |_| {
                        toggle_open();
                    },
                });
        });
    }
}

struct Shop<'a> {
    shop_slots: &'a [ShopSlot],
    purchase_item: &'a dyn Fn(usize),
}
impl Component for Shop<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            shop_slots,
            purchase_item,
        } = self;

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::horizontal(shop_slots.iter().enumerate().map(
                    |(shop_slot_index, shop_slot)| {
                        ratio(1, move |wh, ctx| {
                            ctx.add(ShopItem {
                                wh,
                                shop_slot,
                                shop_slot_index,
                                purchase_item,
                            });
                        })
                    },
                )),
            )(SHOP_WH, ctx);
        });
    }
}

struct ShopItem<'a> {
    wh: Wh<Px>,
    shop_slot: &'a ShopSlot,
    shop_slot_index: usize,
    purchase_item: &'a dyn Fn(usize),
}
impl Component for ShopItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            shop_slot,
            shop_slot_index,
            purchase_item,
        } = self;

        let money = use_game_state(ctx).money;
        let purchase_item = || purchase_item(shop_slot_index);

        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                match shop_slot {
                    ShopSlot::Locked => {
                        ctx.add(ShopItemLocked { wh });
                    }
                    ShopSlot::Item {
                        item,
                        cost,
                        purchased,
                    } => {
                        ctx.add(ShopItemContent {
                            wh,
                            item,
                            purchase_item: &purchase_item,
                            cost: *cost,
                            purchased: *purchased,
                            not_enough_money: money < *cost,
                        });
                    }
                }

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
            })(wh, ctx);
        });
    }
}

struct ShopItemLocked {
    wh: Wh<Px>,
}
impl Component for ShopItemLocked {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(SOLD_OUT_HEIGHT, |wh, ctx| {
                    ctx.add(typography::title::center(wh, "🔒", palette::ON_SURFACE));
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}

struct ShopItemContent<'a> {
    wh: Wh<Px>,
    item: &'a Item,
    purchase_item: &'a dyn Fn(),
    cost: u32,
    purchased: bool,
    not_enough_money: bool,
}
impl Component for ShopItemContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            item,
            purchase_item,
            cost,
            purchased,
            not_enough_money,
        } = self;

        let available = !purchased && !not_enough_money;

        ctx.compose(|ctx| {
            if !purchased {
                return;
            }
            ctx.add(ShopItemSoldOut { wh });
        });

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    wh.width,
                    table::padding(PADDING, |_wh, _ctx| {
                        // TODO: Icons
                    }),
                ),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        ctx.compose(|ctx| {
                            table::padding(
                                PADDING,
                                table::vertical([
                                    table::fixed(36.px(), |_wh, ctx| {
                                        ctx.add(typography::body::left_top(
                                            item.name(),
                                            palette::ON_SURFACE,
                                        ));
                                    }),
                                    table::fixed(PADDING, |_, _| {}),
                                    table::ratio(1, |_wh, ctx| {
                                        ctx.add(typography::body::left_top(
                                            item.description(),
                                            palette::ON_SURFACE_VARIANT,
                                        ));
                                    }),
                                    table::fixed(PADDING, |_, _| {}),
                                    table::fixed(48.px(), |wh, ctx| {
                                        ctx.add(button::TextButton {
                                            rect: wh.to_rect(),
                                            text: format!("${}", cost),
                                            text_color: match available {
                                                true => palette::ON_PRIMARY,
                                                false => palette::ON_SURFACE,
                                            },
                                            stroke_color: palette::OUTLINE,
                                            stroke_width: 1.px(),
                                            fill_color: match available {
                                                true => palette::PRIMARY,
                                                false => palette::SURFACE_CONTAINER_HIGH,
                                            },
                                            mouse_buttons: vec![MouseButton::Left],
                                            on_mouse_up_in: |_| {
                                                if !available {
                                                    return;
                                                }
                                                purchase_item();
                                            },
                                        });
                                    }),
                                ]),
                            )(wh, ctx);
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
                                    color: palette::SURFACE,
                                }),
                                round: Some(RectRound {
                                    radius: palette::ROUND,
                                }),
                            },
                        }));
                    }),
                ),
            ])(wh, ctx);
        });
    }
}

struct ShopItemSoldOut {
    wh: Wh<Px>,
}
impl Component for ShopItemSoldOut {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(SOLD_OUT_HEIGHT, |wh, ctx| {
                    ctx.add(typography::title::center(
                        wh,
                        "Sold Out",
                        palette::ON_SECONDARY,
                    ));
                    ctx.add(simple_rect(
                        wh,
                        Color::TRANSPARENT,
                        0.px(),
                        palette::SECONDARY,
                    ));
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}

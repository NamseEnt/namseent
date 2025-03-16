use crate::{
    game_state::{
        item::{Item, generate_items, item_cost},
        mutate_game_state, use_game_state,
    },
    palette,
    theme::typography::{FontSize, Headline, Paragraph, TextAlign},
};
use namui::*;
use namui_prebuilt::{
    button::{self, TextButton},
    simple_rect,
    table::{self, ratio},
};

const PADDING: Px = px(4.0);
const SHOP_WH: Wh<Px> = Wh {
    width: px(960.0),
    height: px(480.0),
};
const SHOP_BUTTON_WH: Wh<Px> = Wh {
    width: px(64.0),
    height: px(36.0),
};
const SHOP_REFRESH_BUTTON_WH: Wh<Px> = Wh {
    width: px(192.0),
    height: px(36.0),
};
const SOLD_OUT_HEIGHT: Px = px(36.0);

#[derive(Default, Clone)]
pub enum ShopSlot {
    #[default]
    Locked,
    Item {
        item: Item,
        cost: usize,
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
                assert!(state.items.len() <= state.max_shop_slot());

                let slot = &mut state.shop_slots[slot_index];
                let ShopSlot::Item {
                    item,
                    cost,
                    purchased,
                } = slot
                else {
                    panic!("Invalid shop slot");
                };

                assert!(state.gold >= *cost);
                assert!(!*purchased);

                state.items.push(item.clone());
                state.gold -= *cost;
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
            ctx.translate((0.px(), -SHOP_BUTTON_WH.height))
                .add(TextButton {
                    rect: SHOP_BUTTON_WH.to_rect(),
                    text: format!("상점 {}", if opened { "^" } else { "v" }),
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

        let game_state = use_game_state(ctx);
        let disabled = game_state.left_shop_refresh_chance == 0;

        let refresh_shop = || {
            mutate_game_state(|game_state| {
                game_state.left_shop_refresh_chance -= 1;
                let items = generate_items(&game_state, game_state.max_shop_slot());
                for (slot, item) in game_state.shop_slots.iter_mut().zip(items.into_iter()) {
                    if let ShopSlot::Item {
                        item: item_of_slot,
                        cost: cost_of_slot,
                        purchased,
                    } = slot
                    {
                        if *purchased {
                            continue;
                        }
                        let cost =
                            item_cost(&item.rarity, game_state.upgrade_state.shop_item_price_minus);
                        *cost_of_slot = cost;
                        *item_of_slot = item.clone();
                    }
                }
            });
        };

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::ratio(
                        1,
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
                    ),
                    table::fixed(
                        SHOP_REFRESH_BUTTON_WH.height,
                        table::horizontal([
                            ratio(1, |_, _| {}),
                            table::fixed(SHOP_REFRESH_BUTTON_WH.width, |wh, ctx| {
                                ctx.add(TextButton {
                                    rect: wh.to_rect(),
                                    text: format!(
                                        "새로고침-{}",
                                        game_state.left_shop_refresh_chance
                                    ),
                                    text_color: match disabled {
                                        true => palette::ON_SURFACE_VARIANT,
                                        false => palette::ON_SURFACE,
                                    },
                                    stroke_color: palette::OUTLINE,
                                    stroke_width: 1.px(),
                                    fill_color: palette::SURFACE_CONTAINER,
                                    mouse_buttons: vec![MouseButton::Left],
                                    on_mouse_up_in: |_| {
                                        if disabled {
                                            return;
                                        }
                                        refresh_shop();
                                    },
                                });
                            }),
                            ratio(1, |_, _| {}),
                        ]),
                    ),
                ]),
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

        let money = use_game_state(ctx).gold;
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
                    ctx.add(Headline {
                        text: "Locked".to_string(),
                        font_size: FontSize::Medium,
                        text_align: TextAlign::Center { wh },
                        max_width: None,
                    });
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
    cost: usize,
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
                            table::padding(PADDING, |wh, ctx| {
                                table::vertical([
                                    table::fit(table::FitAlign::LeftTop, |ctx| {
                                        ctx.add(Headline {
                                            text: item.kind.name().to_string(),
                                            font_size: FontSize::Small,
                                            text_align: TextAlign::LeftTop,
                                            max_width: Some(wh.width),
                                        });
                                    }),
                                    table::fixed(PADDING, |_, _| {}),
                                    table::ratio(1, |wh, ctx| {
                                        ctx.add(Paragraph {
                                            text: item.kind.description(),
                                            font_size: FontSize::Medium,
                                            text_align: TextAlign::LeftTop,
                                            max_width: Some(wh.width),
                                        });
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
                                ])(wh, ctx);
                            })(wh, ctx);
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
                    ctx.add(Headline {
                        text: "Sold Out".to_string(),
                        font_size: FontSize::Medium,
                        text_align: TextAlign::Center { wh },
                        max_width: None,
                    });
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

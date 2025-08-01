use crate::game_state::MAX_INVENTORY_SLOT;
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::{
    game_state::{
        item::{Item, generate_items, item_cost},
        mutate_game_state, use_game_state,
    },
    icon::{Icon, IconKind, IconSize},
    l10n::ui::TopBarText,
    palette,
    theme::typography::{FontSize, TextAlign, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{
    simple_rect,
    table::{self, ratio},
};

const PADDING: Px = px(4.0);
const SHOP_WH: Wh<Px> = Wh {
    width: px(960.0),
    height: px(480.0),
};
const SHOP_BUTTON_WH: Wh<Px> = Wh {
    width: px(128.0),
    height: px(48.0),
};
const SHOP_REFRESH_BUTTON_WH: Wh<Px> = Wh {
    width: px(192.0),
    height: px(48.0),
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
            mutate_game_state(move |game_state| {
                assert!(game_state.items.len() <= MAX_INVENTORY_SLOT);

                let (item_to_purchase, purchase_cost) = {
                    let slot = &mut game_state.shop_slots[slot_index];
                    let ShopSlot::Item {
                        item,
                        cost,
                        purchased,
                    } = slot
                    else {
                        panic!("Invalid shop slot");
                    };

                    assert!(game_state.gold >= *cost);
                    assert!(!*purchased);

                    let item_to_purchase = item.clone();
                    let purchase_cost = *cost;
                    *purchased = true;
                    (item_to_purchase, purchase_cost)
                };

                game_state.purchase_item(item_to_purchase, purchase_cost);
            });
        };

        let offset = ((screen_wh - SHOP_WH) * 0.5).to_xy();

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
            ctx.translate((0.px(), -SHOP_BUTTON_WH.height)).add(
                Button::new(
                    SHOP_BUTTON_WH,
                    &|| {
                        toggle_open();
                    },
                    &|wh, _text_color, ctx| {
                        ctx.add(Icon::new(IconKind::Shop).size(IconSize::Large).wh(wh));
                    },
                )
                .variant(ButtonVariant::Fab)
                .color(match opened {
                    true => ButtonColor::Primary,
                    false => ButtonColor::Secondary,
                }),
            );
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
                let items = generate_items(game_state, game_state.max_shop_slot());
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
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &|| {
                                            refresh_shop();
                                        },
                                        &|wh, color, ctx| {
                                            ctx.add(
                                                headline(format!(
                                                    "{}-{}",
                                                    Icon::new(IconKind::Refresh)
                                                        .size(IconSize::Large)
                                                        .wh(Wh::single(wh.height))
                                                        .as_tag(),
                                                    game_state.left_shop_refresh_chance
                                                ))
                                                .color(color)
                                                .align(TextAlign::Center { wh })
                                                .build_rich(),
                                            );
                                        },
                                    )
                                    .variant(ButtonVariant::Fab)
                                    .disabled(disabled),
                                );
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
                table::fixed(36.px(), |wh, ctx| {
                    ctx.add(Icon::new(IconKind::Lock).size(IconSize::Large).wh(wh));
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
        let game_state = use_game_state(ctx);
        let available = !purchased && !not_enough_money;
        let name = item.kind.name(&game_state.text());
        let desc = item.kind.description(&game_state.text());
        ctx.compose(|ctx| {
            if purchased {
                ctx.add(ShopItemSoldOut { wh });
            } else {
                table::vertical([
                    table::fixed(
                        wh.width,
                        table::padding(PADDING, |_wh, _ctx| {
                            // TODO: Icons
                        }),
                    ),
                    table::ratio(
                        1,
                        table::padding(
                            PADDING,
                            table::vertical([
                                table::fixed(PADDING, |_, _| {}),
                                table::fit(table::FitAlign::LeftTop, move |ctx| {
                                    ctx.add(
                                        headline(name)
                                            .size(FontSize::Small)
                                            .align(TextAlign::LeftTop)
                                            .max_width(wh.width)
                                            .build(),
                                    );
                                }),
                                table::fixed(PADDING, |_, _| {}),
                                table::ratio(1, move |wh, ctx| {
                                    ctx.add(
                                        paragraph(desc.clone())
                                            .size(FontSize::Medium)
                                            .align(TextAlign::LeftTop)
                                            .max_width(wh.width)
                                            .build_rich(),
                                    );
                                }),
                                table::fixed(PADDING, |_, _| {}),
                                table::fixed(48.px(), |wh, ctx| {
                                    ctx.add(
                                        Button::new(
                                            wh,
                                            &|| {
                                                if !available {
                                                    return;
                                                }
                                                purchase_item();
                                            },
                                            &|wh, color, ctx| {
                                                ctx.add(
                                                    headline(format!(
                                                        "{} {cost}",
                                                        Icon::new(IconKind::Gold)
                                                            .size(IconSize::Large)
                                                            .wh(Wh::single(wh.height))
                                                            .as_tag(),
                                                    ))
                                                    .color(color)
                                                    .build_rich(),
                                                );
                                            },
                                        )
                                        .color(if available {
                                            crate::theme::button::ButtonColor::Primary
                                        } else {
                                            crate::theme::button::ButtonColor::Secondary
                                        })
                                        .disabled(!available),
                                    );
                                }),
                            ]),
                        ),
                    ),
                ])(wh, ctx);
            }
        });
    }
}

struct ShopItemSoldOut {
    wh: Wh<Px>,
}
impl Component for ShopItemSoldOut {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = crate::game_state::use_game_state(ctx);
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(SOLD_OUT_HEIGHT, |wh, ctx| {
                    ctx.add(
                        headline(game_state.text().ui(TopBarText::SoldOut).to_string())
                            .size(FontSize::Medium)
                            .align(TextAlign::Center { wh })
                            .build(),
                    );
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

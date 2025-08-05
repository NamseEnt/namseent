use super::constants::{PADDING, SHOP_REFRESH_BUTTON_WH, SHOP_WH};
use super::items::ShopItem;
use super::slot::ShopSlot;
use crate::game_state::{
    item::{generate_items, item_cost},
    mutate_game_state, use_game_state,
};
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline};
use namui::*;
use namui_prebuilt::table::{self, ratio};

pub struct ShopLayout<'a> {
    pub shop_slots: &'a [ShopSlot],
    pub purchase_item: &'a dyn Fn(usize),
}

impl Component for ShopLayout<'_> {
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

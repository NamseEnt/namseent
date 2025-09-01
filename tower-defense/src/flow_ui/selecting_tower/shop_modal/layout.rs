use super::constants::{PADDING, SHOP_REFRESH_BUTTON_WH, SHOP_WH};
use super::items::ShopItem;
use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::shop::Shop;
use crate::shop::refresh_shop;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline};
use namui::*;
use namui_prebuilt::table::{self, ratio, ratio_no_clip};

pub struct ShopLayout<'a> {
    pub shop: &'a Shop,
    pub purchase_item: &'a dyn Fn(usize),
}

impl Component for ShopLayout<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            shop,
            purchase_item,
        } = self;

        let game_state = use_game_state(ctx);
        let disabled = game_state.left_shop_refresh_chance == 0;

        let refresh_shop = || {
            mutate_game_state(|game_state| {
                game_state.left_shop_refresh_chance -= 1;
                refresh_shop(game_state);
            });
        };

        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::vertical([
                    table::ratio_no_clip(
                        1,
                        table::horizontal(shop.slots.iter().enumerate().map(
                            |(shop_slot_index, shop_slot)| {
                                ratio_no_clip(1, move |wh, ctx| {
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

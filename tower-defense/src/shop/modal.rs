use super::constants::SHOP_WH;
use super::layout::ShopLayout;
use super::open_button::ShopOpenButton;
use super::slot::ShopSlot;
use crate::game_state::{MAX_INVENTORY_SLOT, mutate_game_state, use_game_state};
use namui::*;

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
            ctx.translate(offset).add(ShopLayout {
                shop_slots,
                purchase_item: &purchase_item,
            });
        });
    }
}

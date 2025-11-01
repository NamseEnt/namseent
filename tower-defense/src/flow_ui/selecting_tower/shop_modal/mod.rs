mod constants;
mod items;
mod layout;
mod open_button;

use crate::game_state::{mutate_game_state, use_game_state};
use crate::hand::xy_with_spring;
use crate::shop::{Shop, ShopSlotId};
use constants::{SHOP_BUTTON_WH, SHOP_WH};
use layout::ShopLayout;
use namui::*;
use open_button::ShopOpenButton;

pub struct ShopModal<'a> {
    pub shop: &'a Shop,
}

impl Component for ShopModal<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { shop } = self;
        let screen_wh = screen::size().into_type::<Px>();

        let (opened, set_opened) = ctx.state(|| true);

        let toggle_open = || {
            set_opened.mutate(|opened| *opened = !*opened);
        };

        let purchase_item = |slot_id: ShopSlotId| {
            mutate_game_state(move |game_state| {
                game_state.purchase_shop_item(slot_id);
            });
        };

        let game_state = use_game_state(ctx);
        let can_purchase_item = |slot_id: ShopSlotId| game_state.can_purchase_shop_item(slot_id);

        let center_offset = ((screen_wh - SHOP_WH) * 0.5).to_xy();

        // 버튼을 상점 아이템 바로 위(상점 패널 상단 중앙)에 배치
        let button_xy = center_offset
            + Xy::new(
                (SHOP_WH.width - SHOP_BUTTON_WH.width) * 0.5,
                SHOP_BUTTON_WH.height * -2.0,
            );

        ctx.translate(button_xy).add(ShopOpenButton {
            opened: *opened,
            toggle_open: &toggle_open,
        });
        let target_scale = if *opened {
            Xy::single(1.0)
        } else {
            Xy::single(0.0)
        };

        let target_xy = if *opened {
            center_offset
        } else {
            // 닫혔을 때는 버튼의 중앙 위치로
            button_xy + (SHOP_BUTTON_WH.to_xy() * 0.5) - (SHOP_WH.to_xy() * 0.5)
        };

        let animated_scale = {
            let animated_scale = xy_with_spring(ctx, target_scale, Xy::single(0.0));
            Xy::new(animated_scale.x.max(0.0001), animated_scale.y.max(0.0001))
        };
        let animated_xy = xy_with_spring(ctx, target_xy, button_xy);

        let pivot = SHOP_WH.to_xy() * 0.5;

        ctx.translate(animated_xy)
            .translate(pivot)
            .scale(animated_scale)
            .translate(-pivot)
            .add(ShopLayout {
                shop,
                purchase_item: &purchase_item,
                can_purchase_item: &can_purchase_item,
                button_xy,
            });
    }
}

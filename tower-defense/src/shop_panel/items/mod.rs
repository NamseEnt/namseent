use super::constants::PADDING;
use crate::game_state::use_game_state;
use crate::palette;
use crate::shop::{ShopSlot, ShopSlotData};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use namui::*;
use namui_prebuilt::table;

mod backgrounds;
mod description;
mod layout;
mod title;

use layout::ShopItemLayout;

pub struct ShopItem<'a> {
    pub wh: Wh<Px>,
    pub slot_data: &'a ShopSlotData,
    pub can_purchase_item: bool,
}

impl Component for ShopItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            slot_data,
            can_purchase_item,
        } = self;

        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::padding_no_clip(PADDING, |wh, ctx| {
                match &slot_data.slot {
                    ShopSlot::Item { item, cost } => {
                        let params = layout::layout_params_for_slot(
                            wh,
                            layout::ShopSlotVariant::Item { item, cost: *cost },
                            slot_data.purchased,
                            !can_purchase_item,
                            &game_state,
                        );
                        ctx.add(ShopItemLayout { params });
                    }
                    ShopSlot::Upgrade { upgrade, cost } => {
                        let params = layout::layout_params_for_slot(
                            wh,
                            layout::ShopSlotVariant::Upgrade {
                                upgrade,
                                cost: *cost,
                            },
                            slot_data.purchased,
                            !can_purchase_item,
                            &game_state,
                        );
                        ctx.add(ShopItemLayout { params });
                    }
                }

                ctx.add(PaperContainerBackground {
                    width: wh.width,
                    height: wh.height,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Card,
                    color: palette::SURFACE_CONTAINER_HIGH,
                    shadow: true,
                    arrow: None,
                });
            })(wh, ctx);
        });
    }
}

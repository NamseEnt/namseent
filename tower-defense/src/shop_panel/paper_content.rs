use crate::game_state::{flow::GameFlow, mutate_game_state, use_game_state};
use crate::shop::ShopSlotId;
use crate::shop_panel::constants::*;
use crate::shop_panel::slot_layout_calculator::SlotLayoutCalculator;
use crate::shop_panel::slot_renderer::ShopSlotView;
use crate::shop_panel::slot_rendering_data::SlotRenderingData;
use namui::*;
use namui_prebuilt::table;

pub(super) struct ShopPaperContent {
    pub wh: Wh<Px>,
}

impl Component for ShopPaperContent {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        let shop_context = match &game_state.flow {
            GameFlow::SelectingTower(flow) => Some(&flow.shop),
            GameFlow::SelectingTreasure(flow) => Some(&flow.shop),
            _ => None,
        };

        if let Some(shop) = shop_context {
            let purchase_item = |slot_id: ShopSlotId| {
                mutate_game_state(move |game_state| {
                    game_state.purchase_shop_item(slot_id);
                });
            };
            let can_purchase_item =
                |slot_id: ShopSlotId| game_state.can_purchase_shop_item(slot_id);

            let (exiting_slot_positions, set_exiting_slot_positions) =
                ctx.state::<std::collections::HashMap<ShopSlotId, Xy<Px>>>(Default::default);
            let (hovered_slot_id, set_hovered_slot_id) = ctx.state::<Option<ShopSlotId>>(|| None);

            ctx.compose(|ctx| {
                table::padding_no_clip(
                    PANEL_PADDING,
                    table::vertical([table::ratio_no_clip(1, |wh, ctx| {
                        let content_wh = Wh {
                            width: wh.width - PADDING * 2.0,
                            height: wh.height - PADDING * 2.0,
                        };
                        let items_area_wh = Wh {
                            width: content_wh.width,
                            height: content_wh.height,
                        };

                        let calculator = SlotLayoutCalculator::new(items_area_wh);
                        let (slot_positions, slot_wh) = calculator.calculate_positions(shop);

                        let rendering_data =
                            SlotRenderingData::from_shop(shop, slot_positions.clone());

                        set_exiting_slot_positions.mutate(
                            move |positions: &mut std::collections::HashMap<ShopSlotId, Xy<Px>>| {
                                *positions = slot_positions.clone();
                            },
                        );

                        ctx.compose(|ctx| {
                            if let Some(hovered_id) = *hovered_slot_id
                                && let Some(slot_data) = rendering_data
                                    .active_slots
                                    .iter()
                                    .find(|s| s.id == hovered_id)
                                && let Some(target_xy) = rendering_data.get_position(hovered_id)
                            {
                                ctx.translate((PADDING, PADDING)).add_with_key(
                                    hovered_id,
                                    ShopSlotView {
                                        wh: slot_wh,
                                        slot_data,
                                        purchase_item: &purchase_item,
                                        can_purchase_item: can_purchase_item(hovered_id),
                                        target_xy,
                                        hovered_slot_id: *hovered_slot_id,
                                        set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                                    },
                                );
                            }

                            for slot_data in &rendering_data.active_slots {
                                let slot_id = slot_data.id;
                                if *hovered_slot_id == Some(slot_id) {
                                    continue;
                                }
                                if let Some(target_xy) = rendering_data.get_position(slot_id) {
                                    ctx.translate((PADDING, PADDING)).add_with_key(
                                        slot_id,
                                        ShopSlotView {
                                            wh: slot_wh,
                                            slot_data,
                                            purchase_item: &purchase_item,
                                            can_purchase_item: can_purchase_item(slot_id),
                                            target_xy,
                                            hovered_slot_id: *hovered_slot_id,
                                            set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                                        },
                                    );
                                }
                            }

                            for slot_data in &rendering_data.exiting_slots {
                                let slot_id = slot_data.id;
                                let target_xy = exiting_slot_positions
                                    .get(&slot_id)
                                    .copied()
                                    .unwrap_or(Xy::zero());
                                ctx.translate((PADDING, PADDING)).add_with_key(
                                    slot_id,
                                    ShopSlotView {
                                        wh: slot_wh,
                                        slot_data,
                                        purchase_item: &purchase_item,
                                        can_purchase_item: can_purchase_item(slot_id),
                                        target_xy,
                                        hovered_slot_id: *hovered_slot_id,
                                        set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                                    },
                                );
                            }
                        });
                    })]),
                )(wh, ctx);
            });
        }
    }
}

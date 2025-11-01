use super::constants::{PADDING, SHOP_REFRESH_BUTTON_WH, SHOP_WH};
use super::refresh_button::RefreshButton;
use super::slot_layout_calculator::SlotLayoutCalculator;
use super::slot_renderer::ShopSlotView;
use super::slot_rendering_data::SlotRenderingData;
use crate::shop::{Shop, ShopSlotId};
use namui::*;
use std::collections::HashMap;

pub struct ShopLayout<'a> {
    pub shop: &'a Shop,
    pub purchase_item: &'a dyn Fn(ShopSlotId),
    pub can_purchase_item: &'a dyn Fn(ShopSlotId) -> bool,
}

impl Component for ShopLayout<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            shop,
            purchase_item,
            can_purchase_item,
        } = self;

        // 호버 상태 관리
        let (hovered_slot_id, set_hovered_slot_id) = ctx.state::<Option<ShopSlotId>>(|| None);

        // exit 애니메이션 중인 슬롯들의 마지막 위치를 저장
        let (exiting_slot_positions, set_exiting_slot_positions) =
            ctx.state::<HashMap<ShopSlotId, Xy<Px>>>(Default::default);

        ctx.compose(|ctx| {
            // 레이아웃 영역 계산
            let content_wh = Wh {
                width: SHOP_WH.width - PADDING * 2.0,
                height: SHOP_WH.height - PADDING * 2.0,
            };
            let items_area_wh = Wh {
                width: content_wh.width,
                height: content_wh.height - SHOP_REFRESH_BUTTON_WH.height,
            };

            // 슬롯 레이아웃 계산 (exit 애니메이션 중인 슬롯 제외)
            let calculator = SlotLayoutCalculator::new(items_area_wh);
            let (slot_positions, slot_wh) = calculator.calculate_positions(shop);

            // 슬롯 렌더링 데이터 준비
            let rendering_data = SlotRenderingData::from_shop(shop, slot_positions);

            // exiting_slot_positions 업데이트
            let slot_positions_clone = rendering_data.slot_positions.clone();
            set_exiting_slot_positions.mutate(move |positions| {
                for (slot_id, xy) in &slot_positions_clone {
                    positions.insert(*slot_id, *xy);
                }
            });

            // 호버된 슬롯이 있으면 먼저 렌더링 (위로 올라오도록)
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
                        purchase_item,
                        can_purchase_item: can_purchase_item(hovered_id),
                        target_xy,
                        hovered_slot_id: *hovered_slot_id,
                        set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                    },
                );
            }

            // 활성 슬롯 렌더링 (호버된 것 제외)
            for slot_data in &rendering_data.active_slots {
                let slot_id = slot_data.id;
                if *hovered_slot_id == Some(slot_id) {
                    continue;
                }

                let target_xy = rendering_data.get_position(slot_id).unwrap();

                ctx.translate((PADDING, PADDING)).add_with_key(
                    slot_id,
                    ShopSlotView {
                        wh: slot_wh,
                        slot_data,
                        purchase_item,
                        can_purchase_item: can_purchase_item(slot_id),
                        target_xy,
                        hovered_slot_id: *hovered_slot_id,
                        set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                    },
                );
            }

            // exit 애니메이션 중인 슬롯 렌더링
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
                        purchase_item,
                        can_purchase_item: can_purchase_item(slot_id),
                        target_xy,
                        hovered_slot_id: *hovered_slot_id,
                        set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                    },
                );
            }

            // 리롤 버튼 렌더링
            let btn_xy = Xy::new(
                (content_wh.width - SHOP_REFRESH_BUTTON_WH.width) * 0.5,
                items_area_wh.height,
            );
            ctx.translate((PADDING, PADDING))
                .translate(btn_xy)
                .add(RefreshButton::new(SHOP_REFRESH_BUTTON_WH));
        });
    }
}

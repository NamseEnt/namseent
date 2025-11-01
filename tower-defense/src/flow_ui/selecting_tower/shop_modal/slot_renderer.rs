use super::items::ShopItem;
use crate::hand::xy_with_spring;
use crate::shop::ShopSlotId;
use namui::*;
use namui_prebuilt::simple_rect;

/// 슬롯 단위 애니메이션 및 실제 아이템 렌더링을 담당하는 뷰
pub struct ShopSlotView<'a> {
    pub wh: Wh<Px>,
    pub slot_data: &'a crate::shop::ShopSlotData,
    pub purchase_item: &'a dyn Fn(ShopSlotId),
    pub can_purchase_item: bool,
    pub target_xy: Xy<Px>,
    pub hovered_slot_id: Option<ShopSlotId>,
    pub set_hovered_slot_id: &'a dyn Fn(Option<ShopSlotId>),
}

impl Component for ShopSlotView<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            slot_data,
            purchase_item,
            can_purchase_item,
            target_xy,
            hovered_slot_id,
            set_hovered_slot_id,
        } = self;

        let slot_id = slot_data.id;

        // Exit 애니메이션이 있는 경우 처리 (제자리에서 scale만 0으로)
        let (target_xy, target_scale) = if slot_data.exit_animation.is_some() {
            // 위치는 그대로, scale만 0으로
            (target_xy, Xy::single(0.0))
        } else {
            // 이 슬롯이 현재 호버된 슬롯인지 확인
            let hovering = hovered_slot_id == Some(slot_id);

            // 호버 시 1.2배 스케일
            let scale = if hovering {
                Xy::single(1.2)
            } else {
                Xy::single(1.0)
            };
            (target_xy, scale)
        };

        // 아래에서 위로 스르륵 올라오는 기본 진입 애니메이션
        let initial_xy = Xy::new(target_xy.x, target_xy.y + px(64.0));
        let animated_xy = xy_with_spring(ctx, target_xy, initial_xy);

        let animated_scale = xy_with_spring(ctx, target_scale, Xy::single(0.0));

        let half_xy = wh.to_xy() * 0.5;
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_xy)
            .scale(animated_scale)
            .translate(-half_xy);

        // Exit 애니메이션 중인지 확인
        let is_exiting = slot_data.exit_animation.is_some();
        let hovering = hovered_slot_id == Some(slot_id);

        // 실제 콘텐츠 렌더링
        ctx.compose(|ctx| {
            ctx.add(ShopItem {
                wh,
                slot_data,
                purchase_item,
                can_purchase_item,
            });

            // Exit 애니메이션 중이 아닐 때만 hover 감지
            if !is_exiting {
                ctx.add(
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                        move |event| {
                            let Event::MouseMove { event } = event else {
                                return;
                            };
                            if event.is_local_xy_in() {
                                set_hovered_slot_id(Some(slot_id));
                            } else if hovering {
                                // 현재 호버된 슬롯에서 마우스가 벗어났을 때만 None으로 설정
                                set_hovered_slot_id(None);
                            }
                        },
                    ),
                );
            }
        });
    }
}

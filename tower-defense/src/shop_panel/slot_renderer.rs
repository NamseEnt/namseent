use super::items::ShopItem;
use crate::game_state::shop_purchase::ShopPurchaseStatus;
use crate::hand::xy_with_spring;
use crate::shop::{ShopSlot, ShopSlotId};
use crate::tooltip::WithHoverArea;
use namui::*;
use namui_prebuilt::simple_rect;

pub struct ShopSlotView<'a> {
    pub wh: Wh<Px>,
    pub slot_data: &'a crate::shop::ShopSlotData,
    pub purchase_item: &'a dyn Fn(ShopSlotId),
    pub purchase_status: ShopPurchaseStatus,
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
            purchase_status,
            target_xy,
            hovered_slot_id,
            set_hovered_slot_id,
        } = self;

        let slot_id = slot_data.id;

        let hovering = hovered_slot_id == Some(slot_id);
        let ctx: ComposeCtx<'_, '_> = apply_slot_transform(ctx, wh, slot_data, target_xy, hovering);

        let cursor = if purchase_status.is_available() {
            MouseCursor::Standard(StandardCursor::Pointer)
        } else {
            MouseCursor::Standard(StandardCursor::NotAllowed)
        };

        let is_exiting = slot_data.exit_animation.is_some();

        ctx.mouse_cursor(cursor).compose(|ctx| {
            ctx.add(ShopItem {
                wh,
                slot_data,
                purchase_status: purchase_status.clone(),
            });

            if !is_exiting {
                ctx.add(WithHoverArea {
                    component_key: "item tooltip",
                    component: simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT),
                    placement: crate::tooltip::TooltipPlacement::Above,
                    on_enter: || {
                        set_hovered_slot_id(Some(slot_id));
                        match &slot_data.slot {
                            ShopSlot::Item { item, .. } => {
                                Some(crate::tooltip::TooltipContent::shop(
                                    crate::tooltip::TooltipContent::Item(item.clone()),
                                    slot_id,
                                ))
                            }
                            ShopSlot::Upgrade { upgrade, .. } => {
                                Some(crate::tooltip::TooltipContent::shop(
                                    crate::tooltip::TooltipContent::Upgrade(*upgrade),
                                    slot_id,
                                ))
                            }
                            ShopSlot::CardService { card_service, .. } => {
                                Some(crate::tooltip::TooltipContent::shop(
                                    crate::tooltip::TooltipContent::CardService(
                                        card_service.clone(),
                                    ),
                                    slot_id,
                                ))
                            }
                        }
                    },
                    on_exit: || {
                        if hovered_slot_id == Some(slot_id) {
                            set_hovered_slot_id(None);
                        }
                    },
                })
                .attach_event(|event| {
                    let Event::MouseDown { event } = event else {
                        return;
                    };

                    if !purchase_status.is_available()
                        || !event.is_local_xy_in()
                        || !matches!(event.button, Some(MouseButton::Left))
                    {
                        return;
                    }

                    event.stop_propagation();
                    purchase_item(slot_id);
                });
            }
        });
    }
}

fn apply_slot_transform<'a>(
    ctx: &'a RenderCtx<'a, 'a>,
    wh: Wh<Px>,
    slot_data: &'a crate::shop::ShopSlotData,
    target_xy: Xy<Px>,
    hovering: bool,
) -> ComposeCtx<'a, 'a> {
    let (target_xy, target_scale) = if slot_data.exit_animation.is_some() {
        (target_xy, Xy::single(0.0))
    } else {
        let scale = if hovering {
            Xy::single(1.12)
        } else {
            Xy::single(1.0)
        };
        (target_xy, scale)
    };

    let initial_xy = Xy::new(target_xy.x, target_xy.y + px(64.0));
    let animated_xy = xy_with_spring(ctx, target_xy, initial_xy);

    let animated_scale = {
        let scale = xy_with_spring(ctx, target_scale, Xy::single(0.0));
        Xy::new(scale.x.max(0.0001), scale.y.max(0.0001))
    };

    let half_xy = wh.to_xy() * 0.5;
    ctx.translate(animated_xy)
        .translate(half_xy)
        .scale(animated_scale)
        .translate(-half_xy)
}

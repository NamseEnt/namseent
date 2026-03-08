use super::items::ShopItem;
use crate::hand::xy_with_spring;
use crate::shop::ShopSlotId;
use namui::*;
use namui_prebuilt::simple_rect;

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

        let (target_xy, target_scale) = if slot_data.exit_animation.is_some() {
            (target_xy, Xy::single(0.0))
        } else {
            let hovering = hovered_slot_id == Some(slot_id);

            let scale = if hovering {
                Xy::single(1.2)
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
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_xy)
            .scale(animated_scale)
            .translate(-half_xy);

        let is_exiting = slot_data.exit_animation.is_some();
        let hovering = hovered_slot_id == Some(slot_id);

        ctx.compose(|ctx| {
            ctx.add(ShopItem {
                wh,
                slot_data,
                purchase_item,
                can_purchase_item,
            });

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
                                set_hovered_slot_id(None);
                            }
                        },
                    ),
                );
            }
        });
    }
}

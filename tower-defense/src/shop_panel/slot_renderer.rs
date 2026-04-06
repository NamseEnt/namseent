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

        let hovering = hovered_slot_id == Some(slot_id);
        let ctx = apply_slot_transform(ctx, wh, slot_data, target_xy, hovering);

        let cursor = if can_purchase_item {
            MouseCursor::Standard(StandardCursor::Pointer)
        } else {
            MouseCursor::Standard(StandardCursor::NotAllowed)
        };

        let is_exiting = slot_data.exit_animation.is_some();

        ctx.mouse_cursor(cursor).compose(|ctx| {
            ctx.add(ShopItem {
                wh,
                slot_data,
                can_purchase_item,
            });

            if !is_exiting {
                ctx.add(
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                        move |event| match event {
                            Event::MouseMove { event } => {
                                if event.is_local_xy_in() {
                                    set_hovered_slot_id(Some(slot_id));
                                } else if hovering {
                                    set_hovered_slot_id(None);
                                }
                            }
                            Event::MouseDown { event } => {
                                if !can_purchase_item || !event.is_local_xy_in() {
                                    return;
                                }
                                event.stop_propagation();
                                purchase_item(slot_id);
                            }
                            _ => {}
                        },
                    ),
                );
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
    ctx.translate(animated_xy)
        .translate(half_xy)
        .scale(animated_scale)
        .translate(-half_xy)
}

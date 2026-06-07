use super::items::ShopItem;
use crate::animation::with_spring;
use crate::hand::xy_with_spring;
use crate::l10n;
use crate::palette;
use crate::shop::{ShopSlot, ShopSlotId};
use crate::theme::paper_container::{
    ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
};
use crate::theme::typography::{FontSize, memoized_text};
use namui::*;
use namui_prebuilt::{simple_rect, table};

mod tooltip {
    use namui::*;
    pub const PADDING: Px = px(12.0);
    pub const MAX_WIDTH: Px = px(320.0);
    pub const ARROW_WIDTH: Px = px(12.0);
    pub const ARROW_HEIGHT: Px = px(10.0);
    pub const OFFSET_Y: Px = px(8.0);
}

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
        let tooltip_scale = with_spring(
            ctx,
            if hovering { 1.0 } else { 0.0 },
            0.0,
            |v| v * v,
            || 0.0,
        );
        let ctx: ComposeCtx<'_, '_> = apply_slot_transform(ctx, wh, slot_data, target_xy, hovering);

        let cursor = if can_purchase_item {
            MouseCursor::Standard(StandardCursor::Pointer)
        } else {
            MouseCursor::Standard(StandardCursor::NotAllowed)
        };

        let is_exiting = slot_data.exit_animation.is_some();

        ctx.mouse_cursor(cursor).compose(|ctx| {
            if tooltip_scale > 0.01 && !is_exiting {
                let tooltip = ctx.ghost_add("shop-slot-tooltip", ShopSlotTooltip { slot_data });
                if let Some(tooltip_wh) = tooltip.bounding_box().map(|rect| rect.wh()) {
                    let base = Xy::new(
                        (wh.width - tooltip_wh.width) / 2.0,
                        -tooltip_wh.height - tooltip::ARROW_HEIGHT - tooltip::OFFSET_Y,
                    );
                    let pivot = Xy::new(tooltip_wh.width / 2.0, tooltip_wh.height);
                    ctx.translate(base + pivot)
                        .scale(Xy::new(tooltip_scale, tooltip_scale))
                        .translate(-pivot)
                        .on_top()
                        .add(tooltip);
                }
            }

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
                                    event.stop_propagation();
                                } else if hovering {
                                    set_hovered_slot_id(None);
                                }
                            }
                            Event::MouseDown { event } => {
                                if !can_purchase_item
                                    || !event.is_local_xy_in()
                                    || !matches!(event.button, Some(MouseButton::Left))
                                {
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

struct ShopSlotTooltip<'a> {
    slot_data: &'a crate::shop::ShopSlotData,
}

impl Component for ShopSlotTooltip<'_> {
    fn render(self, ctx: &RenderCtx) {
        let locale = crate::game_state::use_game_state(ctx).text().locale();
        let max_width = tooltip::MAX_WIDTH;
        let text_max = max_width - tooltip::PADDING * 2.0;

        let slot_data = self.slot_data;
        let content = ctx.ghost_compose("shop-slot-tooltip-content", |ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    render_name_text(ctx, slot_data, locale, text_max);
                }),
                table::fixed_no_clip(tooltip::PADDING, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    render_description_text(ctx, slot_data, locale, text_max);
                }),
            ])(Wh::new(text_max, f32::MAX.px()), ctx);
        });

        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };

        let container_wh = content_wh + Wh::single(tooltip::PADDING * 2.0);
        ctx.translate((tooltip::PADDING, tooltip::PADDING))
            .add(content);
        ctx.add(PaperContainerBackground {
            width: container_wh.width,
            height: container_wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER,
            outline_color: Some(palette::SURFACE_CONTAINER_OUTLINE),
            shadow: true,
            arrow: Some(PaperArrow {
                side: ArrowSide::Bottom,
                width: tooltip::ARROW_WIDTH,
                height: tooltip::ARROW_HEIGHT,
                offset: container_wh.width / 2.0,
            }),
        });
    }
}

fn render_name_text(
    ctx: ComposeCtx,
    slot_data: &crate::shop::ShopSlotData,
    locale: l10n::Locale,
    text_max: Px,
) {
    let key = match &slot_data.slot {
        ShopSlot::Item { item, .. } => format!("shop:{:?}:name", item.discriminant()),
        ShopSlot::Upgrade { upgrade, .. } => format!("shop:{upgrade:?}:name"),
    };

    ctx.add(memoized_text(
        (&key, &text_max, &locale.language),
        |mut builder| {
            builder
                .headline()
                .size(FontSize::Medium)
                .max_width(text_max)
                .color(palette::WHITE)
                .stroke(2.px(), palette::DARK_CHARCOAL);
            match &slot_data.slot {
                ShopSlot::Item { item, .. } => {
                    builder.l10n(l10n::item_kind::ItemText::Name((*item).clone()), &locale);
                }
                ShopSlot::Upgrade { upgrade, .. } => {
                    builder.l10n(l10n::upgrade::UpgradeTypeText::Name(upgrade), &locale);
                }
            }
            builder.render_left_top()
        },
    ));
}

fn render_description_text(
    ctx: ComposeCtx,
    slot_data: &crate::shop::ShopSlotData,
    locale: l10n::Locale,
    text_max: Px,
) {
    let key = match &slot_data.slot {
        ShopSlot::Item { item, .. } => format!("shop:{item:?}:description"),
        ShopSlot::Upgrade { upgrade, .. } => format!("shop:{upgrade:?}:description"),
    };

    ctx.add(memoized_text(
        (&key, &text_max, &locale.language),
        |mut builder| {
            builder
                .paragraph()
                .size(FontSize::Large)
                .max_width(text_max)
                .color(palette::WHITE)
                .stroke(2.px(), palette::DARK_CHARCOAL);
            match &slot_data.slot {
                ShopSlot::Item { item, .. } => {
                    builder.l10n(
                        l10n::item_kind::ItemText::Description((*item).clone()),
                        &locale,
                    );
                }
                ShopSlot::Upgrade { upgrade, .. } => {
                    builder.l10n(
                        l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(upgrade),
                        &locale,
                    );
                }
            }
            builder.render_left_top()
        },
    ));
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

use super::constants::PADDING;
use crate::game_state::shop_purchase::ShopPurchaseStatus;
use crate::icon::IconKind;
use crate::palette;
use crate::shop::{ShopSlot, ShopSlotData};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::{FontSize, memoized_text};
use namui::*;
use namui_prebuilt::table;

const PRICE_HEIGHT: Px = px(36.0);
const THUMBNAIL_STROKE: Px = px(6.0);
const PURCHASED_THUMBNAIL_OPACITY: f32 = 0.5;
const UNAVAILABLE_THUMBNAIL_OPACITY: f32 = 0.45;

pub struct ShopItem<'a> {
    pub wh: Wh<Px>,
    pub slot_data: &'a ShopSlotData,
    pub purchase_status: ShopPurchaseStatus,
}

impl Component for ShopItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            slot_data,
            purchase_status,
        } = self;

        let available = !slot_data.purchased && purchase_status.is_available();

        ctx.compose(|ctx| {
            table::padding_no_clip(PADDING, |wh, ctx| {
                ctx.compose(|ctx| {
                    render_thumbnail(wh, ctx, slot_data, &purchase_status);
                });
                render_price(wh, ctx, slot_data, available);
            })(wh, ctx);
        });
    }
}

fn render_thumbnail(
    wh: Wh<Px>,
    ctx: ComposeCtx,
    slot_data: &ShopSlotData,
    purchase_status: &ShopPurchaseStatus,
) {
    let thumbnail_size = (wh.width - PADDING * 2.0).min(wh.height - PRICE_HEIGHT - PADDING * 2.0);
    let thumbnail_wh = Wh::single(thumbnail_size);
    let thumbnail_xy = Xy::new((wh.width - thumbnail_size) / 2.0, PADDING);

    let thumbnail_opacity = if slot_data.purchased {
        PURCHASED_THUMBNAIL_OPACITY
    } else if !purchase_status.is_available() {
        UNAVAILABLE_THUMBNAIL_OPACITY
    } else {
        1.0
    };

    ctx.translate(thumbnail_xy)
        .compose(|ctx| match &slot_data.slot {
            ShopSlot::Item { item, .. } => {
                ctx.add(item.thumbnail_with_shadow_opacity(
                    thumbnail_wh,
                    THUMBNAIL_STROKE,
                    true,
                    thumbnail_opacity,
                ));
            }
            ShopSlot::Upgrade { upgrade, .. } => {
                ctx.add(upgrade.thumbnail_with_opacity(thumbnail_wh, true, thumbnail_opacity));
            }
            ShopSlot::CardService { card_service, .. } => {
                ctx.add(card_service.thumbnail_with_opacity(
                    thumbnail_wh,
                    THUMBNAIL_STROKE,
                    true,
                    thumbnail_opacity,
                ));
            }
        });
}

fn render_price(wh: Wh<Px>, ctx: ComposeCtx, slot_data: &ShopSlotData, available: bool) {
    let cost = match &slot_data.slot {
        ShopSlot::Item { cost, .. }
        | ShopSlot::Upgrade { cost, .. }
        | ShopSlot::CardService { cost, .. } => *cost,
    };
    let cost_color = if available {
        palette::YELLOW
    } else {
        palette::RED
    };
    let price_wh = Wh::new(wh.width, PRICE_HEIGHT);

    ctx.translate((0.px(), wh.height - PRICE_HEIGHT))
        .add(memoized_text(
            (&slot_data.id, &available, &cost),
            |mut builder| {
                builder
                    .headline()
                    .size(FontSize::Medium)
                    .stroke(2.px(), palette::DARK_CHARCOAL)
                    .color(cost_color)
                    .icon(IconKind::Gold)
                    .space()
                    .text(format!("{cost}"))
                    .render_center(price_wh)
            },
        ))
        .add(PaperContainerBackground {
            width: price_wh.width,
            height: price_wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Tape,
            color: match slot_data.slot {
                ShopSlot::Item { .. } => palette::GREEN,
                ShopSlot::Upgrade { .. } => palette::BLUE,
                ShopSlot::CardService { .. } => palette::YELLOW,
            },
            outline_color: Some(palette::WHITE),
            shadow: true,
            arrow: None,
        });
}

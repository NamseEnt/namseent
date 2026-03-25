use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::halo::Halo;
use namui::*;
use namui_prebuilt::table;

use crate::shop_panel::constants::PADDING;

fn halo_config_for_rarity(rarity: Rarity) -> Option<(Color, f32)> {
    match rarity {
        Rarity::Common => None,
        Rarity::Rare => Some((rarity.color(), 0.1)),
        Rarity::Epic => Some((rarity.color(), 0.2)),
        Rarity::Legendary => Some((rarity.color(), 0.4)),
    }
}

pub(crate) mod bottom;

pub(crate) fn render_body<'a>(ctx: &RenderCtx, params: super::ShopItemLayoutParams<'a>) {
    let super::ShopItemLayoutParams {
        wh,
        name,
        description,
        cost,
        available,
        item_kind,
        upgrade_kind,
        rarity,
    } = params;

    ctx.compose(|ctx| {
        table::vertical([
            table::fixed_no_clip(
                wh.width * 0.9,
                table::padding_no_clip(PADDING, |wh, ctx| {
                    ctx.translate(Xy::single(PADDING)).add(
                        Icon::new(IconKind::Rarity { rarity })
                            .size(IconSize::Large)
                            .wh(Wh::single(IconSize::Large.px())),
                    );
                    ctx.compose(|ctx| {
                        table::padding_no_clip(
                            PADDING,
                            table::horizontal([
                                table::ratio_no_clip(1, |_, _| {}),
                                table::fixed_no_clip(wh.height, |wh, ctx| {
                                    let halo_config = halo_config_for_rarity(rarity);

                                    ctx.compose(|ctx| {
                                        if let Some(kind) = item_kind {
                                            ctx.add(kind.thumbnail(wh));
                                        } else if let Some(upgrade) = upgrade_kind {
                                            ctx.add(upgrade.thumbnail(wh));
                                        } else {
                                            ctx.add(
                                                Icon::new(IconKind::Config)
                                                    .size(IconSize::Large)
                                                    .wh(wh),
                                            );
                                        }

                                        if let Some((color, strength)) = halo_config {
                                            ctx.add(Halo {
                                                wh,
                                                radius: 64.px(),
                                                color,
                                                strength,
                                                rotation_deg_per_sec: 45.0,
                                            });
                                        }
                                    });
                                }),
                                table::ratio_no_clip(1, |_, _| {}),
                            ]),
                        )(wh, ctx);
                    });
                }),
            ),
            table::ratio_no_clip(1, bottom::make_renderer(name, description, cost, available)),
        ])(wh, ctx);
    });
}

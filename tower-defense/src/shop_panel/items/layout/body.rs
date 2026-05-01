use crate::asset::image::thumbnail::rim;
use namui::*;
use namui_prebuilt::table;

use crate::shop_panel::constants::PADDING;

pub(crate) mod bottom;

pub(crate) fn render_body<'a>(ctx: &RenderCtx, params: super::ShopItemLayoutParams<'a>) {
    let super::ShopItemLayoutParams {
        wh,
        name,
        description,
        cost,
        available,
        item_kind,
        upgrade,
    } = params;

    let rim_image = if let Some(_upgrade) = upgrade {
        rim::UPGRADE
    } else {
        rim::ITEM
    };

    ctx.compose(|ctx| {
        table::vertical([
            table::fixed_no_clip(
                wh.width,
                table::padding_no_clip(PADDING, |wh, ctx| {
                    ctx.compose(|ctx| {
                        table::padding_no_clip(
                            PADDING,
                            table::horizontal([
                                table::ratio_no_clip(1, |_, _| {}),
                                table::fixed_no_clip(wh.height, |wh, ctx| {
                                    let padding = wh.height * 0.125;

                                    ctx.compose(|ctx| {
                                        table::padding_no_clip(padding, |inner_wh, inner_ctx| {
                                            if let Some(kind) = item_kind {
                                                inner_ctx.add(kind.thumbnail(inner_wh));
                                            } else if let Some(upgrade) = upgrade {
                                                inner_ctx.add(upgrade.thumbnail(inner_wh));
                                            } else {
                                                inner_ctx.add(
                                                    crate::thumbnail::render_placeholder_thumbnail(
                                                        inner_wh,
                                                        crate::thumbnail::STICKER_THUMBNAIL_STROKE,
                                                        false,
                                                    ),
                                                );
                                            }
                                        })(wh, ctx);
                                    });
                                }),
                                table::ratio_no_clip(1, |_, _| {}),
                            ]),
                        )(wh, ctx);
                    });
                    ctx.add(namui::image(ImageParam {
                        rect: wh.to_rect(),
                        image: rim_image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: None,
                        },
                    }));
                }),
            ),
            table::ratio_no_clip(1, bottom::make_renderer(name, description, cost, available)),
        ])(wh, ctx);
    });
}

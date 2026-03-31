use crate::icon::{Icon, IconKind, IconSize};
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
        upgrade_kind,
    } = params;

    ctx.compose(|ctx| {
        table::vertical([
            table::fixed_no_clip(
                wh.width * 0.9,
                table::padding_no_clip(PADDING, |wh, ctx| {
                    ctx.compose(|ctx| {
                        table::padding_no_clip(
                            PADDING,
                            table::horizontal([
                                table::ratio_no_clip(1, |_, _| {}),
                                table::fixed_no_clip(wh.height, |wh, ctx| {
                                    if let Some(kind) = item_kind {
                                        ctx.add(kind.thumbnail(wh));
                                    } else if let Some(upgrade) = upgrade_kind {
                                        ctx.add(upgrade.thumbnail(wh));
                                    } else {
                                        ctx.add(Icon::new(IconKind::Config).size(IconSize::Large).wh(wh));
                                    }
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

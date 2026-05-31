use crate::icon::IconKind;
use crate::l10n;
use crate::palette;
use crate::theme::typography::{FontSize, memoized_text};
use namui::{ComposeCtx, Px, PxExt, Wh};
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, table};

use crate::shop_panel::constants::PADDING;
use crate::shop_panel::items::backgrounds::ShopItemInfoBackground;
use crate::shop_panel::items::description::ShopItemDescription;
use crate::shop_panel::items::title::ShopItemTitle;

fn render_description(wh: Wh<Px>, ctx: ComposeCtx, description: &ShopItemDescription<'_>) {
    let wh = wh - Wh::single(PADDING * 2);
    ctx.add(AutoScrollViewWithCtx {
        wh,
        scroll_bar_width: PADDING,
        content: |ctx| {
            let description_key = description.key();

            ctx.translate((PADDING, PADDING)).add(memoized_text(
                (&description_key, &wh.width),
                |mut builder| {
                    builder
                        .paragraph()
                        .size(FontSize::Medium)
                        .bold()
                        .color(palette::WHITE)
                        .stroke(2.px(), palette::DARK_CHARCOAL)
                        .max_width(wh.width);
                    match description {
                        ShopItemDescription::Item { item, locale } => {
                            builder.l10n(
                                l10n::item_kind::ItemText::Description((*item).clone()),
                                locale,
                            );
                        }
                        ShopItemDescription::Upgrade { upgrade, locale } => {
                            builder.l10n(
                                l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(upgrade),
                                locale,
                            );
                        }
                    };
                    builder.render_left_top()
                },
            ));
        },
    });
}

fn render_cost_bar(wh: Wh<Px>, ctx: ComposeCtx, available: bool, cost: usize) {
    let cost_color = if available {
        palette::YELLOW
    } else {
        palette::RED
    };

    ctx.add(memoized_text((&available, &cost), |mut builder| {
        builder
            .headline()
            .stroke(2.px(), palette::DARK_CHARCOAL)
            .color(cost_color)
            .icon(IconKind::Gold)
            .space()
            .text(format!("{cost}"))
            .render_center(wh)
    }));
    ctx.add(ShopItemInfoBackground {
        wh,
        color: palette::SURFACE_CONTAINER_LOWEST,
    });
}

pub(crate) fn make_renderer<'a>(
    name: ShopItemTitle<'a>,
    description: ShopItemDescription<'a>,
    cost: usize,
    available: bool,
) -> impl FnOnce(Wh<Px>, ComposeCtx) + 'a {
    move |wh, ctx| {
        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::vertical([
                    table::fixed_no_clip(28.px(), move |title_wh, ctx| {
                        let name_key = name.key();
                        ctx.add(memoized_text(
                            (&name_key, &title_wh.width),
                            |mut builder| {
                                builder
                                    .headline()
                                    .size(FontSize::Small)
                                    .color(palette::WHITE)
                                    .stroke(2.px(), palette::DARK_CHARCOAL)
                                    .max_width(title_wh.width)
                                    .text_align(namui::TextAlign::Center);
                                match &name {
                                    ShopItemTitle::Item { item_kind, locale } => {
                                        builder.l10n(
                                            l10n::item_kind::ItemText::Name(item_kind.clone()),
                                            locale,
                                        );
                                    }
                                    ShopItemTitle::Upgrade { upgrade, locale } => {
                                        builder.l10n(
                                            l10n::upgrade::UpgradeTypeText::Name(upgrade),
                                            locale,
                                        );
                                    }
                                };
                                builder.render_center(title_wh)
                            },
                        ));
                    }),
                    table::fixed_no_clip(PADDING, |_, _| {}),
                    table::ratio_no_clip(1, move |wh, ctx| {
                        ctx.compose(|ctx| {
                            table::padding_no_clip(
                                PADDING,
                                table::vertical([
                                    table::ratio_no_clip(1, move |wh, ctx| {
                                        render_description(wh, ctx, &description);
                                    }),
                                    table::fixed_no_clip(PADDING, |_, _| {}),
                                    table::fixed_no_clip(48.px(), move |wh, ctx| {
                                        render_cost_bar(wh, ctx, available, cost);
                                    }),
                                ]),
                            )(wh, ctx);
                        });

                        ctx.add(ShopItemInfoBackground {
                            wh,
                            color: palette::SURFACE_CONTAINER_LOW,
                        });
                    }),
                ]),
            )(wh, ctx);
        });
    }
}

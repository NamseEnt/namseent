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
    ctx.add(AutoScrollViewWithCtx {
        wh,
        scroll_bar_width: PADDING,
        content: |ctx| {
            let description_key = description.key();

            ctx.add(memoized_text(
                (&description_key, &wh.width),
                |mut builder| {
                    builder
                        .paragraph()
                        .size(FontSize::Medium)
                        .max_width(wh.width);
                    match description {
                        ShopItemDescription::Item { item, locale } => {
                            builder.l10n(
                                l10n::item_kind::ItemText::Description((*item).clone()),
                                locale,
                            );
                        }
                        ShopItemDescription::Upgrade {
                            upgrade_kind,
                            locale,
                        } => {
                            builder.l10n(
                                l10n::upgrade::UpgradeKindText::Description(upgrade_kind),
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
    let color = if available {
        palette::ON_SURFACE
    } else {
        palette::ON_DISABLED_CONTAINER
    };
    ctx.add(memoized_text((&available, &cost), |mut builder| {
        builder
            .headline()
            .icon(IconKind::Gold)
            .space()
            .color(color)
            .text(format!("{cost}"))
            .render_center(wh)
    }));
    ctx.add(ShopItemInfoBackground {
        wh,
        color: palette::SURFACE_CONTAINER_LOWEST,
    });
}

pub(crate) fn make_renderer<'a>(
    name: ShopItemTitle,
    description: ShopItemDescription<'a>,
    cost: usize,
    available: bool,
) -> impl FnOnce(Wh<Px>, ComposeCtx) + 'a {
    move |wh, ctx| {
        ctx.compose(|ctx| {
            table::padding_no_clip(
                PADDING,
                table::vertical([
                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                        let name_key = name.key();
                        ctx.add(memoized_text((&name_key, &wh.width), |mut builder| {
                            builder.headline().size(FontSize::Small).max_width(wh.width);
                            match &name {
                                ShopItemTitle::Item { item_kind, locale } => {
                                    builder.l10n(
                                        l10n::item_kind::ItemText::Name(item_kind.clone()),
                                        locale,
                                    );
                                }
                                ShopItemTitle::Upgrade {
                                    upgrade_kind,
                                    locale,
                                } => {
                                    builder.l10n(
                                        l10n::upgrade::UpgradeKindText::Name(upgrade_kind),
                                        locale,
                                    );
                                }
                            };
                            builder.render_left_top()
                        }));
                    }),
                    table::fixed_no_clip(PADDING, |_, _| {}),
                    table::ratio_no_clip(1, move |wh, ctx| {
                        ctx.compose(|ctx| {
                            table::padding_no_clip(
                                PADDING,
                                table::vertical([
                                    table::ratio(1, move |wh, ctx| {
                                        ctx.add(AutoScrollViewWithCtx {
                                            wh,
                                            scroll_bar_width: PADDING,
                                            content: |ctx| {
                                                render_description(wh, ctx, &description);
                                            },
                                        });
                                    }),
                                    table::fixed_no_clip(PADDING, |_, _| {}),
                                    table::fixed_no_clip(
                                        if cost == 0 {
                                            0.0_f32.px()
                                        } else {
                                            48.0_f32.px()
                                        },
                                        move |wh, ctx| {
                                            if cost != 0 {
                                                render_cost_bar(wh, ctx, available, cost);
                                            }
                                        },
                                    ),
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

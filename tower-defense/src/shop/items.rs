use super::constants::{PADDING, SOLD_OUT_HEIGHT};
use super::slot::ShopSlot;
use crate::game_state::item::{self, Item};
use crate::game_state::upgrade::{Upgrade, UpgradeKind};
use crate::game_state::use_game_state;
use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::TopBarText;
use crate::palette;
use crate::theme::button::{Button, ButtonColor};
use crate::theme::typography::{FontSize, TextAlign, headline, paragraph};
use namui::*;
use namui_prebuilt::{simple_rect, table};

pub struct ShopItem<'a> {
    pub wh: Wh<Px>,
    pub shop_slot: &'a ShopSlot,
    pub shop_slot_index: usize,
    pub purchase_item: &'a dyn Fn(usize),
}

impl Component for ShopItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            shop_slot,
            shop_slot_index,
            purchase_item,
        } = self;

        let money = use_game_state(ctx).gold;
        let purchase_item = || purchase_item(shop_slot_index);

        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                match shop_slot {
                    ShopSlot::Locked => {
                        ctx.add(ShopItemLocked { wh });
                    }
                    ShopSlot::Item {
                        item,
                        cost,
                        purchased,
                    } => {
                        ctx.add(ShopItemContent {
                            wh,
                            item,
                            purchase_item: &purchase_item,
                            cost: *cost,
                            purchased: *purchased,
                            not_enough_money: money < *cost,
                        });
                    }
                    ShopSlot::Upgrade {
                        upgrade,
                        cost,
                        purchased,
                    } => {
                        ctx.add(ShopUpgradeContent {
                            wh,
                            upgrade,
                            purchase_upgrade: &purchase_item,
                            cost: *cost,
                            purchased: *purchased,
                            not_enough_money: money < *cost,
                        });
                    }
                }

                ctx.add(rect(RectParam {
                    rect: wh.to_rect(),
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: palette::OUTLINE,
                            width: 1.px(),
                            border_position: BorderPosition::Inside,
                        }),
                        fill: Some(RectFill {
                            color: palette::SURFACE_CONTAINER,
                        }),
                        round: Some(RectRound {
                            radius: palette::ROUND,
                        }),
                    },
                }));
            })(wh, ctx);
        });
    }
}

pub struct ShopItemLocked {
    pub wh: Wh<Px>,
}

impl Component for ShopItemLocked {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(36.px(), |wh, ctx| {
                    ctx.add(Icon::new(IconKind::Lock).size(IconSize::Large).wh(wh));
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}

pub struct ShopItemContent<'a> {
    pub wh: Wh<Px>,
    pub item: &'a Item,
    pub purchase_item: &'a dyn Fn(),
    pub cost: usize,
    pub purchased: bool,
    pub not_enough_money: bool,
}

struct ShopItemLayoutParams<'a> {
    wh: Wh<Px>,
    name: String,
    description: String,
    cost: usize,
    purchased: bool,
    available: bool,
    purchase_action: &'a dyn Fn(),
    item_kind: Option<&'a item::ItemKind>,
    upgrade_kind: Option<&'a UpgradeKind>,
}

fn render_shop_item_layout(params: ShopItemLayoutParams, ctx: &RenderCtx) {
    let ShopItemLayoutParams {
        wh,
        name,
        description,
        cost,
        purchased,
        available,
        purchase_action,
        item_kind,
        upgrade_kind,
    } = params;

    ctx.compose(|ctx| {
        if !purchased {
            return;
        }
        ctx.add(ShopItemSoldOut { wh });
    });

    ctx.compose(|ctx| {
        table::vertical([
            table::fixed(
                wh.width,
                table::padding(PADDING, |wh, ctx| {
                    if let Some(kind) = item_kind {
                        ctx.add(kind.thumbnail(wh));
                    } else if let Some(upgrade) = upgrade_kind {
                        ctx.add(upgrade.thumbnail(wh));
                    } else {
                        // 기본 아이콘
                        ctx.add(Icon::new(IconKind::Config).size(IconSize::Large).wh(wh));
                    }
                }),
            ),
            table::ratio(
                1,
                table::padding(
                    PADDING,
                    table::vertical([
                        table::fixed(PADDING, |_, _| {}),
                        table::fit(table::FitAlign::LeftTop, move |ctx| {
                            ctx.add(
                                headline(name)
                                    .size(FontSize::Small)
                                    .align(TextAlign::LeftTop)
                                    .max_width(wh.width)
                                    .build_rich(),
                            );
                        }),
                        table::fixed(PADDING, |_, _| {}),
                        table::ratio(1, move |wh, ctx| {
                            ctx.add(
                                paragraph(description.clone())
                                    .size(FontSize::Medium)
                                    .align(TextAlign::LeftTop)
                                    .max_width(wh.width)
                                    .build_rich(),
                            );
                        }),
                        table::fixed(PADDING, |_, _| {}),
                        table::fixed(48.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        if !available {
                                            return;
                                        }
                                        purchase_action();
                                    },
                                    &|wh, color, ctx| {
                                        ctx.add(
                                            headline(format!(
                                                "{} {cost}",
                                                Icon::new(IconKind::Gold)
                                                    .size(IconSize::Large)
                                                    .wh(Wh::single(wh.height))
                                                    .as_tag(),
                                            ))
                                            .color(color)
                                            .build_rich(),
                                        );
                                    },
                                )
                                .color(if available {
                                    ButtonColor::Primary
                                } else {
                                    ButtonColor::Secondary
                                })
                                .disabled(!available),
                            );
                        }),
                    ]),
                ),
            ),
        ])(wh, ctx);
    });
}

impl Component for ShopItemContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            item,
            purchase_item,
            cost,
            purchased,
            not_enough_money,
        } = self;
        let game_state = use_game_state(ctx);
        let available = !purchased && !not_enough_money;
        let name = item.kind.name(&game_state.text());
        let description = item.kind.description(&game_state.text());

        render_shop_item_layout(
            ShopItemLayoutParams {
                wh,
                name,
                description,
                cost,
                purchased,
                available,
                purchase_action: purchase_item,
                item_kind: Some(&item.kind),
                upgrade_kind: None,
            },
            ctx,
        );
    }
}

struct ShopUpgradeContent<'a> {
    wh: Wh<Px>,
    upgrade: &'a Upgrade,
    purchase_upgrade: &'a dyn Fn(),
    cost: usize,
    purchased: bool,
    not_enough_money: bool,
}

impl Component for ShopUpgradeContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade,
            purchase_upgrade,
            cost,
            purchased,
            not_enough_money,
        } = self;
        let game_state = use_game_state(ctx);
        let available = !purchased && !not_enough_money;
        let name = upgrade.kind.name(&game_state.text());
        let description = upgrade.kind.description(&game_state.text());

        render_shop_item_layout(
            ShopItemLayoutParams {
                wh,
                name,
                description,
                cost,
                purchased,
                available,
                purchase_action: purchase_upgrade,
                item_kind: None,
                upgrade_kind: Some(&upgrade.kind),
            },
            ctx,
        );
    }
}

struct ShopItemSoldOut {
    wh: Wh<Px>,
}

impl Component for ShopItemSoldOut {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(SOLD_OUT_HEIGHT, |wh, ctx| {
                    ctx.add(
                        headline(game_state.text().ui(TopBarText::SoldOut).to_string())
                            .size(FontSize::Medium)
                            .align(TextAlign::Center { wh })
                            .build(),
                    );
                    ctx.add(simple_rect(
                        wh,
                        Color::TRANSPARENT,
                        0.px(),
                        palette::SECONDARY,
                    ));
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}

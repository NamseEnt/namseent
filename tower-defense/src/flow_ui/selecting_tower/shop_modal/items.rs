use super::constants::{PADDING, SOLD_OUT_HEIGHT};
use crate::game_state::contract::Contract;
use crate::game_state::item::{Effect, Item};
use crate::game_state::upgrade::{Upgrade, UpgradeKind};
use crate::game_state::use_game_state;
use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::TopBarText;
use crate::palette;
use crate::shop::{ShopSlot, ShopSlotData, ShopSlotId};
use crate::theme::button::{Button, ButtonColor};
use crate::theme::typography::{self, FontSize};
use crate::thumbnail::ThumbnailComposer;
use namui::*;
use namui_prebuilt::{simple_rect, table};

pub struct ShopItem<'a> {
    pub wh: Wh<Px>,
    pub slot_data: &'a ShopSlotData,
    pub purchase_item: &'a dyn Fn(ShopSlotId),
    pub can_purchase_item: bool,
}

impl Component for ShopItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            slot_data,
            purchase_item,
            can_purchase_item,
        } = self;

        let slot_id = slot_data.id;
        let purchase_item_fn = || purchase_item(slot_id);

        ctx.compose(|ctx| {
            table::padding_no_clip(PADDING, |wh, ctx| {
                match &slot_data.slot {
                    ShopSlot::Locked => {
                        ctx.add(ShopItemLocked { wh });
                    }
                    ShopSlot::Item { item, cost } => {
                        ctx.add(ShopItemContent {
                            wh,
                            item,
                            purchase_item: &purchase_item_fn,
                            cost: *cost,
                            purchased: slot_data.purchased,
                            disabled: !can_purchase_item,
                        });
                    }
                    ShopSlot::Upgrade { upgrade, cost } => {
                        ctx.add(ShopUpgradeContent {
                            wh,
                            upgrade,
                            purchase_upgrade: &purchase_item_fn,
                            cost: *cost,
                            purchased: slot_data.purchased,
                            disabled: !can_purchase_item,
                        });
                    }
                    ShopSlot::Contract { contract, cost } => {
                        ctx.add(ShopContractContent {
                            wh,
                            contract,
                            purchase_contract: &purchase_item_fn,
                            cost: *cost,
                            purchased: slot_data.purchased,
                            disabled: !can_purchase_item,
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
    pub disabled: bool,
}

struct ShopItemLayoutParams<'a> {
    wh: Wh<Px>,
    name: String,
    description: String,
    cost: usize,
    purchased: bool,
    available: bool,
    purchase_action: &'a dyn Fn(),
    item_kind: Option<&'a Effect>,
    upgrade_kind: Option<&'a UpgradeKind>,
    contract_kind: Option<&'a Contract>,
    rarity: crate::rarity::Rarity,
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
        contract_kind,
        rarity,
    } = params;

    ctx.compose(|ctx| {
        if !purchased {
            return;
        }
        ctx.add(ShopItemSoldOut { wh });
    });

    ctx.compose(|ctx| {
        table::vertical([
            table::fixed_no_clip(
                wh.width,
                table::padding_no_clip(PADDING, |wh, ctx| {
                    ctx.translate(Xy::single(PADDING)).add(
                        Icon::new(IconKind::Rarity { rarity })
                            .size(IconSize::Large)
                            .wh(Wh::single(IconSize::Large.px())),
                    );
                    ctx.compose(|ctx| {
                        table::padding(PADDING, |wh, ctx| {
                            if let Some(kind) = item_kind {
                                ctx.add(kind.thumbnail(wh));
                            } else if let Some(upgrade) = upgrade_kind {
                                ctx.add(upgrade.thumbnail(wh));
                            } else if contract_kind.is_some() {
                                ctx.add(
                                    ThumbnailComposer::new(wh)
                                        .with_icon_base(IconKind::Contract)
                                        .build(),
                                );
                            } else {
                                // 기본 아이콘
                                ctx.add(Icon::new(IconKind::Config).size(IconSize::Large).wh(wh));
                            }
                        })(wh, ctx);
                    });
                    ctx.add(rect(RectParam {
                        rect: wh.to_rect(),
                        style: RectStyle {
                            stroke: Some(RectStroke {
                                color: palette::OUTLINE,
                                width: 1.px(),
                                border_position: BorderPosition::Inside,
                            }),
                            fill: Some(RectFill {
                                color: palette::SURFACE_CONTAINER_LOWEST,
                            }),
                            round: Some(RectRound {
                                radius: palette::ROUND,
                            }),
                        },
                    }));
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
                                typography::headline()
                                    .size(FontSize::Small)
                                    .max_width(wh.width)
                                    .text(&name)
                                    .left_top(),
                            );
                        }),
                        table::fixed(PADDING, |_, _| {}),
                        table::ratio(1, move |wh, ctx| {
                            ctx.add(
                                typography::paragraph()
                                    .size(FontSize::Medium)
                                    .max_width(wh.width)
                                    .text(&description)
                                    .left_top(),
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
                                            typography::headline()
                                                .icon::<()>(IconKind::Gold)
                                                .space()
                                                .color(color)
                                                .text(format!("{cost}"))
                                                .center(wh),
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
            disabled,
        } = self;
        let game_state = use_game_state(ctx);
        let available = !purchased && !disabled;
        let name = item.name(&game_state.text());
        let description = item.description(&game_state.text());

        render_shop_item_layout(
            ShopItemLayoutParams {
                wh,
                name,
                description,
                cost,
                purchased,
                available,
                purchase_action: purchase_item,
                item_kind: Some(&item.effect),
                upgrade_kind: None,
                contract_kind: None,
                rarity: item.rarity,
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
    disabled: bool,
}

impl Component for ShopUpgradeContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade,
            purchase_upgrade,
            cost,
            purchased,
            disabled,
        } = self;
        let game_state = use_game_state(ctx);
        let available = !purchased && !disabled;
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
                contract_kind: None,
                rarity: upgrade.rarity,
            },
            ctx,
        );
    }
}

struct ShopContractContent<'a> {
    wh: Wh<Px>,
    contract: &'a Contract,
    purchase_contract: &'a dyn Fn(),
    cost: usize,
    purchased: bool,
    disabled: bool,
}

impl Component for ShopContractContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            contract,
            purchase_contract,
            cost,
            purchased,
            disabled,
        } = self;
        let available = !purchased && !disabled;
        let game_state = use_game_state(ctx);
        let name = game_state
            .text()
            .contract_name(crate::l10n::contract::ContractNameText::Rarity(
                contract.rarity,
            ))
            .to_string();
        let duration_text = game_state.text().contract_duration(&contract.status);
        let risk_text = game_state
            .text()
            .contract(crate::l10n::contract::ContractText::Risk(&contract.risk));
        let reward_text = game_state
            .text()
            .contract(crate::l10n::contract::ContractText::Reward(
                &contract.reward,
            ));
        let description = if duration_text.is_empty() {
            format!("{}\n{}", risk_text, reward_text)
        } else {
            format!("{}\n{}\n{}", duration_text, risk_text, reward_text)
        };

        render_shop_item_layout(
            ShopItemLayoutParams {
                wh,
                name,
                description,
                cost,
                purchased,
                available,
                purchase_action: purchase_contract,
                item_kind: None,
                upgrade_kind: None,
                contract_kind: Some(contract),
                rarity: contract.rarity,
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
                        typography::headline()
                            .size(FontSize::Medium)
                            .text(game_state.text().ui(TopBarText::SoldOut))
                            .center(wh),
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

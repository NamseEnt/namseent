use crate::card::{REVERSED_RANKS, SUITS};
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::{Upgrade, UpgradeKind};
use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::palette;
use crate::theme::typography::{self};
use namui::*;
use namui_prebuilt::table;
use rand::{Rng, seq::SliceRandom, thread_rng};

const BUTTON_HEIGHT: Px = px(36.);
const GAP: Px = px(8.);
const DROPDOWN_ICON_SIZE: Px = px(16.);
const DROPDOWN_ITEM_HEIGHT: Px = px(32.);

// Dropdown type: 0 = none, 1 = Kind, 2 = Rarity
const RARITIES: [Rarity; 4] = [
    Rarity::Common,
    Rarity::Rare,
    Rarity::Epic,
    Rarity::Legendary,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeCategory {
    Random,
    GoldEarnPlus,
    RankDamage,
    SuitDamage,
    HandDamage,
    ShopSlotExpansion,
    RerollCountPlus,
    LowCardDamage,
    ShopItemPriceMinus,
    ShopRefreshPlus,
    NoRerollDamage,
    EvenOddDamage,
    FaceNumberDamage,
    ShortenStraightFlush,
    SkipRankForStraight,
    TreatSuitsAsSame,
    RerollDamage,
}

const UPGRADE_CATEGORIES: [UpgradeCategory; 17] = [
    UpgradeCategory::Random,
    UpgradeCategory::GoldEarnPlus,
    UpgradeCategory::RankDamage,
    UpgradeCategory::SuitDamage,
    UpgradeCategory::HandDamage,
    UpgradeCategory::ShopSlotExpansion,
    UpgradeCategory::RerollCountPlus,
    UpgradeCategory::LowCardDamage,
    UpgradeCategory::ShopItemPriceMinus,
    UpgradeCategory::ShopRefreshPlus,
    UpgradeCategory::NoRerollDamage,
    UpgradeCategory::EvenOddDamage,
    UpgradeCategory::FaceNumberDamage,
    UpgradeCategory::ShortenStraightFlush,
    UpgradeCategory::SkipRankForStraight,
    UpgradeCategory::TreatSuitsAsSame,
    UpgradeCategory::RerollDamage,
];

const EXPECTED_UPGRADES_BY_STAGE: [(Rarity, UpgradeCategory); 50] = [
    (Rarity::Common, UpgradeCategory::NoRerollDamage),
    (Rarity::Common, UpgradeCategory::ShopItemPriceMinus),
    (Rarity::Common, UpgradeCategory::HandDamage),
    (Rarity::Common, UpgradeCategory::RerollDamage),
    (Rarity::Common, UpgradeCategory::GoldEarnPlus),
    (Rarity::Common, UpgradeCategory::ShopItemPriceMinus),
    (Rarity::Common, UpgradeCategory::LowCardDamage),
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Common, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Common, UpgradeCategory::ShopItemPriceMinus),
    (Rarity::Common, UpgradeCategory::HandDamage),
    (Rarity::Epic, UpgradeCategory::SkipRankForStraight),
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Common, UpgradeCategory::NoRerollDamage),
    (Rarity::Rare, UpgradeCategory::RerollCountPlus),
    (Rarity::Rare, UpgradeCategory::ShopRefreshPlus),
    (Rarity::Epic, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Rare, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::LowCardDamage),
    (Rarity::Epic, UpgradeCategory::FaceNumberDamage),
    (Rarity::Epic, UpgradeCategory::RerollCountPlus),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::ShopSlotExpansion),
    (Rarity::Legendary, UpgradeCategory::ShopSlotExpansion),
    (Rarity::Epic, UpgradeCategory::ShopRefreshPlus),
    (Rarity::Epic, UpgradeCategory::FaceNumberDamage),
    (Rarity::Legendary, UpgradeCategory::RerollDamage),
    (Rarity::Rare, UpgradeCategory::GoldEarnPlus),
    (Rarity::Legendary, UpgradeCategory::FaceNumberDamage),
    (Rarity::Legendary, UpgradeCategory::NoRerollDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Epic, UpgradeCategory::SkipRankForStraight),
    (Rarity::Epic, UpgradeCategory::HandDamage),
    (Rarity::Rare, UpgradeCategory::GoldEarnPlus),
    (Rarity::Legendary, UpgradeCategory::LowCardDamage),
    (Rarity::Epic, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Rare, UpgradeCategory::HandDamage),
    (Rarity::Rare, UpgradeCategory::RankDamage),
    (Rarity::Common, UpgradeCategory::RankDamage),
    (Rarity::Legendary, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::HandDamage),
    (Rarity::Rare, UpgradeCategory::RerollDamage),
    (Rarity::Legendary, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::LowCardDamage),
];

pub fn get_expected_upgrade_for_stage(stage: usize) -> (Rarity, UpgradeCategory) {
    if stage == 0 || stage > 50 {
        (Rarity::Common, UpgradeCategory::GoldEarnPlus)
    } else {
        EXPECTED_UPGRADES_BY_STAGE[stage - 1]
    }
}

impl UpgradeCategory {
    fn display_name(&self) -> &'static str {
        match self {
            UpgradeCategory::Random => "Random",
            UpgradeCategory::GoldEarnPlus => "Gold Earn +",
            UpgradeCategory::RankDamage => "Rank Damage",
            UpgradeCategory::SuitDamage => "Suit Damage",
            UpgradeCategory::HandDamage => "Hand Damage",
            UpgradeCategory::ShopSlotExpansion => "Shop Slot +",
            UpgradeCategory::RerollCountPlus => "Reroll Count +",
            UpgradeCategory::LowCardDamage => "Low Card Damage",
            UpgradeCategory::ShopItemPriceMinus => "Shop Price -",
            UpgradeCategory::ShopRefreshPlus => "Shop Refresh +",
            UpgradeCategory::NoRerollDamage => "No Reroll Damage",
            UpgradeCategory::EvenOddDamage => "Even/Odd Damage",
            UpgradeCategory::FaceNumberDamage => "Face/Number Damage",
            UpgradeCategory::ShortenStraightFlush => "Shorten Straight Flush",
            UpgradeCategory::SkipRankForStraight => "Skip Rank for Straight",
            UpgradeCategory::TreatSuitsAsSame => "Treat Suits as Same",
            UpgradeCategory::RerollDamage => "Reroll Damage",
        }
    }

    pub fn generate_upgrade_kind(&self, rarity: Rarity) -> UpgradeKind {
        let mut rng = thread_rng();
        match self {
            UpgradeCategory::Random => panic!("Should not generate Random category"),
            UpgradeCategory::GoldEarnPlus => UpgradeKind::GoldEarnPlus,
            UpgradeCategory::RankDamage => UpgradeKind::RankAttackDamageMultiply {
                rank: *REVERSED_RANKS.choose(&mut rng).unwrap(),
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5)),
            },
            UpgradeCategory::SuitDamage => UpgradeKind::SuitAttackDamageMultiply {
                suit: *SUITS.choose(&mut rng).unwrap(),
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5)),
            },
            UpgradeCategory::HandDamage => {
                let tower_kinds = [
                    TowerKind::High,
                    TowerKind::OnePair,
                    TowerKind::TwoPair,
                    TowerKind::ThreeOfAKind,
                    TowerKind::Straight,
                    TowerKind::Flush,
                    TowerKind::FullHouse,
                    TowerKind::FourOfAKind,
                    TowerKind::StraightFlush,
                    TowerKind::RoyalFlush,
                ];
                UpgradeKind::HandAttackDamageMultiply {
                    tower_kind: *tower_kinds.choose(&mut rng).unwrap(),
                    damage_multiplier: rarity_gen(
                        rarity,
                        (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5),
                    ),
                }
            }
            UpgradeCategory::ShopSlotExpansion => UpgradeKind::ShopSlotExpansion,
            UpgradeCategory::RerollCountPlus => UpgradeKind::RerollCountPlus,
            UpgradeCategory::LowCardDamage => UpgradeKind::LowCardTowerDamageMultiply {
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            UpgradeCategory::ShopItemPriceMinus => UpgradeKind::ShopItemPriceMinus,
            UpgradeCategory::ShopRefreshPlus => UpgradeKind::ShopRefreshPlus,
            UpgradeCategory::NoRerollDamage => UpgradeKind::NoRerollTowerAttackDamageMultiply {
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            UpgradeCategory::EvenOddDamage => UpgradeKind::EvenOddTowerAttackDamageMultiply {
                even: rng.gen_bool(0.5),
                damage_multiplier: rarity_gen(rarity, (1.1..1.2, 1.2..1.4, 1.4..1.5, 1.5..1.6)),
            },
            UpgradeCategory::FaceNumberDamage => {
                UpgradeKind::FaceNumberCardTowerAttackDamageMultiply {
                    face: rng.gen_bool(0.5),
                    damage_multiplier: rarity_gen(rarity, (1.1..1.2, 1.2..1.4, 1.4..1.5, 1.5..1.6)),
                }
            }
            UpgradeCategory::ShortenStraightFlush => UpgradeKind::ShortenStraightFlushTo4Cards,
            UpgradeCategory::SkipRankForStraight => UpgradeKind::SkipRankForStraight,
            UpgradeCategory::TreatSuitsAsSame => UpgradeKind::TreatSuitsAsSame,
            UpgradeCategory::RerollDamage => UpgradeKind::RerollTowerAttackDamageMultiply {
                damage_multiplier: rarity_gen(
                    rarity,
                    (1.1..1.15, 1.15..1.25, 1.25..1.35, 1.35..1.5),
                ),
            },
        }
    }
}

fn rarity_gen(
    rarity: Rarity,
    ranges: (
        std::ops::Range<f32>,
        std::ops::Range<f32>,
        std::ops::Range<f32>,
        std::ops::Range<f32>,
    ),
) -> f32 {
    thread_rng().gen_range(match rarity {
        Rarity::Common => ranges.0,
        Rarity::Rare => ranges.1,
        Rarity::Epic => ranges.2,
        Rarity::Legendary => ranges.3,
    })
}

pub struct AddUpgradeTool {
    pub width: Px,
}

impl Component for AddUpgradeTool {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let (expected_rarity, expected_category) = get_expected_upgrade_for_stage(game_state.stage);
        let expected_category_idx = UPGRADE_CATEGORIES
            .iter()
            .position(|&c| c == expected_category)
            .unwrap_or(0);

        // Using index instead of enum for State compatibility
        let (selected_category_idx, set_selected_category_idx) =
            ctx.state(|| expected_category_idx);
        let (selected_rarity, set_selected_rarity) = ctx.state(|| expected_rarity);
        // 0 = none, 1 = Category, 2 = Rarity
        let (dropdown, set_dropdown) = ctx.state(|| 0u8);

        let selected_category = UPGRADE_CATEGORIES[*selected_category_idx];

        let add_upgrade = || {
            let category = selected_category;
            let rarity = *selected_rarity;
            mutate_game_state(move |gs| {
                let upgrade = if category == UpgradeCategory::Random {
                    crate::game_state::upgrade::generate_upgrade(gs, rarity)
                } else {
                    let kind = category.generate_upgrade_kind(rarity);
                    let value = thread_rng().gen_range(0.0..=1.0);
                    Upgrade {
                        kind,
                        rarity,
                        value: value.into(),
                    }
                };
                gs.upgrade_state.upgrade(upgrade);
            });
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(typography::headline().text("Add upgrade").left_top());
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    let info_text = format!(
                        "Stage {}: Expected - {:?} {}",
                        game_state.stage,
                        expected_rarity,
                        expected_category.display_name()
                    );
                    ctx.add(
                        typography::paragraph()
                            .color(palette::ON_SURFACE_VARIANT)
                            .text(&info_text)
                            .left_top(),
                    );
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    table::horizontal([
                        table::ratio(1, |wh, ctx| {
                            let text = selected_category.display_name();
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        if *dropdown == 1 {
                                            set_dropdown.set(0);
                                        } else {
                                            set_dropdown.set(1);
                                        }
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.compose(|ctx| {
                                            table::horizontal([
                                                table::ratio(1, |wh, ctx| {
                                                    ctx.add(
                                                        typography::paragraph()
                                                            .color(text_color)
                                                            .text(text)
                                                            .left_center(wh.height),
                                                    );
                                                }),
                                                table::fixed(DROPDOWN_ICON_SIZE, |wh, ctx| {
                                                    ctx.add(
                                                        Icon::new(IconKind::Down)
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                    );
                                                }),
                                            ])(wh, ctx);
                                        });
                                    },
                                )
                                .variant(ButtonVariant::Outlined),
                            );
                        }),
                        table::fixed(GAP, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            let text = format!("{:?}", *selected_rarity);
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        if *dropdown == 2 {
                                            set_dropdown.set(0);
                                        } else {
                                            set_dropdown.set(2);
                                        }
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.compose(|ctx| {
                                            table::horizontal([
                                                table::ratio(1, |wh, ctx| {
                                                    ctx.add(
                                                        typography::paragraph()
                                                            .color(text_color)
                                                            .text(&text)
                                                            .left_center(wh.height),
                                                    );
                                                }),
                                                table::fixed(DROPDOWN_ICON_SIZE, |wh, ctx| {
                                                    ctx.add(
                                                        Icon::new(IconKind::Down)
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                    );
                                                }),
                                            ])(wh, ctx);
                                        });
                                    },
                                )
                                .variant(ButtonVariant::Outlined),
                            );
                        }),
                    ])(Wh::new(self.width, BUTTON_HEIGHT), ctx);
                }),
                table::fixed(GAP, |_, _| {}),
                // Render dropdown menu if open
                match *dropdown {
                    1 => table::fit(table::FitAlign::LeftTop, |ctx| {
                        let selector_width = (self.width - GAP) / 2.;
                        table::vertical(
                            UPGRADE_CATEGORIES
                                .iter()
                                .enumerate()
                                .map(|(idx, category)| {
                                    let category = *category;

                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        let text = category.display_name();
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_category_idx.set(idx);
                                                    set_dropdown.set(0);
                                                },
                                                &|wh, text_color, ctx| {
                                                    ctx.add(
                                                        typography::paragraph()
                                                            .color(text_color)
                                                            .text(text)
                                                            .left_center(wh.height),
                                                    );
                                                },
                                            )
                                            .variant(
                                                if *selected_category_idx == idx {
                                                    ButtonVariant::Contained
                                                } else {
                                                    ButtonVariant::Outlined
                                                },
                                            ),
                                        );
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )(Wh::new(selector_width, f32::MAX.px()), ctx);
                    }),
                    2 => table::fit(table::FitAlign::LeftTop, |ctx| {
                        let selector_width = (self.width - GAP) / 2.;
                        table::vertical(
                            RARITIES
                                .iter()
                                .map(|rarity| {
                                    let rarity = *rarity;

                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        let text = format!("{:?}", rarity);
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_rarity.set(rarity);
                                                    set_dropdown.set(0);
                                                },
                                                &|wh, text_color, ctx| {
                                                    ctx.add(
                                                        typography::paragraph()
                                                            .color(text_color)
                                                            .text(&text)
                                                            .left_center(wh.height),
                                                    );
                                                },
                                            )
                                            .variant(
                                                if *selected_rarity == rarity {
                                                    ButtonVariant::Contained
                                                } else {
                                                    ButtonVariant::Outlined
                                                },
                                            ),
                                        );
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )(Wh::new(selector_width, f32::MAX.px()), ctx);
                    }),
                    _ => table::fixed(0.px(), |_, _| {}),
                },
                table::fixed(GAP * 2.0, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &add_upgrade,
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    typography::paragraph()
                                        .color(text_color)
                                        .text("업그레이드 획득")
                                        .center(wh),
                                );
                            },
                        )
                        .variant(ButtonVariant::Contained),
                    );
                }),
            ])(Wh::new(self.width, f32::MAX.px()), ctx);
        });
    }
}

impl AddUpgradeTool {}

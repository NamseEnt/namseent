use crate::card::SUITS;
use crate::game_state::upgrade::{Upgrade, UpgradeKind};
use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::palette;
use crate::theme::typography::memoized_text;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum UpgradeCategory {
    Random,
    Magnet,
    SuitDamage,
    ShopSlotExpansion,
    ExtraDice,
    LowCardDamage,
    ShopItemPriceMinus,
    NoRerollDamage,
    EvenOddDamage,
    FaceNumberDamage,
    ShortenStraightFlush,
    SkipRankForStraight,
    TreatSuitsAsSame,
    RerollDamage,
}

const UPGRADE_CATEGORIES: [UpgradeCategory; 14] = [
    UpgradeCategory::Random,
    UpgradeCategory::Magnet,
    UpgradeCategory::SuitDamage,
    UpgradeCategory::ShopSlotExpansion,
    UpgradeCategory::ExtraDice,
    UpgradeCategory::LowCardDamage,
    UpgradeCategory::ShopItemPriceMinus,
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
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Common, UpgradeCategory::RerollDamage),
    (Rarity::Common, UpgradeCategory::Magnet),
    (Rarity::Common, UpgradeCategory::ShopItemPriceMinus),
    (Rarity::Common, UpgradeCategory::LowCardDamage),
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Common, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Common, UpgradeCategory::ShopItemPriceMinus),
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SkipRankForStraight),
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Common, UpgradeCategory::NoRerollDamage),
    (Rarity::Rare, UpgradeCategory::ExtraDice),
    (Rarity::Rare, UpgradeCategory::ExtraDice),
    (Rarity::Epic, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Rare, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::LowCardDamage),
    (Rarity::Epic, UpgradeCategory::FaceNumberDamage),
    (Rarity::Epic, UpgradeCategory::ExtraDice),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::ShopSlotExpansion),
    (Rarity::Legendary, UpgradeCategory::ShopSlotExpansion),
    (Rarity::Epic, UpgradeCategory::ExtraDice),
    (Rarity::Epic, UpgradeCategory::FaceNumberDamage),
    (Rarity::Legendary, UpgradeCategory::RerollDamage),
    (Rarity::Rare, UpgradeCategory::Magnet),
    (Rarity::Legendary, UpgradeCategory::FaceNumberDamage),
    (Rarity::Legendary, UpgradeCategory::NoRerollDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Epic, UpgradeCategory::SkipRankForStraight),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Rare, UpgradeCategory::Magnet),
    (Rarity::Legendary, UpgradeCategory::LowCardDamage),
    (Rarity::Epic, UpgradeCategory::TreatSuitsAsSame),
    (Rarity::Rare, UpgradeCategory::SuitDamage),
    (Rarity::Rare, UpgradeCategory::SuitDamage),
    (Rarity::Common, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Legendary, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::SuitDamage),
    (Rarity::Rare, UpgradeCategory::RerollDamage),
    (Rarity::Legendary, UpgradeCategory::SuitDamage),
    (Rarity::Epic, UpgradeCategory::LowCardDamage),
];

pub fn get_expected_upgrade_for_stage(stage: usize) -> (Rarity, UpgradeCategory) {
    if stage == 0 || stage > 50 {
        (Rarity::Common, UpgradeCategory::Magnet)
    } else {
        EXPECTED_UPGRADES_BY_STAGE[stage - 1]
    }
}

impl UpgradeCategory {
    fn display_name(&self) -> &'static str {
        match self {
            UpgradeCategory::Random => "Random",
            UpgradeCategory::Magnet => "Cat",
            UpgradeCategory::SuitDamage => "Suit",

            UpgradeCategory::ShopSlotExpansion => "Backpack",
            UpgradeCategory::ExtraDice => "Dice Bundle",
            UpgradeCategory::LowCardDamage => "Tricycle",
            UpgradeCategory::ShopItemPriceMinus => "Energy Drink",
            UpgradeCategory::NoRerollDamage => "Perfect Pottery",
            UpgradeCategory::EvenOddDamage => "Chopsticks",
            UpgradeCategory::FaceNumberDamage => "Pen / Brush",
            UpgradeCategory::ShortenStraightFlush => "Four Leaf Clover",
            UpgradeCategory::SkipRankForStraight => "Rabbit",
            UpgradeCategory::TreatSuitsAsSame => "Black & White",
            UpgradeCategory::RerollDamage => "Broken Pottery",
        }
    }

    pub fn generate_upgrade_kind(&self, rarity: Rarity) -> UpgradeKind {
        let mut rng = thread_rng();
        match self {
            UpgradeCategory::Random => panic!("Should not generate Random category"),
            UpgradeCategory::Magnet => UpgradeKind::Cat { add: 1 },
            UpgradeCategory::SuitDamage => {
                let suit = *SUITS.choose(&mut rng).unwrap();
                let damage_multiplier =
                    rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5));
                match suit {
                    crate::card::Suit::Diamonds => UpgradeKind::Staff { damage_multiplier },
                    crate::card::Suit::Spades => UpgradeKind::LongSword { damage_multiplier },
                    crate::card::Suit::Hearts => UpgradeKind::Mace { damage_multiplier },
                    crate::card::Suit::Clubs => UpgradeKind::ClubSword { damage_multiplier },
                }
            }
            UpgradeCategory::ShopSlotExpansion => UpgradeKind::Backpack { add: 1 },
            UpgradeCategory::ExtraDice => UpgradeKind::DiceBundle { add: 1 },
            UpgradeCategory::LowCardDamage => UpgradeKind::Tricycle {
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            UpgradeCategory::ShopItemPriceMinus => UpgradeKind::EnergyDrink { add: 5 },
            UpgradeCategory::NoRerollDamage => UpgradeKind::PerfectPottery {
                damage_multiplier: rarity_gen(rarity, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..4.0)),
            },
            UpgradeCategory::EvenOddDamage => {
                let damage_multiplier =
                    rarity_gen(rarity, (1.1..1.2, 1.2..1.4, 1.4..1.5, 1.5..1.6));
                if rng.gen_bool(0.5) {
                    UpgradeKind::SingleChopstick { damage_multiplier }
                } else {
                    UpgradeKind::PairChopsticks { damage_multiplier }
                }
            }
            UpgradeCategory::FaceNumberDamage => {
                let damage_multiplier =
                    rarity_gen(rarity, (1.1..1.2, 1.2..1.4, 1.4..1.5, 1.5..1.6));
                if rng.gen_bool(0.5) {
                    UpgradeKind::FountainPen { damage_multiplier }
                } else {
                    UpgradeKind::Brush { damage_multiplier }
                }
            }
            UpgradeCategory::ShortenStraightFlush => UpgradeKind::FourLeafClover,
            UpgradeCategory::SkipRankForStraight => UpgradeKind::Rabbit,
            UpgradeCategory::TreatSuitsAsSame => UpgradeKind::BlackWhite,
            UpgradeCategory::RerollDamage => UpgradeKind::BrokenPottery {
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
                    crate::game_state::upgrade::generate_treasure_upgrade(gs)
                } else {
                    let kind = category.generate_upgrade_kind(rarity);
                    let value = thread_rng().gen_range(0.0..=1.0);
                    Upgrade {
                        kind,
                        value: value.into(),
                    }
                };
                gs.upgrade_state.upgrade(upgrade);
            });
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(memoized_text((), |mut builder| {
                        builder.headline().text("Add upgrade").render_left_top()
                    }));
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    let info_text = format!(
                        "Stage {}: Expected - {:?} {}",
                        game_state.stage,
                        expected_rarity,
                        expected_category.display_name()
                    );
                    ctx.add(memoized_text(&info_text, |mut builder| {
                        builder
                            .paragraph()
                            .color(palette::ON_SURFACE_VARIANT)
                            .text(&info_text)
                            .render_left_top()
                    }));
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    table::horizontal([
                        table::ratio(1, |wh, ctx| {
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
                                                    ctx.add(memoized_text(
                                                        (
                                                            &selected_category,
                                                            &text_color,
                                                            &wh.height,
                                                        ),
                                                        |mut builder| {
                                                            builder
                                                                .paragraph()
                                                                .color(text_color)
                                                                .text(
                                                                    selected_category
                                                                        .display_name()
                                                                        .to_string(),
                                                                )
                                                                .render_left_center(wh.height)
                                                        },
                                                    ));
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
                                                    ctx.add(memoized_text(
                                                        (selected_rarity, &text_color, &wh.height),
                                                        |mut builder| {
                                                            builder
                                                                .color(text_color)
                                                                .text(format!(
                                                                    "{:?}",
                                                                    *selected_rarity
                                                                ))
                                                                .render_left_center(wh.height)
                                                        },
                                                    ));
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
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_category_idx.set(idx);
                                                    set_dropdown.set(0);
                                                },
                                                &|wh, text_color, ctx| {
                                                    ctx.add(memoized_text(
                                                        (&category, &text_color, &wh.height),
                                                        |mut builder| {
                                                            builder
                                                                .paragraph()
                                                                .color(text_color)
                                                                .text(
                                                                    category
                                                                        .display_name()
                                                                        .to_string(),
                                                                )
                                                                .render_left_center(wh.height)
                                                        },
                                                    ));
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
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_rarity.set(rarity);
                                                    set_dropdown.set(0);
                                                },
                                                &|wh, text_color, ctx| {
                                                    ctx.add(memoized_text(
                                                        (&rarity, &text_color, &wh.height),
                                                        |mut builder| {
                                                            builder
                                                                .color(text_color)
                                                                .text(format!("{:?}", rarity))
                                                                .render_left_center(wh.height)
                                                        },
                                                    ));
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
                                ctx.add(memoized_text((&text_color, &wh), |mut builder| {
                                    builder
                                        .color(text_color)
                                        .text("업그레이드 획득")
                                        .render_center(wh)
                                }));
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

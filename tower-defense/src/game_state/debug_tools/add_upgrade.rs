use crate::game_state::upgrade::Upgrade;
use crate::game_state::{mutate_game_state, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::palette;
use crate::theme::typography::memoized_text;
use namui::*;
use namui_prebuilt::table;
use rand::{Rng, thread_rng};

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

use crate::game_state::upgrade::UpgradeDiscriminants;
use strum::IntoEnumIterator;

const EXPECTED_UPGRADES_BY_STAGE: [(Rarity, usize); 50] = [
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Epic, 0),
    (Rarity::Common, 0),
    (Rarity::Common, 0),
    (Rarity::Rare, 0),
    (Rarity::Rare, 0),
    (Rarity::Epic, 0),
    (Rarity::Rare, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Legendary, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Legendary, 0),
    (Rarity::Rare, 0),
    (Rarity::Legendary, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Rare, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
    (Rarity::Rare, 0),
    (Rarity::Rare, 0),
    (Rarity::Common, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Epic, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
    (Rarity::Rare, 0),
    (Rarity::Legendary, 0),
    (Rarity::Epic, 0),
];

pub fn get_expected_upgrade_for_stage(stage: usize) -> (Rarity, UpgradeCategory) {
    let idx = if stage == 0 || stage > 50 {
        0
    } else {
        EXPECTED_UPGRADES_BY_STAGE[stage - 1].1
    };

    let category = if idx == RANDOM_SELECTION_IDX {
        UpgradeCategory::Random
    } else {
        UpgradeCategory::Kind(
            UpgradeDiscriminants::iter()
                .nth(idx - 1)
                .expect("Expected upgrade category index out of range"),
        )
    };

    let rarity = if stage == 0 || stage > 50 {
        Rarity::Common
    } else {
        EXPECTED_UPGRADES_BY_STAGE[stage - 1].0
    };

    (rarity, category)
}

const RANDOM_SELECTION_IDX: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeCategory {
    Random,
    Kind(UpgradeDiscriminants),
}

impl UpgradeCategory {
    pub fn selection_idx(self) -> usize {
        match self {
            UpgradeCategory::Random => RANDOM_SELECTION_IDX,
            UpgradeCategory::Kind(disc) => UpgradeDiscriminants::iter()
                .enumerate()
                .find(|(_, current)| *current == disc)
                .map(|(idx, _)| idx + 1)
                .unwrap_or(RANDOM_SELECTION_IDX),
        }
    }

    pub fn label(self) -> String {
        match self {
            UpgradeCategory::Random => "Random".to_string(),
            UpgradeCategory::Kind(disc) => disc.as_ref().to_string(),
        }
    }

    pub fn generate_upgrade_kind(self, rarity: Rarity) -> Upgrade {
        match self {
            UpgradeCategory::Random => {
                panic!("Random category cannot generate upgrade kind directly")
            }
            UpgradeCategory::Kind(disc) => {
                let mut rng = thread_rng();
                generate_mock_upgrade(disc, rarity, &mut rng)
            }
        }
    }
}

fn selection_label(selection_idx: usize) -> String {
    if selection_idx == RANDOM_SELECTION_IDX {
        "Random".to_string()
    } else {
        UpgradeDiscriminants::iter()
            .nth(selection_idx - 1)
            .map(|d| d.as_ref().to_string())
            .unwrap_or_else(|| "Random".to_string())
    }
}

fn rarity_gen(
    rarity: Rarity,
    rng: &mut impl Rng,
    ranges: (
        std::ops::Range<f32>,
        std::ops::Range<f32>,
        std::ops::Range<f32>,
        std::ops::Range<f32>,
    ),
) -> f32 {
    rng.gen_range(match rarity {
        Rarity::Common => ranges.0,
        Rarity::Rare => ranges.1,
        Rarity::Epic => ranges.2,
        Rarity::Legendary => ranges.3,
    })
}

fn generate_mock_upgrade(
    disc: UpgradeDiscriminants,
    rarity: Rarity,
    rng: &mut rand::rngs::ThreadRng,
) -> Upgrade {
    use crate::game_state::upgrade::*;
    let damage_multiplier = rarity_gen(rarity, rng, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5));
    match disc {
        UpgradeDiscriminants::Cat => crate::game_state::upgrade::CatUpgrade::into_upgrade(1),
        UpgradeDiscriminants::Staff => crate::game_state::upgrade::StaffUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::LongSword => crate::game_state::upgrade::LongSwordUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::Mace => crate::game_state::upgrade::MaceUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::ClubSword => crate::game_state::upgrade::ClubSwordUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::Backpack => crate::game_state::upgrade::BackpackUpgrade::into_upgrade(1),
        UpgradeDiscriminants::DiceBundle => crate::game_state::upgrade::DiceBundleUpgrade::into_upgrade(1),
        UpgradeDiscriminants::Tricycle => crate::game_state::upgrade::TricycleUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::EnergyDrink => crate::game_state::upgrade::EnergyDrinkUpgrade::into_upgrade(5),
        UpgradeDiscriminants::PerfectPottery => crate::game_state::upgrade::PerfectPotteryUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::SingleChopstick => crate::game_state::upgrade::SingleChopstickUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::PairChopsticks => crate::game_state::upgrade::PairChopsticksUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::FountainPen => crate::game_state::upgrade::FountainPenUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::Brush => crate::game_state::upgrade::BrushUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::FourLeafClover => crate::game_state::upgrade::FourLeafCloverUpgrade::into_upgrade(),
        UpgradeDiscriminants::Rabbit => crate::game_state::upgrade::RabbitUpgrade::into_upgrade(),
        UpgradeDiscriminants::BlackWhite => crate::game_state::upgrade::BlackWhiteUpgrade::into_upgrade(),
        UpgradeDiscriminants::Trophy => crate::game_state::upgrade::TrophyUpgrade::into_upgrade(),
        UpgradeDiscriminants::Crock => crate::game_state::upgrade::CrockUpgrade::into_upgrade(),
        UpgradeDiscriminants::DemolitionHammer => crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::Metronome => crate::game_state::upgrade::MetronomeUpgrade::into_upgrade(),
        UpgradeDiscriminants::Tape => crate::game_state::upgrade::TapeUpgrade::into_upgrade(0),
        UpgradeDiscriminants::NameTag => crate::game_state::upgrade::NameTagUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::ShoppingBag => crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::Resolution => crate::game_state::upgrade::ResolutionUpgrade::into_upgrade(damage_multiplier),
        UpgradeDiscriminants::Mirror => crate::game_state::upgrade::MirrorUpgrade::into_upgrade(),
        UpgradeDiscriminants::IceCream => crate::game_state::upgrade::IceCreamUpgrade::into_upgrade(damage_multiplier, 5),
        UpgradeDiscriminants::Spanner => crate::game_state::upgrade::SpannerUpgrade::into_upgrade(),
        UpgradeDiscriminants::Pea => crate::game_state::upgrade::PeaUpgrade::into_upgrade(),
        UpgradeDiscriminants::SlotMachine => crate::game_state::upgrade::SlotMachineUpgrade::into_upgrade(10),
        UpgradeDiscriminants::PiggyBank => crate::game_state::upgrade::PiggyBankUpgrade::into_upgrade(),
        UpgradeDiscriminants::Camera => crate::game_state::upgrade::CameraUpgrade::into_upgrade(),
        UpgradeDiscriminants::GiftBox => crate::game_state::upgrade::GiftBoxUpgrade::into_upgrade(),
        UpgradeDiscriminants::Fang => crate::game_state::upgrade::FangUpgrade::into_upgrade(),
        UpgradeDiscriminants::Popcorn => crate::game_state::upgrade::PopcornUpgrade::into_upgrade(damage_multiplier, 5, 5),
        UpgradeDiscriminants::MembershipCard => crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade(),
        UpgradeDiscriminants::Eraser => crate::game_state::upgrade::EraserUpgrade::into_upgrade(1),
        UpgradeDiscriminants::BrokenPottery => crate::game_state::upgrade::BrokenPotteryUpgrade::into_upgrade(damage_multiplier),
    }
}

pub struct AddUpgradeTool {
    pub width: Px,
}

impl Component for AddUpgradeTool {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let (expected_rarity, expected_category) = get_expected_upgrade_for_stage(game_state.stage);
        let expected_selection_idx = expected_category.selection_idx();

        // Using index instead of enum for State compatibility
        let (selected_category_idx, set_selected_category_idx) =
            ctx.state(|| expected_selection_idx);
        let (selected_rarity, set_selected_rarity) = ctx.state(|| expected_rarity);
        // 0 = none, 1 = Category, 2 = Rarity
        let (dropdown, set_dropdown) = ctx.state(|| 0u8);

        let selected_selection_idx = *selected_category_idx;

        let add_upgrade = || {
            let selection_idx = selected_selection_idx;
            let rarity = *selected_rarity;
            if selection_idx == RANDOM_SELECTION_IDX {
                mutate_game_state(move |gs| {
                    let upgrade = crate::game_state::upgrade::generate_treasure_upgrade(gs);
                    gs.upgrade_state.upgrade(upgrade);
                });
            } else {
                let mut rng = thread_rng();
                let disc = UpgradeDiscriminants::iter().nth(selection_idx - 1).unwrap();
                let upgrade = generate_mock_upgrade(disc, rarity, &mut rng);
                mutate_game_state(move |gs| {
                    gs.upgrade_state.upgrade(upgrade);
                });
            }
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
                        expected_category.label()
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
                                                            &selected_selection_idx,
                                                            &text_color,
                                                            &wh.height,
                                                        ),
                                                        |mut builder| {
                                                            builder
                                                                .paragraph()
                                                                .color(text_color)
                                                                .text(selection_label(
                                                                    selected_selection_idx,
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
                            (0..=UpgradeDiscriminants::iter().count())
                                .map(|idx| {
                                    let label = selection_label(idx);

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
                                                        (&idx, &text_color, &wh.height),
                                                        |mut builder| {
                                                            builder
                                                                .paragraph()
                                                                .color(text_color)
                                                                .text(&label)
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

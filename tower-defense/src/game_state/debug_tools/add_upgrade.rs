use crate::game_state::upgrade::{Upgrade, UpgradeKind};
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

use crate::game_state::upgrade::UpgradeKindDiscriminants;
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
            UpgradeKindDiscriminants::iter()
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
    Kind(UpgradeKindDiscriminants),
}

impl UpgradeCategory {
    pub fn selection_idx(self) -> usize {
        match self {
            UpgradeCategory::Random => RANDOM_SELECTION_IDX,
            UpgradeCategory::Kind(disc) => UpgradeKindDiscriminants::iter()
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

    pub fn generate_upgrade_kind(self, rarity: Rarity) -> UpgradeKind {
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
        UpgradeKindDiscriminants::iter()
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
    disc: UpgradeKindDiscriminants,
    rarity: Rarity,
    rng: &mut rand::rngs::ThreadRng,
) -> UpgradeKind {
    use crate::game_state::upgrade::*;
    let damage_multiplier = rarity_gen(rarity, rng, (1.2..1.5, 1.3..1.75, 1.5..2.5, 2.0..3.5));
    match disc {
        UpgradeKindDiscriminants::Cat => UpgradeKind::Cat(CatUpgrade { add: 1 }),
        UpgradeKindDiscriminants::Staff => UpgradeKind::Staff(StaffUpgrade { damage_multiplier }),
        UpgradeKindDiscriminants::LongSword => {
            UpgradeKind::LongSword(LongSwordUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::Mace => UpgradeKind::Mace(MaceUpgrade { damage_multiplier }),
        UpgradeKindDiscriminants::ClubSword => {
            UpgradeKind::ClubSword(ClubSwordUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::Backpack => UpgradeKind::Backpack(BackpackUpgrade { add: 1 }),
        UpgradeKindDiscriminants::DiceBundle => {
            UpgradeKind::DiceBundle(DiceBundleUpgrade { add: 1 })
        }
        UpgradeKindDiscriminants::Tricycle => {
            UpgradeKind::Tricycle(TricycleUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::EnergyDrink => {
            UpgradeKind::EnergyDrink(EnergyDrinkUpgrade { add: 5 })
        }
        UpgradeKindDiscriminants::PerfectPottery => {
            UpgradeKind::PerfectPottery(PerfectPotteryUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::SingleChopstick => {
            UpgradeKind::SingleChopstick(SingleChopstickUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::PairChopsticks => {
            UpgradeKind::PairChopsticks(PairChopsticksUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::FountainPen => {
            UpgradeKind::FountainPen(FountainPenUpgrade { damage_multiplier })
        }
        UpgradeKindDiscriminants::Brush => UpgradeKind::Brush(BrushUpgrade { damage_multiplier }),
        UpgradeKindDiscriminants::FourLeafClover => {
            UpgradeKind::FourLeafClover(FourLeafCloverUpgrade)
        }
        UpgradeKindDiscriminants::Rabbit => UpgradeKind::Rabbit(RabbitUpgrade),
        UpgradeKindDiscriminants::BlackWhite => UpgradeKind::BlackWhite(BlackWhiteUpgrade),
        UpgradeKindDiscriminants::Trophy => UpgradeKind::Trophy(TrophyUpgrade {
            perfect_clear_stacks: 0,
        }),
        UpgradeKindDiscriminants::Crock => UpgradeKind::Crock(CrockUpgrade),
        UpgradeKindDiscriminants::DemolitionHammer => {
            UpgradeKind::DemolitionHammer(DemolitionHammerUpgrade {
                damage_multiplier,
                removed_tower_count: 0,
            })
        }
        UpgradeKindDiscriminants::Metronome => {
            UpgradeKind::Metronome(MetronomeUpgrade { start_stage: None })
        }
        UpgradeKindDiscriminants::Tape => UpgradeKind::Tape(TapeUpgrade { acquired_stage: 0 }),
        UpgradeKindDiscriminants::NameTag => UpgradeKind::NameTag(NameTagUpgrade {
            damage_multiplier,
            pending: false,
        }),
        UpgradeKindDiscriminants::ShoppingBag => UpgradeKind::ShoppingBag(ShoppingBagUpgrade {
            damage_multiplier,
            stacks: 0,
        }),
        UpgradeKindDiscriminants::Resolution => UpgradeKind::Resolution(ResolutionUpgrade {
            damage_multiplier_per_reroll: damage_multiplier,
            pending: false,
        }),
        UpgradeKindDiscriminants::Mirror => UpgradeKind::Mirror(MirrorUpgrade { pending: false }),
        UpgradeKindDiscriminants::IceCream => UpgradeKind::IceCream(IceCreamUpgrade {
            damage_multiplier,
            waves_remaining: 5,
        }),
        UpgradeKindDiscriminants::Spanner => UpgradeKind::Spanner(SpannerUpgrade),
        UpgradeKindDiscriminants::Pea => UpgradeKind::Pea(PeaUpgrade),
        UpgradeKindDiscriminants::SlotMachine => UpgradeKind::SlotMachine(SlotMachineUpgrade {
            next_round_dice: 10,
        }),
        UpgradeKindDiscriminants::PiggyBank => UpgradeKind::PiggyBank(PiggyBankUpgrade),
        UpgradeKindDiscriminants::Camera => UpgradeKind::Camera(CameraUpgrade),
        UpgradeKindDiscriminants::GiftBox => UpgradeKind::GiftBox(GiftBoxUpgrade),
        UpgradeKindDiscriminants::Fang => UpgradeKind::Fang(FangUpgrade),
        UpgradeKindDiscriminants::Popcorn => UpgradeKind::Popcorn(PopcornUpgrade {
            max_multiplier: damage_multiplier,
            duration: 5,
            waves_remaining: 5,
        }),
        UpgradeKindDiscriminants::MembershipCard => {
            UpgradeKind::MembershipCard(MembershipCardUpgrade {
                pending_free_shop: true,
            })
        }
        UpgradeKindDiscriminants::Eraser => UpgradeKind::Eraser(EraserUpgrade { add: 1 }),
        UpgradeKindDiscriminants::BrokenPottery => {
            UpgradeKind::BrokenPottery(BrokenPotteryUpgrade { damage_multiplier })
        }
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
            mutate_game_state(move |gs| {
                let upgrade = if selection_idx == RANDOM_SELECTION_IDX {
                    crate::game_state::upgrade::generate_treasure_upgrade(gs)
                } else {
                    let mut rng = thread_rng();
                    let disc = UpgradeKindDiscriminants::iter()
                        .nth(selection_idx - 1)
                        .unwrap();
                    let kind = generate_mock_upgrade(disc, rarity, &mut rng);
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
                            (0..=UpgradeKindDiscriminants::iter().count())
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

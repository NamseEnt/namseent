use crate::{
    game_state::{
        upgrade::{
            LOW_CARD_COUNT, TowerSelectUpgradeTarget, TowerUpgradeState, TowerUpgradeTarget,
            UpgradeState,
        },
        use_game_state,
    },
    l10n::upgrade_board::UpgradeBoardText,
    palette,
    theme::typography::{FontSize, TextAlign, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{
    list_view, simple_rect,
    table::{self, horizontal},
};

const SCROLL_BAR_WIDTH: Px = px(4.0);
const TITLE_HEIGHT: Px = px(36.0);
const PADDING: Px = px(4.0);
const UPGRADE_BOARD_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const ITEM_HEIGHT: Px = px(48.0);

pub struct UpgradeBoardModal {
    pub screen_wh: Wh<Px>,
}
impl Component for UpgradeBoardModal {
    fn render(self, ctx: &namui::RenderCtx) {
        let Self { screen_wh } = self;

        ctx.compose(|ctx| {
            let offset = ((screen_wh - UPGRADE_BOARD_WH) * 0.5).to_xy();

            ctx.translate(offset).add(UpgradeBoard {});

            ctx.add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::from_u8(0, 0, 0, 128),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        event.stop_propagation();
                    }
                    Event::Wheel { event } => {
                        event.stop_propagation();
                    }
                    _ => {}
                }),
            );
        });
    }
}

pub struct UpgradeBoard {}
impl Component for UpgradeBoard {
    fn render(self, ctx: &namui::RenderCtx) {
        let game_state = use_game_state(ctx);
        let upgrade_description_texts =
            get_upgrade_description_texts(&game_state.upgrade_state, &game_state.text());

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::fixed(TITLE_HEIGHT, |wh, ctx| {
                        ctx.add(
                            headline(
                                game_state
                                    .text()
                                    .upgrade_board(UpgradeBoardText::Title)
                                    .to_string(),
                            )
                            .size(FontSize::Large)
                            .align(TextAlign::Center { wh })
                            .max_width(wh.width)
                            .build(),
                        );
                    }),
                    table::ratio(1, |wh, ctx| {
                        let item_wh = Wh {
                            width: wh.width - SCROLL_BAR_WIDTH,
                            height: ITEM_HEIGHT,
                        };
                        ctx.add(list_view::AutoListView {
                            height: wh.height,
                            scroll_bar_width: SCROLL_BAR_WIDTH,
                            item_wh,
                            items: upgrade_description_texts.into_iter().enumerate().map(
                                |(index, upgrade_description_text)| {
                                    (
                                        index,
                                        UpgradeItem {
                                            wh: item_wh,
                                            upgrade_description_text,
                                        },
                                    )
                                },
                            ),
                        });
                    }),
                ]),
            )(UPGRADE_BOARD_WH, ctx);
        });

        ctx.add(rect(RectParam {
            rect: UPGRADE_BOARD_WH.to_rect(),
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
    }
}

struct UpgradeItem {
    wh: Wh<Px>,
    upgrade_description_text: String,
}
impl Component for UpgradeItem {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade_description_text,
        } = self;

        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                ctx.compose(|ctx| {
                    horizontal([
                        table::fixed(
                            wh.height,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(simple_rect(
                                    wh,
                                    palette::OUTLINE,
                                    1.px(),
                                    palette::SURFACE_CONTAINER_HIGHEST,
                                ));
                            }),
                        ),
                        table::fixed(PADDING, |_, _| {}),
                        table::ratio(
                            1,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(
                                    paragraph(upgrade_description_text)
                                        .size(FontSize::Medium)
                                        .align(TextAlign::LeftTop)
                                        .max_width(wh.width)
                                        .build(),
                                );
                            }),
                        ),
                    ])(wh, ctx);
                });
                ctx.add(simple_rect(
                    wh,
                    palette::OUTLINE,
                    1.px(),
                    palette::SURFACE_CONTAINER_HIGH,
                ));
            })(wh, ctx);
        });
    }
}

fn get_upgrade_description_texts(
    state: &UpgradeState,
    text: &crate::l10n::TextManager,
) -> Vec<String> {
    let mut texts = vec![];
    if state.gold_earn_plus != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::GoldEarnPlus { 
                amount: state.gold_earn_plus 
            }),
        );
    }
    if state.shop_slot_expand != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::ShopSlotExpand { 
                amount: state.shop_slot_expand 
            }),
        );
    }
    if state.quest_slot_expand != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::QuestSlotExpand { 
                amount: state.quest_slot_expand 
            }),
        );
    }
    if state.quest_board_slot_expand != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::QuestBoardSlotExpand { 
                amount: state.quest_board_slot_expand 
            }),
        );
    }
    if state.reroll_chance_plus != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::RerollChancePlus { 
                amount: state.reroll_chance_plus 
            }),
        );
    }
    if state.shop_item_price_minus != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::ShopItemPriceMinus { 
                amount: state.shop_item_price_minus 
            }),
        );
    }
    if state.shop_refresh_chance_plus != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::ShopRefreshChancePlus { 
                amount: state.shop_refresh_chance_plus 
            }),
        );
    }
    if state.quest_board_refresh_chance_plus != 0 {
        texts.push(
            text.upgrade_board(UpgradeBoardText::QuestBoardRefreshChancePlus { 
                amount: state.quest_board_refresh_chance_plus 
            }),
        );
    }
    if state.shorten_straight_flush_to_4_cards {
        texts.push(
            text.upgrade_board(UpgradeBoardText::ShortenStraightFlushTo4Cards),
        );
    }
    if state.skip_rank_for_straight {
        texts.push(
            text.upgrade_board(UpgradeBoardText::SkipRankForStraight),
        );
    }
    if state.treat_suits_as_same {
        texts.push(
            text.upgrade_board(UpgradeBoardText::TreatSuitsAsSame),
        );
    }

    for (target, tower_upgrade_state) in &state.tower_select_upgrade_states {
        let target_prefix = match target {
            TowerSelectUpgradeTarget::LowCard => {
                text.upgrade_board(UpgradeBoardText::TowerSelectLowCard { 
                    amount: LOW_CARD_COUNT 
                })
            }
            TowerSelectUpgradeTarget::NoReroll => {
                text.upgrade_board(UpgradeBoardText::TowerSelectNoReroll)
            }
            TowerSelectUpgradeTarget::Reroll => {
                text.upgrade_board(UpgradeBoardText::TowerSelectReroll)
            }
        };
        texts.extend(tower_upgrade_state_description_texts(
            text,
            &target_prefix,
            tower_upgrade_state,
        ));
    }

    for (target, tower_upgrade_state) in &state.tower_upgrade_states {
        let target_prefix = match target {
            TowerUpgradeTarget::Rank { rank } => {
                text.upgrade_board(UpgradeBoardText::TowerUpgradeRank { 
                    name: rank.to_string() 
                })
            }
            TowerUpgradeTarget::Suit { suit } => {
                text.upgrade_board(UpgradeBoardText::TowerUpgradeSuit { 
                    name: suit.to_string() 
                })
            }
            TowerUpgradeTarget::TowerKind { tower_kind } => {
                text.upgrade_board(UpgradeBoardText::TowerUpgradeKind { 
                    name: text.tower(tower_kind.to_text()).to_string() 
                })
            }
            TowerUpgradeTarget::EvenOdd { even } => {
                let name = if *even { "짝수" } else { "홀수" };
                text.upgrade_board(UpgradeBoardText::TowerUpgradeEvenOdd { 
                    name: name.to_string() 
                })
            }
            TowerUpgradeTarget::FaceNumber { face } => {
                let name = if *face { "그림" } else { "숫자" };
                text.upgrade_board(UpgradeBoardText::TowerUpgradeFaceNumber { 
                    name: name.to_string() 
                })
            }
        };
        texts.extend(tower_upgrade_state_description_texts(
            text,
            &target_prefix,
            tower_upgrade_state,
        ));
    }

    texts
}

fn tower_upgrade_state_description_texts(
    text: &crate::l10n::TextManager,
    target_prefix: &str,
    tower_upgrade_state: &TowerUpgradeState,
) -> Vec<String> {
    let mut texts = vec![];
    if tower_upgrade_state.damage_plus != 0.0 {
        let suffix = text.upgrade_board(UpgradeBoardText::DamagePlus { 
            amount: tower_upgrade_state.damage_plus 
        });
        texts.push(format!("{target_prefix} {suffix}"));
    }
    if tower_upgrade_state.damage_multiplier != 1.0 {
        let suffix = text.upgrade_board(UpgradeBoardText::DamageMultiplier { 
            amount: tower_upgrade_state.damage_multiplier 
        });
        texts.push(format!("{target_prefix} {suffix}"));
    }
    if tower_upgrade_state.speed_plus != 0.0 {
        let suffix = text.upgrade_board(UpgradeBoardText::SpeedPlus { 
            amount: tower_upgrade_state.speed_plus 
        });
        texts.push(format!("{target_prefix} {suffix}"));
    }
    if tower_upgrade_state.speed_multiplier != 1.0 {
        let suffix = text.upgrade_board(UpgradeBoardText::SpeedMultiplier { 
            amount: tower_upgrade_state.speed_multiplier 
        });
        texts.push(format!("{target_prefix} {suffix}"));
    }
    if tower_upgrade_state.range_plus != 0.0 {
        let suffix = text.upgrade_board(UpgradeBoardText::RangePlus { 
            amount: tower_upgrade_state.range_plus 
        });
        texts.push(format!("{target_prefix} {suffix}"));
    }
    texts
}

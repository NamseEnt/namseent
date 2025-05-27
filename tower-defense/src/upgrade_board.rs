use crate::{
    game_state::{
        upgrade::{
            LOW_CARD_COUNT, TowerSelectUpgradeTarget, TowerUpgradeState, TowerUpgradeTarget,
            UpgradeState,
        },
        use_game_state,
    },
    palette,
    theme::typography::{FontSize, Headline, Paragraph, TextAlign},
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
        let upgrade_description_texts = get_upgrade_description_texts(&game_state.upgrade_state);

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::fixed(TITLE_HEIGHT, |wh, ctx| {
                        ctx.add(Headline {
                            text: "강화 정보".to_string(),
                            font_size: FontSize::Large,
                            text_align: TextAlign::Center { wh },
                            max_width: None,
                        });
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
                                ctx.add(Paragraph {
                                    text: upgrade_description_text,
                                    font_size: FontSize::Medium,
                                    text_align: TextAlign::LeftTop,
                                    max_width: Some(wh.width),
                                });
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

fn get_upgrade_description_texts(state: &UpgradeState) -> Vec<String> {
    let mut texts = vec![];

    if state.gold_earn_plus != 0 {
        texts.push(format!(
            "몬스터 처치 시 {}골드를 추가로 얻습니다",
            state.gold_earn_plus
        ));
    }
    if state.shop_slot_expand != 0 {
        texts.push(format!(
            "상점 슬롯이 {}개 증가합니다",
            state.shop_slot_expand
        ));
    }
    if state.quest_slot_expand != 0 {
        texts.push(format!(
            "퀘스트 슬롯이 {}개 증가합니다",
            state.quest_slot_expand
        ));
    }
    if state.quest_board_slot_expand != 0 {
        texts.push(format!(
            "퀘스트 게시판 슬롯이 {}개 증가합니다",
            state.quest_board_slot_expand
        ));
    }
    if state.reroll_chance_plus != 0 {
        texts.push(format!(
            "리롤 기회가 {}개 증가합니다",
            state.reroll_chance_plus
        ));
    }
    if state.shop_item_price_minus != 0 {
        texts.push(format!(
            "상점 아이템 가격이 {} 감소합니다",
            state.shop_item_price_minus
        ));
    }
    if state.shop_refresh_chance_plus != 0 {
        texts.push(format!(
            "상점 새로고침 기회가 {}개 증가합니다",
            state.shop_refresh_chance_plus
        ));
    }
    if state.quest_board_refresh_chance_plus != 0 {
        texts.push(format!(
            "퀘스트 게시판 새로고침 기회가 {}개 증가합니다",
            state.quest_board_refresh_chance_plus
        ));
    }
    if state.shorten_straight_flush_to_4_cards {
        texts.push("스트레이트와 플러시를 4장으로 줄입니다".to_string());
    }
    if state.skip_rank_for_straight {
        texts.push("스트레이트를 만들 때 랭크 하나를 건너뛸 수 있습니다".to_string());
    }
    if state.treat_suits_as_same {
        texts.push("색이 같으면 같은 문양으로 취급합니다".to_string());
    }

    for (target, tower_upgrade_state) in &state.tower_select_upgrade_states {
        let target_text = match target {
            TowerSelectUpgradeTarget::LowCard => {
                format!("카드 {LOW_CARD_COUNT}개 이하로 타워를 만들 때 타워의")
            }
            TowerSelectUpgradeTarget::NoReroll => "리롤을 하지 않고 타워를 만들 때 타워의".to_string(),
            TowerSelectUpgradeTarget::Reroll => "리롤을 할 때 마다 타워의".to_string(),
        };
        texts.extend(tower_upgrade_state_description_texts(
            &target_text,
            tower_upgrade_state,
        ));
    }

    for (target, tower_upgrade_state) in &state.tower_upgrade_states {
        let target_text = match target {
            TowerUpgradeTarget::Rank { rank } => {
                format!("랭크가 {}인 타워의", rank)
            }
            TowerUpgradeTarget::Suit { suit } => {
                format!("문양이 {}인 타워의", suit)
            }
            TowerUpgradeTarget::TowerKind { tower_kind } => {
                format!("{} 타워의", tower_kind)
            }
            TowerUpgradeTarget::EvenOdd { even } => {
                format!("{} 타워의", if *even { "짝수" } else { "홀수" })
            }
            TowerUpgradeTarget::FaceNumber { face } => {
                format!("{} 타워의", if *face { "그림" } else { "숫자" })
            }
        };
        texts.extend(tower_upgrade_state_description_texts(
            &target_text,
            tower_upgrade_state,
        ));
    }

    texts
}

fn tower_upgrade_state_description_texts(
    target_text: &str,
    tower_upgrade_state: &TowerUpgradeState,
) -> Vec<String> {
    let mut texts = vec![];
    if tower_upgrade_state.damage_plus != 0.0 {
        texts.push(format!(
            "{target_text} 공격력이 {}만큼 증가합니다",
            tower_upgrade_state.damage_plus
        ));
    }
    if tower_upgrade_state.damage_multiplier != 1.0 {
        texts.push(format!(
            "{target_text} 공격력이 {}배 증가합니다",
            tower_upgrade_state.damage_multiplier
        ));
    }
    if tower_upgrade_state.speed_plus != 0.0 {
        texts.push(format!(
            "{target_text} 공격 속도가 {}만큼 증가합니다",
            tower_upgrade_state.speed_plus
        ));
    }
    if tower_upgrade_state.speed_multiplier != 1.0 {
        texts.push(format!(
            "{target_text} 공격 속도가 {}배 증가합니다",
            tower_upgrade_state.speed_multiplier
        ));
    }
    if tower_upgrade_state.range_plus != 0.0 {
        texts.push(format!(
            "{target_text} 사정거리가 {}만큼 증가합니다",
            tower_upgrade_state.range_plus
        ));
    }
    texts
}

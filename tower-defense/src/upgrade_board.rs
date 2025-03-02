use crate::{game_state::use_game_state, palette, upgrade::UpgradeState};
use namui::*;
use namui_prebuilt::{
    list_view, simple_rect,
    table::{self, horizontal},
    typography,
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
            let offset = ((screen_wh - UPGRADE_BOARD_WH) * 0.5).as_xy();

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
                        ctx.add(typography::body::left(
                            wh.height,
                            "강화 정보",
                            palette::ON_SURFACE,
                        ));
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
                                ctx.add(typography::body::left(
                                    wh.height,
                                    upgrade_description_text,
                                    palette::ON_SURFACE,
                                ));
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
    if state.shop_slot != 0 {
        texts.push(format!("상점 슬롯이 {}개 증가합니다", state.shop_slot));
    }
    if state.quest_slot != 0 {
        texts.push(format!("퀘스트 슬롯이 {}개 증가합니다", state.quest_slot));
    }
    if state.quest_board_slot != 0 {
        texts.push(format!(
            "퀘스트 게시판 슬롯이 {}개 증가합니다",
            state.quest_board_slot
        ));
    }
    if state.reroll != 0 {
        texts.push(format!("리롤 기회가 {}개 증가합니다", state.reroll));
    }
    for (target, tower_upgrade_state) in &state.tower_upgrade_states {
        let target_text = match target {
            crate::upgrade::TowerUpgradeTarget::Rank { rank } => {
                format!("랭크가 {}인 타워의", rank)
            }
            crate::upgrade::TowerUpgradeTarget::Suit { suit } => {
                format!("문양이 {}인 타워의", suit)
            }
        };
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
    }

    texts
}

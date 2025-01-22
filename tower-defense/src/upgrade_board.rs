use crate::{palette, status::UPGRADES_ATOM, upgrade::Upgrade};
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
        let (upgrades, _set_upgrades) = ctx.atom(&UPGRADES_ATOM);

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
                            items: upgrades.iter().enumerate().map(|(index, upgrade)| {
                                (
                                    index,
                                    UpgradeItem {
                                        wh: item_wh,
                                        upgrade,
                                    },
                                )
                            }),
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

struct UpgradeItem<'a> {
    wh: Wh<Px>,
    upgrade: &'a Upgrade,
}
impl Component for UpgradeItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, upgrade } = self;

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
                                    &upgrade_description_text(upgrade),
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

fn upgrade_description_text(upgrade: &Upgrade) -> String {
    match upgrade {
        Upgrade::Tower {
            target,
            upgrade: tower_upgrade,
        } => {
            let target_text = match target {
                crate::upgrade::TowerUpgradeTarget::Rank { rank } => {
                    format!("랭크가 {}인 타워의", rank)
                }
                crate::upgrade::TowerUpgradeTarget::Suit { suit } => {
                    format!("문양이 {}인 타워의", suit)
                }
            };
            let upgrade_text = match tower_upgrade {
                crate::upgrade::TowerUpgrade::DamagePlus { damage } => {
                    format!("공격력이 {}만큼 증가합니다", damage)
                }
                crate::upgrade::TowerUpgrade::DamageMultiplier { multiplier } => {
                    format!("공격력이 {}배 증가합니다", multiplier)
                }
                crate::upgrade::TowerUpgrade::SpeedPlus { speed } => {
                    format!("공격 속도가 {}만큼 증가합니다", speed)
                }
                crate::upgrade::TowerUpgrade::SpeedMultiplier { multiplier } => {
                    format!("공격 속도가 {}배 증가합니다", multiplier)
                }
                crate::upgrade::TowerUpgrade::RangePlus { range } => {
                    format!("사정거리가 {}만큼 증가합니다", range)
                }
            };
            format!("{} {}", target_text, upgrade_text)
        }
        Upgrade::ShopSlot { extra_slot } => {
            format!("상점에 {}개의 추가슬롯을 제공합니다", extra_slot)
        }
        Upgrade::QuestSlot { extra_slot } => {
            format!("퀘스트 슬롯에 {}개의 추가슬롯을 제공합니다", extra_slot)
        }
        Upgrade::QuestBoardSlot { extra_slot } => {
            format!("퀘스트 게시판에 {}개의 추가슬롯을 제공합니다", extra_slot)
        }
        Upgrade::Reroll { extra_reroll } => {
            format!("{}개의 리롤 기회를 얻습니다", extra_reroll)
        }
    }
}

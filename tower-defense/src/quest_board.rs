use crate::{
    game_state::{mutate_game_state, quest::Quest, use_game_state},
    palette,
    theme::typography::{FontSize, Headline, Paragraph, TextAlign},
};
use namui::*;
use namui_prebuilt::{
    button::{self, TextButton},
    simple_rect,
    table::{self},
};

const PADDING: Px = px(4.0);
const QUEST_BOARD_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const QUEST_BOARD_BUTTON_WH: Wh<Px> = Wh {
    width: px(128.0),
    height: px(36.0),
};
const ACCEPTED_LABEL_HEIGHT: Px = px(24.0);

#[derive(Default, Clone)]
pub enum QuestBoardSlot {
    #[default]
    Locked,
    Quest {
        quest: Quest,
        accepted: bool,
    },
}

pub struct QuestBoardModal {
    pub screen_wh: Wh<Px>,
}
impl Component for QuestBoardModal {
    fn render(self, ctx: &namui::RenderCtx) {
        let Self { screen_wh } = self;
        let game_state = use_game_state(ctx);
        let (opened, set_opened) = ctx.state(|| true);

        let toggle_open = || {
            set_opened.mutate(|opened| *opened = !*opened);
        };

        let accept_quest = |slot_index: usize| {
            mutate_game_state(move |state| {
                let slot = &mut state.quest_board_slots[slot_index];
                let QuestBoardSlot::Quest { quest, accepted } = slot else {
                    panic!("Invalid shop slot");
                };

                assert!(state.items.len() <= state.max_shop_slot);
                assert!(!*accepted);

                state.quests.push(quest.clone());
                *accepted = true;
            });
        };

        let quest_board_slots = &game_state.quest_board_slots;
        let offset = ((screen_wh - QUEST_BOARD_WH) * 0.5).as_xy();

        ctx.compose(|ctx| {
            ctx.translate(offset).add(QuestBoardOpenButton {
                opened: *opened,
                toggle_open: &toggle_open,
            });
        });

        ctx.compose(|ctx| {
            if !*opened {
                return;
            }
            ctx.translate(offset).add(QuestBoard {
                quest_board_slots,
                accept_quest: &accept_quest,
            });
        });
    }
}

struct QuestBoardOpenButton<'a> {
    opened: bool,
    toggle_open: &'a dyn Fn(),
}
impl Component for QuestBoardOpenButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            opened,
            toggle_open,
        } = self;

        ctx.compose(|ctx| {
            ctx.translate((0.px(), -QUEST_BOARD_BUTTON_WH.height))
                .add(TextButton {
                    rect: QUEST_BOARD_BUTTON_WH.to_rect(),
                    text: format!("퀘스트 {}", if opened { "^" } else { "v" }),
                    text_color: palette::ON_SURFACE,
                    stroke_color: palette::OUTLINE,
                    stroke_width: 1.px(),
                    fill_color: palette::SURFACE_CONTAINER,
                    mouse_buttons: vec![MouseButton::Left],
                    on_mouse_up_in: |_| {
                        toggle_open();
                    },
                });
        });
    }
}

pub struct QuestBoard<'a> {
    quest_board_slots: &'a [QuestBoardSlot],
    accept_quest: &'a dyn Fn(usize),
}
impl Component for QuestBoard<'_> {
    fn render(self, ctx: &namui::RenderCtx) {
        let Self {
            quest_board_slots,
            accept_quest,
        } = self;

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::horizontal(quest_board_slots.iter().enumerate().map(
                    |(shop_slot_index, shop_slot)| {
                        table::ratio(1, move |wh, ctx| {
                            ctx.add(QuestBoardItem {
                                wh,
                                quest_board_slot: shop_slot,
                                quest_board_slot_index: shop_slot_index,
                                accept_quest,
                            });
                        })
                    },
                )),
            )(QUEST_BOARD_WH, ctx);
        });
    }
}

struct QuestBoardItem<'a> {
    wh: Wh<Px>,
    quest_board_slot: &'a QuestBoardSlot,
    quest_board_slot_index: usize,
    accept_quest: &'a dyn Fn(usize),
}
impl Component for QuestBoardItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            quest_board_slot,
            quest_board_slot_index,
            accept_quest,
        } = self;

        let accept_quest = || accept_quest(quest_board_slot_index);

        ctx.compose(|ctx| {
            table::padding(PADDING, |wh, ctx| {
                match quest_board_slot {
                    QuestBoardSlot::Locked => {
                        ctx.add(QuestBoardItemLocked { wh });
                    }
                    QuestBoardSlot::Quest { quest, accepted } => {
                        ctx.add(QuestBoardItemContent {
                            wh,
                            quest,
                            accept_quest: &accept_quest,
                            accepted: *accepted,
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

struct QuestBoardItemLocked {
    wh: Wh<Px>,
}
impl Component for QuestBoardItemLocked {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(ACCEPTED_LABEL_HEIGHT, |wh, ctx| {
                    ctx.add(Headline {
                        text: "Locked".to_string(),
                        font_size: FontSize::Medium,
                        text_align: TextAlign::Center { wh },
                        max_width: None,
                    });
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}

struct QuestBoardItemContent<'a> {
    wh: Wh<Px>,
    quest: &'a Quest,
    accept_quest: &'a dyn Fn(),
    accepted: bool,
}
impl Component for QuestBoardItemContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            quest,
            accept_quest,
            accepted,
        } = self;

        let game_state = use_game_state(ctx);

        let available = !accepted;

        ctx.compose(|ctx| {
            if !accepted {
                return;
            }
            ctx.add(QuestBoardItemSoldOut { wh });
        });

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    wh.width,
                    table::padding(PADDING, |_wh, _ctx| {
                        // TODO: Icons
                    }),
                ),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        ctx.compose(|ctx| {
                            table::padding(PADDING, |wh, ctx| {
                                table::vertical([
                                    table::fit(table::FitAlign::LeftTop, |ctx| {
                                        ctx.add(Headline {
                                            text: quest.requirement.description(&game_state),
                                            font_size: FontSize::Small,
                                            text_align: TextAlign::LeftTop,
                                            max_width: Some(wh.width),
                                        });
                                    }),
                                    table::fixed(PADDING, |_, _| {}),
                                    table::ratio(1, |wh, ctx| {
                                        ctx.add(Paragraph {
                                            text: quest.reward.description(),
                                            font_size: FontSize::Medium,
                                            text_align: TextAlign::LeftTop,
                                            max_width: Some(wh.width),
                                        });
                                    }),
                                    table::fixed(PADDING, |_, _| {}),
                                    table::fixed(48.px(), |wh, ctx| {
                                        ctx.add(button::TextButton {
                                            rect: wh.to_rect(),
                                            text: "수락",
                                            text_color: match available {
                                                true => palette::ON_PRIMARY,
                                                false => palette::ON_SURFACE,
                                            },
                                            stroke_color: palette::OUTLINE,
                                            stroke_width: 1.px(),
                                            fill_color: match available {
                                                true => palette::PRIMARY,
                                                false => palette::SURFACE_CONTAINER_HIGH,
                                            },
                                            mouse_buttons: vec![MouseButton::Left],
                                            on_mouse_up_in: |_| {
                                                if !available {
                                                    return;
                                                }
                                                accept_quest();
                                            },
                                        });
                                    }),
                                ])(wh, ctx);
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
                                    color: palette::SURFACE,
                                }),
                                round: Some(RectRound {
                                    radius: palette::ROUND,
                                }),
                            },
                        }));
                    }),
                ),
            ])(wh, ctx);
        });
    }
}

struct QuestBoardItemSoldOut {
    wh: Wh<Px>,
}
impl Component for QuestBoardItemSoldOut {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(ACCEPTED_LABEL_HEIGHT, |wh, ctx| {
                    ctx.add(Headline {
                        text: "Sold Out".to_string(),
                        font_size: FontSize::Medium,
                        text_align: TextAlign::Center { wh },
                        max_width: None,
                    });
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

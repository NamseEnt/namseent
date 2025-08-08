use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::{
    game_state::{
        MAX_INVENTORY_SLOT, mutate_game_state,
        quest::{Quest, generate_quests},
        use_game_state,
    },
    icon::{Icon, IconKind, IconSize},
    l10n::ui::TopBarText,
    palette,
    theme::typography::{FontSize, TextAlign, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{
    simple_rect,
    table::{self},
};

const PADDING: Px = px(8.0);
const QUEST_BOARD_WH: Wh<Px> = Wh {
    width: px(640.0),
    height: px(480.0),
};
const QUEST_BOARD_BUTTON_WH: Wh<Px> = Wh {
    width: px(64.0),
    height: px(48.0),
};
const QUEST_BOARD_REFRESH_BUTTON_WH: Wh<Px> = Wh {
    width: px(192.0),
    height: px(48.0),
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

                assert!(state.items.len() <= MAX_INVENTORY_SLOT);
                assert!(!*accepted);

                state.quest_states.push(quest.to_state());
                *accepted = true;
            });
        };

        let quest_board_slots = &game_state.quest_board_slots;
        let offset = ((screen_wh - QUEST_BOARD_WH) * 0.5).to_xy();

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
            ctx.translate((0.px(), -QUEST_BOARD_BUTTON_WH.height)).add(
                Button::new(
                    QUEST_BOARD_BUTTON_WH,
                    &|| {
                        toggle_open();
                    },
                    &|wh, _text_color, ctx| {
                        ctx.add(Icon::new(IconKind::Quest).size(IconSize::Large).wh(wh));
                    },
                )
                .variant(ButtonVariant::Fab)
                .color(match opened {
                    true => ButtonColor::Primary,
                    false => ButtonColor::Secondary,
                }),
            );
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

        let game_state = use_game_state(ctx);
        let disabled = game_state.left_quest_board_refresh_chance == 0;

        let refresh_quest_board = || {
            mutate_game_state(|game_state| {
                game_state.left_quest_board_refresh_chance -= 1;
                let quests = generate_quests(game_state, game_state.max_quest_board_slot());
                for (slot, quest) in game_state
                    .quest_board_slots
                    .iter_mut()
                    .zip(quests.into_iter())
                {
                    if let QuestBoardSlot::Quest {
                        quest: quest_of_slot,
                        accepted,
                    } = slot
                    {
                        if *accepted {
                            continue;
                        }
                        *quest_of_slot = quest;
                    }
                }
            });
        };

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::ratio(
                        1,
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
                    ),
                    table::fixed(
                        QUEST_BOARD_REFRESH_BUTTON_WH.height,
                        table::horizontal([
                            table::ratio(1, |_, _| {}),
                            table::fixed(QUEST_BOARD_REFRESH_BUTTON_WH.width, |wh, ctx| {
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &|| {
                                            refresh_quest_board();
                                        },
                                        &|wh, color, ctx| {
                                            ctx.add(
                                                headline(format!(
                                                    "{}-{}",
                                                    Icon::new(IconKind::Refresh)
                                                        .size(IconSize::Large)
                                                        .wh(Wh::single(wh.height))
                                                        .as_tag(),
                                                    game_state.left_quest_board_refresh_chance
                                                ))
                                                .color(color)
                                                .align(TextAlign::Center { wh })
                                                .build_rich(),
                                            );
                                        },
                                    )
                                    .variant(ButtonVariant::Fab)
                                    .disabled(disabled),
                                );
                            }),
                            table::ratio(1, |_, _| {}),
                        ]),
                    ),
                ]),
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
        let _game_state = use_game_state(ctx);
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
                table::fixed(36.px(), |wh, ctx| {
                    ctx.add(Icon::new(IconKind::Lock).size(IconSize::Large).wh(wh));
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

        let is_quest_slots_full = game_state.quest_states.len() >= game_state.max_quest_slot();
        let disabled = accepted || is_quest_slots_full;

        ctx.compose(|ctx| {
            if !accepted {
                return;
            }
            ctx.add(QuestBoardItemAccepted { wh });
        });

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    wh.width,
                    table::padding(PADDING, |wh, ctx| {
                        ctx.add(quest.requirement.icon(wh));
                    }),
                ),
                table::ratio(
                    1,
                    table::padding(PADDING, |wh, ctx| {
                        table::vertical([
                            table::fit(table::FitAlign::LeftTop, |compose_ctx| {
                                compose_ctx.add(
                                    headline(quest.requirement.description(&game_state))
                                        .size(FontSize::Small)
                                        .align(TextAlign::LeftTop)
                                        .max_width(wh.width)
                                        .build_rich(),
                                );
                            }),
                            table::fixed(PADDING, |_, _| {}),
                            table::ratio(1, |wh, compose_ctx| {
                                compose_ctx.add(
                                    paragraph(quest.reward.description(&game_state))
                                        .size(FontSize::Medium)
                                        .align(TextAlign::LeftTop)
                                        .max_width(wh.width)
                                        .build_rich(),
                                );
                            }),
                            table::fixed(PADDING, |_, _| {}),
                            table::fixed(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &|| {
                                            accept_quest();
                                        },
                                        &|wh, _text_color, ctx| {
                                            ctx.add(
                                                Icon::new(IconKind::Accept)
                                                    .size(IconSize::Large)
                                                    .wh(wh),
                                            );
                                        },
                                    )
                                    .color(crate::theme::button::ButtonColor::Primary)
                                    .disabled(disabled),
                                );
                            }),
                        ])(wh, ctx);
                    }),
                ),
            ])(wh, ctx);
        });
    }
}

struct QuestBoardItemAccepted {
    wh: Wh<Px>,
}
impl Component for QuestBoardItemAccepted {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(ACCEPTED_LABEL_HEIGHT, |wh, ctx| {
                    ctx.add(
                        headline(game_state.text().ui(TopBarText::Accepted).to_string())
                            .size(FontSize::Medium)
                            .align(TextAlign::Center { wh })
                            .build(),
                    );
                    ctx.add(simple_rect(
                        wh,
                        Color::TRANSPARENT,
                        0.px(),
                        palette::OUTLINE,
                    ));
                }),
                table::ratio(1, |_, _| {}),
            ])(wh, ctx);
        });
    }
}

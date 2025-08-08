use crate::theme::button::{Button, ButtonVariant};
use crate::{
    game_state::{quest::cancel_quest, use_game_state},
    icon::{Icon, IconKind, IconSize},
    palette,
    theme::typography::{FontSize, HEADLINE_FONT_SIZE_LARGE, TextAlign, headline, paragraph},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, table};

const QUESTS_WIDTH: Px = px(240.);
const PADDING: Px = px(8.);
const TITLE_HEIGHT: Px = px(36.);

pub struct Quests {
    pub screen_wh: Wh<Px>,
}
impl Component for Quests {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::horizontal([table::fixed_no_clip(
                QUESTS_WIDTH,
                table::padding(
                    PADDING,
                    table::vertical([
                        table::fixed(TITLE_HEIGHT, |wh, ctx| {
                            ctx.add(
                                Icon::new(IconKind::Quest)
                                    .size(IconSize::Medium)
                                    .wh(Wh {
                                        width: 32.px(),
                                        height: wh.height
                                    })
                            );
                            let text = format!(
                                "{}/{}",
                                game_state.quest_states.len(),
                                game_state.max_quest_slot()
                            );
                            ctx.add(
                                headline(text)
                                .size(FontSize::Medium)
                                .align(TextAlign::Center { wh })
                                .max_width(wh.width)
                                .build(),
                            );
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
                        }),
                        table::fixed_no_clip(PADDING, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            let content_width = wh.width - PADDING * 2.;

                            ctx.clip(Path::new().add_rect(wh.to_rect()), ClipOp::Intersect)
                                .add(AutoScrollViewWithCtx {
                                    wh,
                                    scroll_bar_width: PADDING,
                                    content: |mut ctx| {
                                        for (quest_index, quest) in
                                            game_state.quest_states.iter().enumerate()
                                        {
                                            let tracking_description = quest.tracking.description(&game_state);
                                            let reward_description = quest.reward.description(&game_state);
                                            let content = ctx.ghost_compose(
                                                format!("QuestItemContent {quest_index}"),
                                                |ctx| {
                                                    table::vertical([
                                                        table::fixed(
                                                            HEADLINE_FONT_SIZE_LARGE.into_px(),
                                                            table::horizontal([
                                                                table::fixed(
                                                                    HEADLINE_FONT_SIZE_LARGE
                                                                        .into_px(),
                                                                    |wh, ctx| {
                                                                        ctx.add(quest.tracking.to_requirement().icon(wh));
                                                                    },
                                                                ),
                                                                table::ratio(1, |_, _| {}),
                                                                table::fixed(
                                                                    HEADLINE_FONT_SIZE_LARGE
                                                                        .into_px(),
                                                                    |wh, ctx| {
                                                                        ctx.add(Button::new(
                                                                            wh,
                                                                            &move || {
                                                                                cancel_quest(
                                                                                    quest_index,
                                                                                );
                                                                            },
                                                                            &|wh, _text_color, ctx| {
                                                                                ctx.add(
                                                                                    Icon::new(
                                                                                        IconKind::Reject,
                                                                                    )
                                                                                    .wh(wh),
                                                                                );
                                                                            }
                                                                        ).variant(ButtonVariant::Text));
                                                                    },
                                                                ),
                                                            ]),
                                                        ),
                                                        table::fixed(PADDING * 2.0, |_, _| {}),
                                                        table::fit(
                                                            table::FitAlign::LeftTop,
                                                            move |ctx| {
                                                                ctx.add(
                                                                    headline(tracking_description)
                                                                        .size(FontSize::Small)
                                                                        .align(TextAlign::LeftTop)
                                                                        .max_width(content_width)
                                                                        .build_rich(),
                                                                );
                                                            },
                                                        ),
                                                        table::fixed(PADDING, |_, _| {}),
                                                        table::fit(
                                                            table::FitAlign::LeftTop,
                                                            move |ctx| {
                                                                ctx.add(
                                                                    paragraph(reward_description)
                                                                        .size(FontSize::Medium)
                                                                        .align(TextAlign::LeftTop)
                                                                        .max_width(content_width)
                                                                        .build_rich(),
                                                                );
                                                            },
                                                        ),
                                                    ])(
                                                        Wh::new(content_width, f32::MAX.px()), ctx
                                                    );
                                                },
                                            );

                                            let Some(content_wh) =
                                                bounding_box(&content).map(|rect| rect.wh())
                                            else {
                                                return;
                                            };
                                            let container_wh =
                                                content_wh + Wh::single(PADDING * 2.);

                                            ctx.translate(Xy::single(PADDING)).add(content);

                                            ctx.add(rect(RectParam {
                                                rect: container_wh.to_rect(),
                                                style: RectStyle {
                                                    stroke: Some(RectStroke {
                                                        color: palette::OUTLINE,
                                                        width: 1.px(),
                                                        border_position: BorderPosition::Inside,
                                                    }),
                                                    fill: None,
                                                    round: Some(RectRound {
                                                        radius: palette::ROUND,
                                                    }),
                                                },
                                            }));

                                            ctx.add(rect(RectParam {
                                                rect: Wh::new(
                                                    container_wh.width,
                                                    HEADLINE_FONT_SIZE_LARGE.into_px()
                                                        + PADDING * 2.0,
                                                )
                                                .to_rect(),
                                                style: RectStyle {
                                                    stroke: None,
                                                    fill: Some(RectFill {
                                                        color: palette::SURFACE_CONTAINER,
                                                    }),
                                                    round: Some(RectRound {
                                                        radius: palette::ROUND,
                                                    }),
                                                },
                                            }));

                                            ctx.add(rect(RectParam {
                                                rect: container_wh.to_rect(),
                                                style: RectStyle {
                                                    stroke: None,
                                                    fill: Some(RectFill {
                                                        color: palette::SURFACE,
                                                    }),
                                                    round: Some(RectRound {
                                                        radius: palette::ROUND,
                                                    }),
                                                },
                                            }));

                                            ctx = ctx
                                                .translate((0.px(), container_wh.height + PADDING));
                                        }
                                    },
                                });
                        }),
                    ]),
                ),
            )])(screen_wh, ctx);
        });
    }
}

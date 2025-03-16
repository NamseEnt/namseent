use crate::{
    game_state::{
        GameState,
        quest::{Quest, cancel_quest},
        use_game_state,
    },
    palette,
    theme::typography::{FontSize, HEADLINE_FONT_SIZE_LARGE, Headline, Paragraph, TextAlign},
};
use namui::*;
use namui_prebuilt::{
    button::TextButton,
    table::{self},
    vh_list_view::AutoVHListView,
};

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
                            ctx.add(Headline {
                                text: format!(
                                    "퀘스트 {}/{}",
                                    game_state.quests.len(),
                                    game_state.max_quest_slot()
                                ),
                                font_size: FontSize::Medium,
                                text_align: TextAlign::Center { wh },
                                max_width: wh.width.into(),
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
                            let quest_items =
                                render_quest_items(&ctx, wh.width, &game_state.quests, &game_state);
                            ctx.add(AutoVHListView {
                                wh,
                                scroll_bar_width: PADDING,
                                items: quest_items,
                                item_height: Box::new(|quest_item| {
                                    namui::bounding_box(quest_item)
                                        .map(|rect| rect.height())
                                        .unwrap_or(0.px())
                                        + PADDING
                                }),
                                item_render: Box::new(|_wh, quest_item, ctx: ComposeCtx| {
                                    ctx.add(quest_item.clone());
                                }),
                            });
                        }),
                    ]),
                ),
            )])(screen_wh, ctx);
        });
    }
}

fn render_quest_items<'a>(
    ctx: &ComposeCtx,
    width: Px,
    quests: &'a [Quest],
    game_state: &'a GameState,
) -> Vec<RenderingTree> {
    let content_width = width - PADDING * 2.;

    let mut quest_items = vec![];
    for (quest_index, quest) in quests.iter().enumerate() {
        let quest_item = ctx.ghost_compose(format!("QuestItem {quest_index}"), |ctx| {
            let content = ctx.ghost_compose(format!("QuestItemContent {quest_index}"), |ctx| {
                table::vertical([
                    table::fixed(
                        HEADLINE_FONT_SIZE_LARGE.into_px(),
                        table::horizontal([
                            table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px(), |_, _| {
                                // TODO: Icons
                            }),
                            table::ratio(1, |_, _| {}),
                            table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px(), |wh, ctx| {
                                ctx.add(TextButton {
                                    rect: wh.to_rect(),
                                    text: "X",
                                    text_color: palette::ON_SURFACE,
                                    stroke_color: palette::OUTLINE,
                                    stroke_width: 1.px(),
                                    fill_color: palette::SURFACE,
                                    mouse_buttons: vec![MouseButton::Left],
                                    on_mouse_up_in: |_| {
                                        cancel_quest(quest_index);
                                    },
                                });
                            }),
                        ]),
                    ),
                    table::fixed(PADDING * 2.0, |_, _| {}),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(Headline {
                            text: quest.requirement.description(&game_state),
                            font_size: FontSize::Small,
                            text_align: TextAlign::LeftTop,
                            max_width: content_width.into(),
                        });
                    }),
                    table::fixed(PADDING, |_, _| {}),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(Paragraph {
                            text: quest.reward.description(),
                            font_size: FontSize::Medium,
                            text_align: TextAlign::LeftTop,
                            max_width: content_width.into(),
                        });
                    }),
                ])(Wh::new(content_width, f32::MAX.px()), ctx);
            });

            let Some(content_wh) = bounding_box(&content).map(|rect| rect.wh()) else {
                return;
            };
            let container_wh = content_wh + Wh::single(PADDING * 2.);

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
                    HEADLINE_FONT_SIZE_LARGE.into_px() + PADDING * 2.0,
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
        });

        quest_items.push(quest_item);
    }

    quest_items
}

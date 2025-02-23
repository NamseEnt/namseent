use crate::{
    game_state::{
        quest::{cancel_quest, Quest},
        use_game_state,
    },
    palette,
};
use namui::*;
use namui_prebuilt::{button::TextButton, table, typography};
use std::iter;

const QUESTS_WIDTH: Px = px(160.);
const PADDING: Px = px(4.);
const ITEM_HEIGHT: Px = px(36.);

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
                table::padding(PADDING, |wh, ctx| {
                    let height = ITEM_HEIGHT * (game_state.quests.len() + 1) as f32;

                    ctx.compose(|ctx| {
                        table::vertical(
                            iter::once(table::fixed(
                                ITEM_HEIGHT,
                                table::padding(PADDING, |wh, ctx| {
                                    ctx.add(typography::body::center(
                                        wh,
                                        format!(
                                            "퀘스트 {}/{}",
                                            game_state.items.len(),
                                            game_state.max_quests
                                        ),
                                        palette::ON_SURFACE,
                                    ));
                                }),
                            ))
                            .chain(
                                game_state
                                    .quests
                                    .iter()
                                    .enumerate()
                                    .map(|(item_index, quest)| {
                                        table::fixed(
                                            ITEM_HEIGHT,
                                            table::padding(PADDING, move |wh, ctx| {
                                                ctx.add(QuestsItem {
                                                    wh,
                                                    quest,
                                                    quest_index: item_index,
                                                });
                                            }),
                                        )
                                    }),
                            ),
                        )(Wh::new(wh.width, height), ctx);
                    });

                    ctx.add(rect(RectParam {
                        rect: Rect::zero_wh(Wh::new(wh.width, height)),
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
                }),
            )])(screen_wh, ctx);
        });
    }
}

struct QuestsItem<'a> {
    wh: Wh<Px>,
    quest: &'a Quest,
    quest_index: usize,
}
impl Component for QuestsItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            quest,
            quest_index,
        } = self;

        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(wh.height, |_, _| {
                    // TODO: Icons
                }),
                table::fixed(PADDING, |_, _| {}),
                table::ratio(1, |wh, ctx| {
                    ctx.add(typography::body::center(
                        wh,
                        quest.requirement.description(&game_state),
                        palette::ON_SURFACE,
                    ));
                }),
                table::fixed(wh.height, |wh, ctx| {
                    ctx.add(TextButton {
                        rect: wh.to_rect(),
                        text: "🗑️",
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
            ])(wh, ctx);
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
                    color: palette::PRIMARY,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}

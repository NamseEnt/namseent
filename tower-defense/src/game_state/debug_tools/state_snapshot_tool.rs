use crate::game_state::debug_tools::state_snapshot;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline, paragraph};
use namui::*;
use namui_prebuilt::table;

const BUTTON_HEIGHT: Px = px(32.);
const GAP: Px = px(8.);

pub struct StateSnapshotTool {
    pub width: Px,
}

impl Component for StateSnapshotTool {
    fn render(self, ctx: &RenderCtx) {
        let snapshots = state_snapshot::list_snapshots();

        ctx.compose(|ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(headline("State snapshots").build());
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &|| state_snapshot::save_current_snapshot(),
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    paragraph("Save snapshot now")
                                        .color(text_color)
                                        .align(TextAlign::Center { wh })
                                        .build(),
                                );
                            },
                        )
                        .variant(ButtonVariant::Outlined),
                    );
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &|| state_snapshot::clear_snapshots(),
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    paragraph("Clear all snapshots")
                                        .color(text_color)
                                        .align(TextAlign::Center { wh })
                                        .build(),
                                );
                            },
                        )
                        .variant(ButtonVariant::Text),
                    );
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    table::vertical(
                        snapshots
                            .into_iter()
                            .rev()
                            .map(|(idx, stage)| {
                                table::fit(table::FitAlign::LeftTop, move |ctx| {
                                    table::horizontal([
                                        table::ratio(1, |wh, ctx| {
                                            ctx.add(
                                                paragraph(format!(
                                                    "Snapshot #{idx} (Stage {stage})"
                                                ))
                                                .align(TextAlign::LeftCenter { height: wh.height })
                                                .build(),
                                            );
                                        }),
                                        table::fixed(GAP, |_, _| {}),
                                        table::fit(table::FitAlign::LeftTop, move |ctx| {
                                            ctx.add(
                                                Button::new(
                                                    Wh::new(px(80.), BUTTON_HEIGHT),
                                                    &move || state_snapshot::restore_snapshot(idx),
                                                    &|wh, text_color, ctx| {
                                                        ctx.add(
                                                            paragraph("Restore")
                                                                .color(text_color)
                                                                .align(TextAlign::Center { wh })
                                                                .build(),
                                                        );
                                                    },
                                                )
                                                .variant(ButtonVariant::Contained),
                                            );
                                        }),
                                    ])(
                                        Wh::new(self.width, BUTTON_HEIGHT), ctx
                                    );
                                })
                            })
                            .collect::<Vec<_>>(),
                    )(Wh::new(self.width, f32::MAX.px()), ctx);
                }),
            ])(Wh::new(self.width, f32::MAX.px()), ctx);
        });
    }
}

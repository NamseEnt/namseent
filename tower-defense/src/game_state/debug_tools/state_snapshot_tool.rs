use crate::game_state::debug_tools::state_snapshot;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{self, memoized_text};
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
                    ctx.add(memoized_text(
                        (),
                        |builder| {
                            builder
                                .headline()
                                .text("State snapshots")
                                .render_left_top()
                        },
                    ));
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &|| state_snapshot::save_current_snapshot(),
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    typography::paragraph()
                                        .color(text_color)
                                        .text("Save snapshot now")
                                        .render_center(wh),
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
                                    typography::paragraph()
                                        .color(text_color)
                                        .text("Clear all snapshots")
                                        .render_center(wh),
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
                                                typography::paragraph()
                                                    .text(format!(
                                                        "Snapshot #{idx} (Stage {stage})"
                                                    ))
                                                    .render_left_center(wh.height),
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
                                                            typography::paragraph()
                                                                .color(text_color)
                                                                .text("Restore")
                                                                .render_center(wh),
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

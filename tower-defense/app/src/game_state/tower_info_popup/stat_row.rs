use crate::theme::typography::{FontSize, memoized_text};
use namui::*;
use namui_prebuilt::table;

pub(super) struct PopupStatRow {
    pub(super) wh: Wh<Px>,
    pub(super) label: &'static str,
    pub(super) value: String,
}

impl Component for PopupStatRow {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, label, value } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio_no_clip(1, |wh, ctx| {
                    let label_string = label.to_string();
                    ctx.add(memoized_text((&label_string, &wh.width), |mut builder| {
                        builder
                            .paragraph()
                            .size(FontSize::Small)
                            .bold()
                            .text(&label_string)
                            .render_left_center(wh.height)
                    }));
                }),
                table::ratio_no_clip(1, |wh, ctx| {
                    let value_string = value.clone();
                    ctx.add(memoized_text((&value_string, &wh.width), |mut builder| {
                        builder
                            .paragraph()
                            .size(FontSize::Small)
                            .bold()
                            .text(&value_string)
                            .render_right_center(wh)
                    }));
                }),
            ])(wh, ctx);
        });
    }
}

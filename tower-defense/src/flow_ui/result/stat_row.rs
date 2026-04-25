use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::theme::typography::{self, memoized_text};
use namui::*;
use namui_prebuilt::table;

pub(super) struct StatRow {
    pub(super) wh: Wh<Px>,
    pub(super) label: &'static str,
    pub(super) value: String,
    pub(super) icon_kind: Option<IconKind>,
}

impl Component for StatRow {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            label,
            value,
            icon_kind,
        } = self;
        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio_no_clip(1, |wh, ctx| {
                    let label_string = label.to_string();
                    ctx.add(memoized_text((&label_string, &wh.width), |mut builder| {
                        builder
                            .paragraph()
                            .size(typography::FontSize::Medium)
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
                            .size(typography::FontSize::Medium)
                            .bold();
                        if let Some(kind) = icon_kind {
                            builder.with_icon_bold(kind, value_string.clone());
                        } else {
                            builder.text(&value_string);
                        }
                        builder.render_right_center(wh)
                    }));
                }),
            ])(wh, ctx);
        });
    }
}

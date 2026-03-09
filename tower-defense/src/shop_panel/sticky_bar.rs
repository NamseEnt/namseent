use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    typography::memoized_text,
};
use namui::*;

/// The small bar that remains visible when the shop panel is closed.  Tapping
/// toggles the panel open/closed.
pub(super) struct StickyBar<'a> {
    pub wh: Wh<Px>,
    pub panel_open: bool,
    pub disabled: bool,
    pub on_toggle: &'a dyn Fn(),
}

impl Component for StickyBar<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            panel_open,
            disabled,
            on_toggle,
        } = self;

        ctx.add(
            Button::new(wh, on_toggle, &|wh, text_color, ctx| {
                let label = if panel_open {
                    "닫기".to_string()
                } else {
                    "꺼내기".to_string()
                };
                ctx.add(memoized_text((&label, &text_color), |mut builder| {
                    builder
                        .headline()
                        .color(text_color)
                        .text(label.clone())
                        .render_center(wh)
                }));
            })
            .variant(ButtonVariant::Contained)
            .color(ButtonColor::Secondary)
            .disabled(disabled),
        );
    }
}

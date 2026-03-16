use crate::{
    icon::IconKind,
    theme::{
        button::{Button, ButtonColor, ButtonVariant},
        typography::memoized_text,
    },
};
use namui::*;

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
                ctx.add(memoized_text((&text_color, &panel_open), |mut builder| {
                    builder
                        .headline()
                        .size(crate::theme::typography::FontSize::Custom { size: wh.height })
                        .icon(IconKind::Card)
                        .render_center(wh)
                }));
            })
            .variant(ButtonVariant::Contained)
            .color(ButtonColor::Secondary)
            .disabled(disabled),
        );
    }
}

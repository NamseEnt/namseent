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
        let icon_size = wh.height - 24.px();
        let icon_wh = Wh::new(icon_size, icon_size);
        let icon_x = (wh.width - icon_wh.width) / 2.0;
        let icon_y = 6.px();

        ctx.add(
            Button::new(wh, on_toggle, &|_wh, text_color, ctx| {
                ctx.translate((icon_x, icon_y)).add(memoized_text(
                    (&text_color, &panel_open),
                    |mut builder| {
                        builder
                            .headline()
                            .size(crate::theme::typography::FontSize::Custom {
                                size: icon_wh.height,
                            })
                            .icon(IconKind::Card)
                            .render_center(icon_wh)
                    },
                ));
            })
            .variant(ButtonVariant::Contained)
            .color(ButtonColor::Secondary)
            .disabled(disabled),
        );
    }
}

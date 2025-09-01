use super::constants::SHOP_BUTTON_WH;
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use namui::*;

pub struct ShopOpenButton<'a> {
    pub opened: bool,
    pub toggle_open: &'a dyn Fn(),
}

impl Component for ShopOpenButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            opened,
            toggle_open,
        } = self;
        ctx.compose(|ctx| {
            ctx.translate((0.px(), -SHOP_BUTTON_WH.height)).add(
                Button::new(
                    SHOP_BUTTON_WH,
                    &|| {
                        toggle_open();
                    },
                    &|wh, _text_color, ctx| {
                        ctx.add(Icon::new(IconKind::Shop).size(IconSize::Large).wh(wh));
                    },
                )
                .variant(ButtonVariant::Fab)
                .color(match opened {
                    true => ButtonColor::Primary,
                    false => ButtonColor::Secondary,
                }),
            );
        });
    }
}

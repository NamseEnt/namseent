mod color;
mod note;
mod player;

use self::{color::THEME, player::Player};
use crate::app::note::load_notes;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (notes, set_notes) = ctx.state(|| None);
        let wh = screen::size().into_type::<Px>();

        ctx.effect("Init", || {
            spawn_local(async move {
                let notes = load_notes().await;
                set_notes.set(Some(notes));
            })
        });

        ctx.compose(|ctx| {
            if let Some(notes) = notes.as_ref() {
                ctx.add(Player { wh, notes });
            }
        });

        ctx.component(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            THEME.background.main,
        ));

        ctx.done()
    }
}

mod big_state;
mod card;
mod hand;
mod palette;
mod status;
mod tower;

use hand::Hand;
use namui::*;
use namui_prebuilt::simple_rect;
use status::{Flow, FLOW_ATOM};

pub fn main() {
    namui::start(|ctx| {
        ctx.add(Game {});
    });
}

struct Game {}
impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let _ = ctx.init_atom(&FLOW_ATOM, || Flow::SelectingTower);

        ctx.add(Hand { screen_wh });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_LOWEST,
        ));
    }
}

mod card;
mod game_state;
mod hand;
mod palette;
mod route;
mod status;
mod tower;

use hand::Hand;
use namui::*;
use namui_prebuilt::simple_rect;
use status::{Flow, FLOW_ATOM};

type BlockUnit = usize;
type BlockUnitF32 = f32;
type MapCoord = Xy<BlockUnit>;
type MapCoordF32 = Xy<BlockUnitF32>;

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
        let game_state = game_state::init_game_state(ctx);

        ctx.add(Hand { screen_wh });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_LOWEST,
        ));

        ctx.add(game_state.as_ref());
    }
}

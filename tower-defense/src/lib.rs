mod card;
mod game_state;
mod hand;
mod palette;
mod route;
mod status;
mod tower;
mod upgrade;
mod upgrade_board;
mod upgrade_select;

use hand::Hand;
use namui::*;
use namui_prebuilt::simple_rect;
use status::{Flow, FLOW_ATOM, UPGRADES_ATOM};
use upgrade_board::UpgradeBoardModal;
use upgrade_select::UpgradeSelectModal;

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
        let (flow, _set_flow) = ctx.init_atom(&FLOW_ATOM, || Flow::SelectingTower);
        let _ = ctx.init_atom(&UPGRADES_ATOM, || vec![]);
        let game_state = game_state::init_game_state(ctx);

        let (open_upgrade_board, set_open_upgrade_board) = ctx.state(|| false);

        let toggle_upgrade_board = || {
            set_open_upgrade_board
                .mutate(|open_upgrade_board| *open_upgrade_board = !*open_upgrade_board);
        };

        ctx.compose(|ctx| {
            if *open_upgrade_board {
                ctx.add(UpgradeBoardModal { screen_wh });
            }
        });

        ctx.compose(|ctx| {
            let Flow::SelectingUpgrade { upgrades } = flow.as_ref() else {
                return;
            };
            ctx.add(UpgradeSelectModal {
                screen_wh,
                upgrades,
            });
        });

        ctx.add(Hand { screen_wh });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_LOWEST,
        ));

        ctx.add(game_state.as_ref());

        ctx.attach_event(|event| {
            match event {
                Event::KeyDown { event } => match event.code {
                    Code::Tab => {
                        toggle_upgrade_board();
                    }
                    _ => {}
                },
                _ => {}
            };
        });
    }
}

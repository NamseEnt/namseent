mod card;
mod game_state;
mod hand;
mod inventory;
mod palette;
mod rarity;
mod route;
mod shop;
mod tower;
mod upgrade;
mod upgrade_board;
mod upgrade_select;

use game_state::{flow::GameFlow, mutate_game_state};
use hand::Hand;
use namui::*;
use namui_prebuilt::simple_rect;
use shop::ShopModal;
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
            let GameFlow::SelectingUpgrade { upgrades } = &game_state.flow else {
                return;
            };
            ctx.add(UpgradeSelectModal {
                screen_wh,
                upgrades,
            });
        });

        ctx.compose(|ctx| {
            let GameFlow::SelectingTower = &game_state.flow else {
                return;
            };
            if game_state.stage % 2 != 0 {
                return;
            }
            ctx.add(ShopModal { screen_wh });
        });

        ctx.add(Hand { screen_wh });

        ctx.add(game_state.as_ref());

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_LOWEST,
        ));

        ctx.attach_event(|event| {
            match event {
                Event::KeyDown { event } => match event.code {
                    Code::Tab => {
                        toggle_upgrade_board();
                    }
                    _ => {}
                },
                Event::Wheel { event } => {
                    let delta = -event.delta_xy.y / 2048.0;
                    mutate_game_state(move |game_state| {
                        game_state.camera.zoom(delta);
                    });
                }
                _ => {}
            };
        });
    }
}

mod asset_loader;
mod card;
mod game_state;
mod inventory;
mod quest_board;
mod quests;
mod rarity;
mod route;
mod shop;
mod theme;
mod top_bar;
mod tower_placing_hand;
mod tower_selecting_hand;
mod upgrade;
mod upgrade_board;
mod upgrade_select;

use asset_loader::AssetLoader;
use game_state::{TILE_PX_SIZE, flow::GameFlow, mutate_game_state};
use inventory::Inventory;
use namui::*;
use namui_prebuilt::simple_rect;
use quest_board::QuestBoardModal;
use quests::Quests;
use shop::ShopModal;
use theme::palette;
use top_bar::TopBar;
use tower_placing_hand::TowerPlacingHand;
use tower_selecting_hand::TowerSelectingHand;
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
        let (middle_mouse_button_dragging, set_middle_mouse_button_dragging) = ctx.state(|| None);

        let (open_upgrade_board, set_open_upgrade_board) = ctx.state(|| false);

        let toggle_upgrade_board = || {
            set_open_upgrade_board
                .mutate(|open_upgrade_board| *open_upgrade_board = !*open_upgrade_board);
        };

        ctx.add(AssetLoader {});

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
                upgrades: &upgrades,
            });
        });

        ctx.compose(|ctx| {
            let GameFlow::SelectingTower { cards } = &game_state.flow else {
                return;
            };
            ctx.add(TowerSelectingHand { screen_wh, cards });

            let in_even_stage = game_state.in_even_stage();

            ctx.compose(|ctx| {
                if !in_even_stage {
                    return;
                }
                ctx.add(ShopModal { screen_wh });
            });

            ctx.compose(|ctx| {
                if in_even_stage {
                    return;
                }
                ctx.add(QuestBoardModal { screen_wh });
            });
        });

        ctx.compose(|ctx| {
            let GameFlow::PlacingTower {
                placing_tower_slots,
            } = &game_state.flow
            else {
                return;
            };
            ctx.add(TowerPlacingHand {
                screen_wh,
                placing_tower_slots,
            });
        });

        ctx.add(Inventory { screen_wh });

        ctx.add(Quests { screen_wh });

        ctx.add(TopBar { screen_wh });

        ctx.add(game_state.as_ref());

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_LOWEST,
        ));

        ctx.attach_event(|event| {
            match event {
                Event::KeyDown { event } => {
                    if event.code == Code::Tab {
                        toggle_upgrade_board();
                    }
                }
                Event::Wheel { event } => {
                    let delta = -event.delta_xy.y / 2048.0;
                    let origin = event.local_xy();
                    mutate_game_state(move |game_state| {
                        game_state.camera.zoom(delta, origin);
                    });
                }

                Event::MouseDown { event } => {
                    let Some(button) = event.button else {
                        return;
                    };
                    if button == MouseButton::Middle {
                        set_middle_mouse_button_dragging.set(Some(MiddleMouseButtonDragging {
                            last_global_xy: event.global_xy,
                        }));
                    };
                }
                Event::MouseMove { event } => {
                    if event.pressing_buttons.contains(&MouseButton::Middle) {
                        if let Some(middle_mouse_button_dragging) =
                            middle_mouse_button_dragging.as_ref()
                        {
                            let global_xy = event.global_xy;
                            let delta = global_xy - middle_mouse_button_dragging.last_global_xy;
                            mutate_game_state(move |game_state| {
                                game_state.camera.move_by(delta * -1.0);
                            });
                            set_middle_mouse_button_dragging.set(Some(MiddleMouseButtonDragging {
                                last_global_xy: global_xy,
                            }));
                        }
                    }
                    if game_state.cursor_preview.should_update_position() {
                        let local_xy_tile =
                            (event.global_xy / game_state.camera.zoom_level) / TILE_PX_SIZE.as_xy();
                        let map_coord = game_state.camera.left_top + local_xy_tile;
                        mutate_game_state(move |game_state| {
                            game_state.cursor_preview.update_position(map_coord);
                        });
                    }
                }
                Event::MouseUp { event } => {
                    let Some(button) = event.button else {
                        return;
                    };

                    if button == MouseButton::Middle {
                        set_middle_mouse_button_dragging.set(None);
                    }
                }
                Event::VisibilityChange => {
                    if middle_mouse_button_dragging.is_some() {
                        set_middle_mouse_button_dragging.set(None);
                    }
                }
                _ => {}
            };
        });
    }
}

struct MiddleMouseButtonDragging {
    last_global_xy: Xy<Px>,
}

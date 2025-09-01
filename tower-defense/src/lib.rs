mod asset_loader;
mod card;
mod flow_ui;
mod game_speed_indicator;
mod game_state;
mod hand;
mod icon;
mod inventory;
pub mod l10n;
mod rarity;
mod route;
mod settings;
mod shop;
mod theme;
mod thumbnail;
mod top_bar;
mod upgrade_board;
mod upgrade_select;

use crate::{
    game_state::{Modal, set_modal},
    icon::{Icon, IconKind, IconSize},
    theme::button::{Button, ButtonVariant},
};
use asset_loader::LoadingScreen;
use game_speed_indicator::GameSpeedIndicator;
use game_state::{TILE_PX_SIZE, flow::GameFlow, mutate_game_state};
use inventory::Inventory;
use namui::*;
use namui_prebuilt::simple_rect;
use theme::palette;
use top_bar::TopBar;
use upgrade_select::UpgradeSelectModal;

type BlockUnit = usize;
type BlockUnitF32 = f32;
type MapCoord = Xy<BlockUnit>;
type MapCoordF32 = Xy<BlockUnitF32>;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(Game {});
    });
}

struct Game {}
impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let game_state = game_state::init_game_state(ctx);
        let (middle_mouse_button_dragging, set_middle_mouse_button_dragging) = ctx.state(|| None);

        if matches!(&game_state.flow, GameFlow::Initializing) {
            ctx.add(LoadingScreen {
                screen_wh,
                on_complete: &|| {
                    mutate_game_state(|game_state| {
                        game_state.goto_selecting_tower();
                    });
                },
            });
            return;
        }

        ctx.compose(|ctx| {
            let Some(modal) = game_state.opened_modal.as_ref() else {
                return;
            };

            ctx.add(modal);
        });

        ctx.translate((8.px(), screen_wh.height - 48.px())).add(
            Button::new(
                Wh::new(36.px(), 36.px()),
                &|| {
                    set_modal(Some(Modal::Settings));
                },
                &|wh, _text_color, ctx| {
                    ctx.add(Icon::new(IconKind::Config).size(IconSize::Large).wh(wh));
                },
            )
            .variant(ButtonVariant::Text),
        );

        // Game speed indicator in bottom-right corner
        ctx.translate((screen_wh.width - 116.px(), screen_wh.height - 88.px()))
            .add(GameSpeedIndicator);

        ctx.compose(|ctx| {
            let GameFlow::SelectingUpgrade { upgrades } = &game_state.flow else {
                return;
            };
            ctx.add(UpgradeSelectModal {
                screen_wh,
                upgrades,
            });
        });

        ctx.add(flow_ui::FlowUi);

        ctx.add(Inventory { screen_wh });

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
                    match event.code {
                        Code::Tab => {
                            set_modal(Some(Modal::UpgradeBoard));
                        }
                        Code::KeyQ => {
                            mutate_game_state(|game_state| {
                                game_state.fast_forward_multiplier =
                                    game_state.fast_forward_multiplier.prev();
                            });
                        }
                        Code::KeyE => {
                            mutate_game_state(|game_state| {
                                game_state.fast_forward_multiplier =
                                    game_state.fast_forward_multiplier.next();
                            });
                        }
                        _ => {}
                    };
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
                    if event.pressing_buttons.contains(&MouseButton::Middle)
                        && let Some(middle_mouse_button_dragging) =
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
                    if game_state.cursor_preview.should_update_position()
                        || matches!(
                            game_state.flow,
                            crate::game_state::flow::GameFlow::PlacingTower { hand: _ }
                        )
                    {
                        let local_xy_tile =
                            (event.global_xy / game_state.camera.zoom_level) / TILE_PX_SIZE.to_xy();
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

mod animation;
mod camera_controller;
mod card;
pub mod config;
mod flow_ui;
mod game_state; // now private; selective re-exports below
mod hand;
mod hand_panel;
mod icon;
mod inventory;
pub mod l10n;
mod rarity; // private; re-export Rarity only
mod route;
mod shop;
mod shop_panel;
#[cfg(feature = "simulator")]
pub mod simulator;
pub mod sound;
mod theme;
mod thumbnail;
mod tooltip;
mod top_bar;
mod upgrades;

use crate::{camera_controller::CameraController, game_state::Modal};
use game_state::{TILE_PX_SIZE, mutate_game_state};
use inventory::Inventory;
use namui::*;
use namui_prebuilt::{simple_rect, table};
use theme::palette;
use top_bar::TopBar;
use upgrades::Upgrades;

const TOP_BAR_HEIGHT: Px = px(48.);

register_assets!();

type BlockUnit = usize;
type BlockUnitF32 = f32;
type MapCoord = Xy<BlockUnit>;
type MapCoordF32 = Xy<BlockUnitF32>;

/// Global headless mode flag. When true, sound and particle side effects are suppressed.
static HEADLESS_MODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_headless(headless: bool) {
    HEADLESS_MODE.store(headless, std::sync::atomic::Ordering::Relaxed);
}

pub fn is_headless() -> bool {
    HEADLESS_MODE.load(std::sync::atomic::Ordering::Relaxed)
}

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
        let _sound_state = sound::init_sound_state(ctx);
        let (middle_mouse_button_dragging, set_middle_mouse_button_dragging) = ctx.state(|| None);

        ctx.compose(|ctx| {
            let Some(modal) = game_state.opened_modal.as_ref() else {
                return;
            };

            ctx.add(modal);
        });

        ctx.add(flow_ui::FlowUi);

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed_no_clip(TOP_BAR_HEIGHT, |wh, ctx| {
                    ctx.add(TopBar { wh });
                }),
                table::ratio_no_clip(
                    1,
                    table::padding_no_clip(
                        8.px(),
                        table::horizontal([
                            table::fixed_no_clip(px(92.), |wh, ctx| {
                                ctx.add(Upgrades { wh });
                            }),
                            table::ratio_no_clip(1, |_, _| {}),
                            table::fixed_no_clip(px(92.), |wh, ctx| {
                                ctx.add(Inventory { wh });
                            }),
                        ]),
                    ),
                ),
                table::fixed(128.px(), |_, _| {}),
            ])(screen_wh, ctx);
        });

        ctx.add(shop_panel::ShopPanel);

        ctx.add(hand_panel::HandPanel);

        ctx.add(sound::SoundRenderer);

        ctx.add(game_state::RenderGameState {
            game_state: game_state.as_ref(),
        });

        ctx.add(CameraController);

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            palette::SURFACE_CONTAINER_LOWEST,
        ));

        ctx.attach_event(move |event| {
            match event {
                Event::KeyDown { event } => match event.code {
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
                    Code::Space => {
                        mutate_game_state(|game_state| {
                            game_state.toggle_panels();
                        });
                    }
                    #[cfg(feature = "debug-tools")]
                    Code::F8 => {
                        mutate_game_state(|game_state| {
                            if matches!(game_state.opened_modal, Some(Modal::DebugTools)) {
                                game_state.opened_modal = None;
                            } else {
                                game_state.opened_modal = Some(Modal::DebugTools);
                            }
                        });
                    }
                    _ => {}
                },
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
                            crate::game_state::flow::GameFlow::PlacingTower
                        )
                    {
                        let local_xy_tile =
                            (event.global_xy / game_state.camera.zoom_level) / TILE_PX_SIZE.to_xy();
                        let map_coord = game_state.camera.visual_left_top() + local_xy_tile;
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

#[derive(State)]
struct MiddleMouseButtonDragging {
    last_global_xy: Xy<Px>,
}
// --- Public API Surface (narrow) -------------------------------------------------
// Re-export only the symbols required by integration tests / external consumers.
pub use card::{Card, Rank, Suit};
pub use game_state::tower::TowerKind;
pub use game_state::upgrade::UpgradeState;
pub use rarity::Rarity;

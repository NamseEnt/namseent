use crate::{
    game_state::{
        Modal, force_start,
        hand::{HAND_WH, HandComponent, HandSlotId},
        mutate_game_state, set_modal, use_game_state,
    },
    theme::{
        button::{Button, ButtonColor, ButtonVariant},
        typography::{TextAlign, headline},
    },
};
use namui::*;
use namui_prebuilt::table;

pub struct TowerPlacingHand;

impl Component for TowerPlacingHand {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let game_state = use_game_state(ctx);

        // Only render if we're in PlacingTower flow
        let (hand, selected_hand_slot_ids) = match &game_state.flow {
            crate::game_state::flow::GameFlow::PlacingTower { hand } => {
                let selected_hand_slot_ids = ctx.track_eq(&hand.selected_slot_ids());
                (hand, selected_hand_slot_ids)
            }
            _ => return, // Don't render if not in PlacingTower flow
        };

        let select_tower = |slot_id: HandSlotId| {
            if !selected_hand_slot_ids.is_empty() {
                return;
            }

            // Find the tower template by slot ID
            let Some(_tower_template) = hand.get_item(slot_id) else {
                return;
            };

            mutate_game_state(move |game_state| {
                if let crate::game_state::flow::GameFlow::PlacingTower { hand } =
                    &mut game_state.flow
                {
                    hand.select_slot(slot_id);
                }
            });
        };

        let handle_start_button_click = || {
            if hand.is_empty() {
                force_start();
            } else {
                set_modal(Some(Modal::StartConfirm));
            }
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(
                    HAND_WH.height,
                    table::horizontal([
                        table::ratio_no_clip(1, |_, _| {}),
                        table::fixed_no_clip(HAND_WH.width, |_wh, ctx| {
                            ctx.add(HandComponent {
                                hand,
                                on_click: &select_tower,
                            });
                        }),
                        table::fixed_no_clip(120.px(), |wh, ctx| {
                            ctx.compose(|ctx| {
                                table::vertical([
                                    table::ratio(1, |_, _| {}),
                                    table::fixed(48.px(), |wh, ctx| {
                                        let padding = px(8.0);
                                        table::padding(padding, |wh, ctx| {
                                            ctx.add(
                                                Button::new(
                                                    wh,
                                                    &|| {
                                                        handle_start_button_click();
                                                    },
                                                    &|wh, text_color, ctx| {
                                                        ctx.add(
                                                            headline("START")
                                                                .color(text_color)
                                                                .align(TextAlign::Center { wh })
                                                                .build(),
                                                        );
                                                    },
                                                )
                                                .variant(ButtonVariant::Contained)
                                                .color(ButtonColor::Primary),
                                            );
                                        })(wh, ctx);
                                    }),
                                    table::ratio(1, |_, _| {}),
                                ])(wh, ctx);
                            });
                        }),
                        table::ratio_no_clip(1, |_, _| {}),
                    ]),
                ),
            ])(screen_wh, ctx);
        });
    }
}

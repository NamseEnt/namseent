use crate::{
    game_state::{
        Modal,
        cursor_preview::PreviewKind,
        force_start,
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

pub struct TowerPlacingHand {
    pub screen_wh: Wh<Px>,
}
impl Component for TowerPlacingHand {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        let game_state = use_game_state(ctx);
        let selected_hand_slot_ids = {
            let selected_hand_slot_ids = game_state.hand.selected_slot_ids();
            ctx.track_eq(&selected_hand_slot_ids)
        };

        let select_tower = |slot_id: HandSlotId| {
            if !selected_hand_slot_ids.is_empty() {
                return;
            }

            // Find the tower template by slot ID
            let Some(tower_template) = game_state.hand.get_tower_template_by_id(slot_id) else {
                return;
            };

            let tower_template = tower_template.clone();
            mutate_game_state(move |game_state| {
                game_state.hand.select_slot(slot_id);
                game_state.cursor_preview.kind = PreviewKind::PlacingTower {
                    tower_template,
                    placing_tower_slot_id: slot_id,
                };
            });
        };

        let handle_start_button_click = || {
            if game_state.hand.is_empty() {
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
                                hand: &game_state.hand,
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

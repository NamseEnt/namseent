use crate::{
    game_state::{Modal, mutate_game_state, set_modal, use_game_state},
    hand::{HAND_WH, HandComponent, HandSlotId},
    icon::{Icon, IconAttribute},
    theme::{
        button::{Button, ButtonColor, ButtonVariant},
        palette,
        typography::{TextAlign, headline},
    },
};
use namui::*;
use namui_prebuilt::table;

const PADDING: Px = px(4.);

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
                        table::fixed_no_clip(
                            HAND_WH.height,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.compose(|ctx| {
                                    table::padding(
                                        PADDING,
                                        table::vertical([
                                            table::fixed(48.px(), |wh, ctx| {
                                                ctx.add(
                                                    Button::new(
                                                        wh,
                                                        &|| {
                                                            set_modal(Some(Modal::Challenge));
                                                        },
                                                        &|wh, _text_color, ctx| {
                                                            ctx.add(Icon::new(crate::icon::IconKind::Gold).wh(wh).size(crate::icon::IconSize::Large).attributes(vec![
                                                                IconAttribute::new(crate::icon::IconKind::Add).position(crate::icon::IconAttributePosition::TopLeft)
                                                            ]));
                                                        },
                                                    )
                                                    .variant(ButtonVariant::Contained)
                                                    .color(ButtonColor::Primary),
                                                );
                                            }),
                                            table::ratio(1, |_, _| {}),
                                            table::fixed(48.px(), |wh, ctx| {
                                                ctx.add(
                                                    Button::new(
                                                        wh,
                                                        &|| {
                                                            mutate_game_state(|game_state| { 
                                                                game_state.goto_defense();
                                                            });
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
                                                    .long_press_time(1.sec())
                                                    .variant(ButtonVariant::Contained)
                                                    .color(ButtonColor::Primary),
                                                );
                                            }),
                                            
                                        ]),
                                    )(wh, ctx);
                                });

                                ctx.add(rect(RectParam {
                                    rect: wh.to_rect(),
                                    style: RectStyle {
                                        stroke: Some(RectStroke {
                                            color: palette::OUTLINE,
                                            width: 1.px(),
                                            border_position: BorderPosition::Inside,
                                        }),
                                        fill: Some(RectFill {
                                            color: palette::SURFACE,
                                        }),
                                        round: Some(RectRound {
                                            radius: palette::ROUND,
                                        }),
                                    },
                                }));
                            }),
                        ),
                        table::ratio_no_clip(1, |_, _| {}),
                    ]),
                ),
            ])(screen_wh, ctx);
        });
    }
}

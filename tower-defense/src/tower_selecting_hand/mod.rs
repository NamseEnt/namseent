mod get_highest_tower;
mod tower_preview;

use crate::theme::button::Button;
use crate::{
    game_state::{
        hand::{HAND_WH, HandComponent, HandSlotId},
        mutate_game_state,
        quest::{QuestTriggerEvent, on_quest_trigger_event},
    },
    l10n::ui::TopBarText,
    palette,
};
use get_highest_tower::get_highest_tower_template;
use namui::*;
use namui_prebuilt::table;
use tower_preview::TowerPreview;

const PADDING: Px = px(4.);

pub struct TowerSelectingHand {
    pub screen_wh: Wh<Px>,
}
impl Component for TowerSelectingHand {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        let game_state = crate::game_state::use_game_state(ctx);
        let selected_hand_slot_ids = {
            let selected_hand_slot_ids = game_state.hand.selected_slot_ids();
            ctx.track_eq(&selected_hand_slot_ids)
        };
        let some_selected = ctx.memo(|| !selected_hand_slot_ids.is_empty());
        let using_cards = ctx.memo(|| {
            let selected_cards = game_state.hand.selected_cards();
            if !selected_cards.is_empty() {
                return selected_cards;
            }
            game_state.hand.all_cards()
        });
        let tower_template = ctx.memo(|| {
            get_highest_tower_template(
                &using_cards,
                &game_state.upgrade_state,
                game_state.rerolled_count,
            )
        });

        let reroll_selected = || {
            if game_state.left_reroll_chance == 0 || selected_hand_slot_ids.is_empty() {
                return;
            }
            let selected_hand_slot_ids = selected_hand_slot_ids.clone_inner();
            mutate_game_state(move |game_state| {
                if game_state.left_reroll_chance == 0 || selected_hand_slot_ids.is_empty() {
                    return;
                }
                let select_count = selected_hand_slot_ids.len();
                game_state.hand.delete_slots(&selected_hand_slot_ids);
                game_state.hand.add_random_cards(select_count);
                game_state.left_reroll_chance -= 1;
                game_state.rerolled_count += 1;
                on_quest_trigger_event(game_state, QuestTriggerEvent::Reroll);
            });
        };

        let on_card_click = |id: HandSlotId| {
            mutate_game_state(move |game_state| {
                if game_state.hand.selected_slot_ids().contains(&id) {
                    game_state.hand.deselect_slot(id);
                } else {
                    game_state.hand.select_slot(id);
                }
            });
        };

        let use_tower = || {
            let tower_template = tower_template.clone_inner();
            mutate_game_state(move |state| {
                state.goto_placing_tower(tower_template);
            });
        };
        ctx.compose(|ctx| {
            table::vertical([
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(
                    HAND_WH.height,
                    table::horizontal([
                        table::ratio_no_clip(1, |_, _| {}),
                        table::fixed_no_clip(
                            HAND_WH.height,
                            table::padding_no_clip(PADDING, |wh, ctx| {
                                ctx.add(TowerPreview {
                                    wh,
                                    tower_template: &tower_template,
                                });
                            }),
                        ),
                        table::fixed_no_clip(HAND_WH.width, |_wh, ctx| {
                            ctx.add(HandComponent {
                                hand: &game_state.hand,
                                on_click: &on_card_click,
                            });
                        }),
                        table::fixed(
                            HAND_WH.height,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(InteractionArea {
                                    wh,
                                    some_selected: *some_selected,
                                    reroll_selected: &reroll_selected,
                                    use_tower: &use_tower,
                                });
                            }),
                        ),
                        table::ratio_no_clip(1, |_, _| {}),
                    ]),
                ),
            ])(screen_wh, ctx);
        });
    }
}

struct InteractionArea<'a> {
    wh: Wh<Px>,
    some_selected: bool,
    reroll_selected: &'a dyn Fn(),
    use_tower: &'a dyn Fn(),
}
impl Component for InteractionArea<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            some_selected,
            reroll_selected,
            use_tower,
        } = self;
        let game_state = crate::game_state::use_game_state(ctx);
        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::fixed(32.px(), |wh, ctx| {
                        ctx.add(Button::new(
                            wh,
                            &|| {
                                reroll_selected();
                            },
                            &|wh, _text_color, ctx| {
                                let text_color = match some_selected {
                                    true => palette::ON_PRIMARY,
                                    false => palette::ON_SURFACE,
                                };
                                ctx.add(namui::text(TextParam {
                                    text: game_state.text().ui(TopBarText::Refresh).to_string(),
                                    x: wh.width / 2.0,
                                    y: wh.height / 2.0,
                                    align: TextAlign::Center,
                                    baseline: TextBaseline::Middle,
                                    font: Font {
                                        size: 14.int_px(),
                                        name: "NotoSansKR-Regular".to_string(),
                                    },
                                    style: TextStyle {
                                        color: text_color,
                                        background: None,
                                        border: None,
                                        drop_shadow: None,
                                        line_height_percent: 100.percent(),
                                        underline: None,
                                    },
                                    max_width: None,
                                }));
                            },
                        ));
                    }),
                    table::ratio(1, |_, _| {}),
                    table::fixed(32.px(), |wh, ctx| {
                        ctx.add(Button::new(
                            wh,
                            &|| {
                                use_tower();
                            },
                            &|wh, _text_color, ctx| {
                                ctx.add(namui::text(TextParam {
                                    text: game_state.text().ui(TopBarText::UseTower).to_string(),
                                    x: wh.width / 2.0,
                                    y: wh.height / 2.0,
                                    align: TextAlign::Center,
                                    baseline: TextBaseline::Middle,
                                    font: Font {
                                        size: 14.int_px(),
                                        name: "NotoSansKR-Regular".to_string(),
                                    },
                                    style: TextStyle {
                                        color: palette::ON_PRIMARY,
                                        background: None,
                                        border: None,
                                        drop_shadow: None,
                                        line_height_percent: 100.percent(),
                                        underline: None,
                                    },
                                    max_width: None,
                                }));
                            },
                        ));
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
    }
}

use crate::game_state::difficulty::{DifficultyChoices, DifficultyOption};
use crate::game_state::{is_boss_stage, mutate_game_state, set_modal, use_game_state};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::{self, memoized_text};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const MODAL_WIDTH: Px = px(960.);
const MODAL_HEIGHT: Px = px(520.);
const PANEL_PADDING: Px = px(16.);
const CARD_GAP: Px = px(24.);
const CARD_PADDING: Px = px(16.);
const CARD_NAME_HEIGHT: Px = px(40.);
const CARD_DIVIDER_HEIGHT: Px = px(2.);
const CARD_EFFECT_ROW_HEIGHT: Px = px(28.);

pub struct OperationPlanModal;

impl Component for OperationPlanModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let modal_wh = Wh::new(MODAL_WIDTH, MODAL_HEIGHT);
        let base_modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        let game_state = use_game_state(ctx);
        let text = game_state.text();
        let choices = game_state.stage_difficulty_choices.clone();

        let (hovered_card, set_hovered_card) = ctx.state::<Option<usize>>(|| None);

        let card_content_width = MODAL_WIDTH - PANEL_PADDING * 2.0;
        let card_content_height = MODAL_HEIGHT - PANEL_PADDING * 2.0;

        let is_boss_pre_stage = is_boss_stage(game_state.stage + 1);
        let options = if is_boss_pre_stage {
            vec![choices.all_in.clone()]
        } else {
            vec![
                choices.fold.clone(),
                choices.call.clone(),
                choices.raise.clone(),
            ]
        };

        let num_cards = options.len() as f32;
        let (card_width, card_horizontal_offset) = if num_cards > 0.0 {
            let total_gaps = CARD_GAP * (num_cards - 1.0);
            let raw_width = (card_content_width - total_gaps) / num_cards;
            let card_width = raw_width.floor();
            let total_cards = card_width * num_cards + total_gaps;
            let offset = ((card_content_width - total_cards) / 2.0).max(0.px());
            (card_width, offset)
        } else {
            (card_content_width, 0.px())
        };

        // Modal content (rendered on top per AGENTS.md front-to-back rule)
        ctx.compose(|ctx| {
            let ctx = ctx.translate(base_modal_xy);

            // Phase 1: Layout (compose consumes inner ctx)
            ctx.compose(|ctx| {
                table::vertical([table::ratio_no_clip(
                    1,
                    table::padding_no_clip(PANEL_PADDING, |_wh, ctx| {
                        let mut cells: Vec<table::TableCell> = Vec::new();
                        let card_count = options.len();
                        for (index, option) in options.into_iter().enumerate() {
                            let option = option.clone();

                            cells.push(table::fixed_no_clip(card_width, move |wh, ctx| {
                                let hovering = *hovered_card == Some(index);
                                let card_bg = if hovering {
                                    palette::SURFACE_CONTAINER_HIGHEST
                                } else {
                                    palette::SURFACE_CONTAINER_HIGH
                                };

                                ctx.compose(|ctx| {
                                    render_card_layout(&ctx, &option, wh, text);

                                    ctx.add(
                                        simple_rect(
                                            wh,
                                            Color::TRANSPARENT,
                                            0.px(),
                                            Color::TRANSPARENT,
                                        )
                                        .attach_event(
                                            move |event| match event {
                                                Event::MouseMove { event } => {
                                                    if event.is_local_xy_in() {
                                                        set_hovered_card.set(Some(index));
                                                    } else if hovering {
                                                        set_hovered_card.set(None);
                                                    }
                                                }
                                                Event::MouseUp { event } => {
                                                    if event.is_local_xy_in() {
                                                        event.stop_propagation();
                                                        mutate_game_state(move |gs| {
                                                            option.apply(gs);
                                                            gs.stage_difficulty_choices =
                                                                DifficultyChoices::default();
                                                            gs.goto_defense();
                                                        });
                                                        set_modal(None);
                                                    }
                                                }
                                                _ => {}
                                            },
                                        ),
                                    );

                                    ctx.add(PaperContainerBackground {
                                        width: wh.width,
                                        height: wh.height,
                                        texture: PaperTexture::Rough,
                                        variant: PaperVariant::Card,
                                        color: card_bg,
                                        shadow: hovering,
                                        arrow: None,
                                    });
                                });
                            }));

                            if index + 1 < card_count {
                                cells.push(table::fixed_no_clip(CARD_GAP, |_, _| {}));
                            }
                        }

                        ctx.translate((card_horizontal_offset, 0.px()))
                            .compose(|ctx| {
                                table::horizontal(cells)(
                                    Wh::new(card_content_width, card_content_height),
                                    ctx,
                                );
                            });
                    }),
                )])(modal_wh, ctx);
            });

            ctx.add(PaperContainerBackground {
                width: modal_wh.width,
                height: modal_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Paper,
                color: palette::SURFACE_CONTAINER_LOWEST,
                shadow: true,
                arrow: None,
            });
        })
        .attach_event(|event| {
            match event {
                Event::MouseDown { event }
                | Event::MouseMove { event }
                | Event::MouseUp { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                }
                Event::Wheel { event } => {
                    if !event.is_local_xy_in() {
                        return;
                    }
                    event.stop_propagation();
                }
                _ => {}
            };
        });

        // Transparent overlay (behind modal, click to close)
        ctx.add(
            simple_rect(
                screen_wh,
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(0, 0, 0, 160),
            )
            .attach_event(|event| match event {
                Event::MouseDown { event } => {
                    set_modal(None);
                    event.stop_propagation();
                }
                Event::MouseMove { event } | Event::MouseUp { event } => {
                    event.stop_propagation();
                }
                _ => {}
            }),
        );
    }
}

enum OperationPlanRow {
    Effect(crate::game_state::effect::Effect),
    Dopamine(i8),
    Token(i8),
    NextOffer(crate::game_state::poker_action::NextStageOffer),
}

fn render_card_layout(
    ctx: &ComposeCtx,
    option: &DifficultyOption,
    card_wh: Wh<Px>,
    text: crate::l10n::TextManager,
) {
    let name = option.action.to_text(&text.locale()).to_string();
    let effects: Vec<crate::game_state::effect::Effect> = option.effects.clone();

    // middle: text content
    ctx.compose(|ctx| {
        let mut rows: Vec<table::TableCell> = Vec::new();

        rows.push(table::fixed_no_clip(CARD_NAME_HEIGHT, move |wh, ctx| {
            let name = name.clone();
            ctx.add(memoized_text((), move |mut builder| {
                builder
                    .headline()
                    .size(typography::FontSize::Medium)
                    .text(name.clone())
                    .render_center(wh)
            }));
        }));

        let mut operation_rows: Vec<OperationPlanRow> =
            vec![OperationPlanRow::Dopamine(option.action.dopamine_delta())];

        if option.next_stage_offer != crate::game_state::poker_action::NextStageOffer::None {
            operation_rows.push(OperationPlanRow::NextOffer(option.next_stage_offer));
        }

        let token_delta = option.action.token_delta();
        if token_delta != 0 {
            operation_rows.push(OperationPlanRow::Token(token_delta));
        }

        for effect in effects {
            operation_rows.push(OperationPlanRow::Effect(effect));
        }

        for row in operation_rows {
            rows.push(table::fixed_no_clip(
                CARD_EFFECT_ROW_HEIGHT,
                move |wh, ctx| match row {
                    OperationPlanRow::Dopamine(delta) => {
                        let color = if delta > 0 {
                            palette::BLUE
                        } else if delta < 0 {
                            palette::RED
                        } else {
                            palette::ON_SURFACE
                        };

                        ctx.add(memoized_text((), move |mut builder| {
                            builder
                                .headline()
                                .size(typography::FontSize::Small)
                                .color(color)
                                .text(format!("도파민 {:+}", delta))
                                .render_left_center(wh.height)
                        }));
                    }
                    OperationPlanRow::Token(delta) => {
                        let color = if delta > 0 {
                            palette::BLUE
                        } else {
                            palette::RED
                        };

                        ctx.add(memoized_text((), move |mut builder| {
                            builder
                                .headline()
                                .size(typography::FontSize::Small)
                                .color(color)
                                .text(format!("토큰 {:+}", delta))
                                .render_left_center(wh.height)
                        }));
                    }
                    OperationPlanRow::Effect(effect) => {
                        let effect_text =
                            crate::l10n::effect::EffectText::Description(effect.clone());
                        let locale = text.locale();
                        ctx.add(memoized_text((), move |mut builder| {
                            builder
                                .headline()
                                .size(typography::FontSize::Small)
                                .color(effect_text_color(&effect))
                                .l10n(effect_text.clone(), &locale)
                                .render_left_center(wh.height)
                        }));
                    }
                    OperationPlanRow::NextOffer(next_offer) => {
                        ctx.add(memoized_text((), move |mut builder| {
                            builder
                                .headline()
                                .size(typography::FontSize::Small)
                                .color(palette::BLUE)
                                .l10n(
                                    crate::l10n::poker_action::NextStageOfferText::Description(
                                        next_offer,
                                    ),
                                    &text.locale(),
                                )
                                .render_left_center(wh.height)
                        }));
                    }
                },
            ));
        }

        rows.push(table::fixed_no_clip(CARD_DIVIDER_HEIGHT, |wh, ctx| {
            ctx.add(simple_rect(
                Wh::new(wh.width, CARD_DIVIDER_HEIGHT),
                palette::OUTLINE,
                0.px(),
                Color::TRANSPARENT,
            ));
        }));

        rows.push(table::fixed_no_clip(8.px(), |_, _| {}));

        rows.push(table::ratio_no_clip(1, |_, _| {}));

        table::padding_no_clip(CARD_PADDING, table::vertical(rows))(card_wh, ctx);
    });
}

fn effect_text_color(effect: &crate::game_state::effect::Effect) -> Color {
    if effect.is_positive() {
        palette::BLUE
    } else {
        palette::RED
    }
}

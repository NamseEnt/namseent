use crate::animation::xy_with_spring;
use crate::game_state::difficulty::{
    DifficultyChoices, DifficultyGroup, DifficultyOption, OperationKind,
};
use crate::game_state::{mutate_game_state, set_modal, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::OperationPlanText;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::{self, memoized_text};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const MODAL_WIDTH: Px = px(960.);
const MODAL_HEIGHT: Px = px(520.);
const TITLE_HEIGHT: Px = px(64.);
const PANEL_PADDING: Px = px(16.);
const CARD_GAP: Px = px(24.);
const CARD_PADDING: Px = px(16.);
const CARD_NAME_HEIGHT: Px = px(40.);
const CARD_DIVIDER_HEIGHT: Px = px(2.);
const CARD_EFFECT_ROW_HEIGHT: Px = px(28.);

#[derive(Clone, Copy, PartialEq, State)]
enum CardSide {
    Low,
    High,
}

pub struct OperationPlanModal;

impl Component for OperationPlanModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let modal_wh = Wh::new(MODAL_WIDTH, MODAL_HEIGHT);
        let base_modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        let game_state = use_game_state(ctx);
        let text = game_state.text();
        let choices = game_state.stage_difficulty_choices.clone();

        let (hovered_card, set_hovered_card) = ctx.state::<Option<CardSide>>(|| None);

        let card_content_width = MODAL_WIDTH - PANEL_PADDING * 2.0;
        let card_content_height = MODAL_HEIGHT - TITLE_HEIGHT - PANEL_PADDING * 2.0;
        let card_width = (card_content_width - CARD_GAP) / 2.0;
        let card_wh = Wh::new(card_width, card_content_height);

        // Spring animations (must be at RenderCtx level)
        let low_hovering = *hovered_card == Some(CardSide::Low);
        let high_hovering = *hovered_card == Some(CardSide::High);

        let low_target_scale = if low_hovering {
            Xy::single(1.05)
        } else {
            Xy::single(1.0)
        };
        let high_target_scale = if high_hovering {
            Xy::single(1.05)
        } else {
            Xy::single(1.0)
        };

        let low_animated_xy =
            xy_with_spring(ctx, Xy::new(0.px(), 0.px()), Xy::new(0.px(), 64.px()));
        let low_animated_scale = {
            let s = xy_with_spring(ctx, low_target_scale, Xy::single(0.0));
            Xy::new(s.x.max(0.0001), s.y.max(0.0001))
        };
        let high_animated_xy =
            xy_with_spring(ctx, Xy::new(0.px(), 0.px()), Xy::new(0.px(), 64.px()));
        let high_animated_scale = {
            let s = xy_with_spring(ctx, high_target_scale, Xy::single(0.0));
            Xy::new(s.x.max(0.0001), s.y.max(0.0001))
        };

        let low_card_bg = if low_hovering {
            palette::SURFACE_CONTAINER_HIGHEST
        } else {
            palette::SURFACE_CONTAINER_HIGH
        };
        let high_card_bg = if high_hovering {
            palette::SURFACE_CONTAINER_HIGHEST
        } else {
            palette::SURFACE_CONTAINER_HIGH
        };

        // Modal content (rendered on top per AGENTS.md front-to-back rule)
        ctx.compose(|ctx| {
            let ctx = ctx.translate(base_modal_xy);

            // Phase 1: Layout (compose consumes inner ctx)
            ctx.compose(|ctx| {
                table::vertical([
                    // Title bar
                    table::fixed_no_clip(
                        TITLE_HEIGHT,
                        table::horizontal([
                            table::fixed_no_clip(PANEL_PADDING, |_, _| {}),
                            table::ratio_no_clip(1, |wh, ctx| {
                                let title = text.operation_plan(OperationPlanText::Title);
                                ctx.add(memoized_text((), |mut builder| {
                                    builder
                                        .headline()
                                        .size(typography::FontSize::Medium)
                                        .static_text(title)
                                        .render_left_center(wh.height)
                                }));
                            }),
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &|| set_modal(None),
                                        &|wh, _text_color, ctx| {
                                            ctx.add(
                                                Icon::new(IconKind::Reject)
                                                    .size(IconSize::Large)
                                                    .wh(wh),
                                            );
                                        },
                                    )
                                    .variant(ButtonVariant::Text),
                                );
                            }),
                        ]),
                    ),
                    // Card section
                    table::ratio_no_clip(
                        1,
                        table::padding_no_clip(PANEL_PADDING, |_wh, ctx| {
                            let half = card_wh.to_xy() * 0.5;

                            // === Low difficulty card ===
                            let low_option = choices.low.clone();
                            ctx.translate(low_animated_xy)
                                .translate(half)
                                .scale(low_animated_scale)
                                .translate(-half)
                                .mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                                .compose(|ctx| {
                                    render_card_layout(&ctx, &choices.low, card_wh, text);

                                    ctx.add(
                                        simple_rect(
                                            card_wh,
                                            Color::TRANSPARENT,
                                            0.px(),
                                            Color::TRANSPARENT,
                                        )
                                        .attach_event(
                                            move |event| match event {
                                                Event::MouseMove { event } => {
                                                    if event.is_local_xy_in() {
                                                        set_hovered_card.set(Some(CardSide::Low));
                                                    } else if low_hovering {
                                                        set_hovered_card.set(None);
                                                    }
                                                }
                                                Event::MouseUp { event } => {
                                                    if event.is_local_xy_in() {
                                                        event.stop_propagation();
                                                        let option = low_option.clone();
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
                                        width: card_wh.width,
                                        height: card_wh.height,
                                        texture: PaperTexture::Rough,
                                        variant: PaperVariant::Card,
                                        color: low_card_bg,
                                        shadow: low_hovering,
                                        arrow: None,
                                    });
                                });

                            // === High difficulty card ===
                            let high_option = choices.high.clone();
                            let high_offset_x = card_width + CARD_GAP;
                            ctx.translate((high_offset_x, 0.px()))
                                .translate(high_animated_xy)
                                .translate(half)
                                .scale(high_animated_scale)
                                .translate(-half)
                                .mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                                .compose(|ctx| {
                                    render_card_layout(&ctx, &choices.high, card_wh, text);

                                    ctx.add(
                                        simple_rect(
                                            card_wh,
                                            Color::TRANSPARENT,
                                            0.px(),
                                            Color::TRANSPARENT,
                                        )
                                        .attach_event(
                                            move |event| match event {
                                                Event::MouseMove { event } => {
                                                    if event.is_local_xy_in() {
                                                        set_hovered_card.set(Some(CardSide::High));
                                                    } else if high_hovering {
                                                        set_hovered_card.set(None);
                                                    }
                                                }
                                                Event::MouseUp { event } => {
                                                    if event.is_local_xy_in() {
                                                        event.stop_propagation();
                                                        let option = high_option.clone();
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
                                        width: card_wh.width,
                                        height: card_wh.height,
                                        texture: PaperTexture::Rough,
                                        variant: PaperVariant::Card,
                                        color: high_card_bg,
                                        shadow: high_hovering,
                                        arrow: None,
                                    });
                                });
                        }),
                    ),
                ])(modal_wh, ctx);
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

fn render_card_layout(
    ctx: &ComposeCtx,
    option: &DifficultyOption,
    card_wh: Wh<Px>,
    text: crate::l10n::TextManager,
) {
    let name = option.operation.to_text(&text.locale()).to_string();
    let effects: Vec<crate::game_state::effect::Effect> = option.effects.clone();
    let op_image = operation_kind_image(option.operation);
    let diff_image = difficulty_group_image(option.group);
    let group_color = difficulty_group_color(option.group);

    // top: difficulty icon (최상단)
    let icon_wh = Wh::new(card_wh.width * 0.22, card_wh.height * 0.22);
    let icon_xy = Xy::new(card_wh.width - icon_wh.width - 8.px(), 8.px());
    ctx.add(namui::image(ImageParam {
        rect: Rect::from_xy_wh(icon_xy, icon_wh),
        image: diff_image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: Some(Paint::new(Color::WHITE.with_alpha(128))),
        },
    }));

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

        rows.push(table::fixed_no_clip(CARD_DIVIDER_HEIGHT, |wh, ctx| {
            ctx.add(simple_rect(
                Wh::new(wh.width, CARD_DIVIDER_HEIGHT),
                palette::OUTLINE,
                0.px(),
                Color::TRANSPARENT,
            ));
        }));

        rows.push(table::fixed_no_clip(8.px(), |_, _| {}));

        for effect in effects {
            rows.push(table::fixed_no_clip(
                CARD_EFFECT_ROW_HEIGHT,
                move |wh, ctx| {
                    let effect = effect.clone();
                    let effect_text = crate::l10n::effect::EffectText::Description(effect.clone());
                    let locale = text.locale();
                    ctx.add(memoized_text((), move |mut builder| {
                        builder
                            .headline()
                            .size(typography::FontSize::Small)
                            .color(effect_text_color(&effect))
                            .l10n(effect_text.clone(), &locale)
                            .render_left_center(wh.height)
                    }));
                },
            ));
        }

        rows.push(table::ratio_no_clip(1, |_, _| {}));

        table::padding_no_clip(CARD_PADDING, table::vertical(rows))(card_wh, ctx);
    });

    // bottom: operation background (backmost)
    ctx.add(namui::image(ImageParam {
        rect: Rect::from_xy_wh(Xy::zero(), card_wh),
        image: op_image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: Some(Paint::new(Color::WHITE.with_alpha(128)).set_color_filter(
                ColorFilter::Blend {
                    color: group_color.with_alpha(128),
                    blend_mode: BlendMode::Modulate,
                },
            )),
        },
    }));
}

fn difficulty_group_image(group: DifficultyGroup) -> Image {
    match group {
        DifficultyGroup::StrongTaunt => crate::asset::image::difficulty::difficulty::STRONG_TAUNT,
        DifficultyGroup::Taunt => crate::asset::image::difficulty::difficulty::TAUNT,
        DifficultyGroup::Normal => crate::asset::image::difficulty::difficulty::NORMAL,
        DifficultyGroup::Peace => crate::asset::image::difficulty::difficulty::PEACE,
        DifficultyGroup::BigPeace => crate::asset::image::difficulty::difficulty::BIG_PEACE,
    }
}

fn operation_kind_image(operation: OperationKind) -> Image {
    match operation {
        OperationKind::StrongTaunt => crate::asset::image::difficulty::operation::STRONG_TAUNT,
        OperationKind::Taunt => crate::asset::image::difficulty::operation::TAUNT,
        OperationKind::TeaTime => crate::asset::image::difficulty::operation::TEA_TIME,
        OperationKind::FlowerWatering => {
            crate::asset::image::difficulty::operation::FLOWER_WATERING
        }
        OperationKind::Tribute => crate::asset::image::difficulty::operation::TRIBUTE,
        OperationKind::PeaceGift => crate::asset::image::difficulty::operation::PEACE_GIFT,
        OperationKind::Plead => crate::asset::image::difficulty::operation::PLEAD,
        OperationKind::HandstandApology => {
            crate::asset::image::difficulty::operation::HANDSTAND_APOLOGY
        }
    }
}

fn difficulty_group_color(group: DifficultyGroup) -> Color {
    let base = match group {
        DifficultyGroup::StrongTaunt | DifficultyGroup::Taunt => palette::RED,
        DifficultyGroup::Normal => Color::from_u8(142, 142, 147, 255),
        DifficultyGroup::Peace | DifficultyGroup::BigPeace => palette::BLUE,
    };
    base.with_alpha(128)
}

fn effect_text_color(effect: &crate::game_state::effect::Effect) -> Color {
    if effect.is_positive() {
        palette::BLUE
    } else {
        palette::RED
    }
}

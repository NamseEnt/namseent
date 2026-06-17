mod game_speed_indicator;

use crate::game_state::{Modal, set_modal, use_game_state};
use crate::l10n::Locale;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::tooltip::TooltipContent::Word;
use crate::tooltip::WithHoverArea;
use crate::top_bar::game_speed_indicator::GameSpeedIndicator;
use crate::{
    icon::{Icon, IconKind, IconSize},
    palette,
    theme::button::{Button, ButtonVariant},
    theme::typography::{FontSize, memoized_text},
};
use namui::*;
use namui_prebuilt::table;

const TOP_BAR_HEIGHT: Px = px(48.);
const PADDING: Px = px(8.);

const SETTINGS_BUTTON_SIZE: Px = px(36.);
const SPEED_INDICATOR_WIDTH: Px = px(192.);

const BG_OVERSIZE_H: Px = px(4.);
const BG_OVERSIZE_V: Px = px(4.);
const TOP_BAR_TEXT_STROKE_WIDTH: Px = px(2.);

pub struct TopBar {
    pub wh: Wh<Px>,
}
impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            let locale = game_state.text().locale();
            let stage = game_state.stage;
            let current_hp = game_state.hp.clamp(0.0, game_state.max_hp());
            let max_hp = game_state.max_hp();
            let shield = game_state.shield;
            let gold = game_state.gold;
            let left_dice = game_state.left_dice;
            let max_dice = game_state.max_dice_chance();

            table::horizontal([
                table::ratio_no_clip(
                    1,
                    table::horizontal([
                        table::fixed_no_clip(PADDING, |_, _| {}),
                        table::fit(table::FitAlign::LeftTop, |ctx| {
                            ctx.add(StageText {
                                stage,
                                height: wh.height,
                                locale: &locale,
                            });
                        }),
                        table::fixed_no_clip(PADDING * 4, |_, _| {}),
                        table::fit(table::FitAlign::LeftTop, |ctx| {
                            ctx.add(HealthText {
                                current_hp,
                                max_hp,
                                height: wh.height,
                            });
                        }),
                        table::fit(table::FitAlign::LeftTop, |ctx| {
                            ctx.add(ShieldText {
                                shield,
                                height: wh.height,
                            });
                        }),
                        table::fixed_no_clip(PADDING * 4, |_, _| {}),
                        table::fit(table::FitAlign::LeftTop, |ctx| {
                            ctx.add(GoldText {
                                gold,
                                height: wh.height,
                            });
                        }),
                        table::fixed_no_clip(PADDING * 4, |_, _| {}),
                        table::fit(table::FitAlign::LeftTop, |ctx| {
                            ctx.add(DiceText {
                                left_dice,
                                max_dice,
                                height: wh.height,
                            });
                        }),
                    ]),
                ),
                table::fixed_no_clip(PADDING, |_, _| {}),
                table::fixed_no_clip(
                    SPEED_INDICATOR_WIDTH,
                    table::padding_no_clip(PADDING, |wh, ctx| {
                        ctx.add(GameSpeedIndicator { wh });
                    }),
                ),
                table::fixed_no_clip(SETTINGS_BUTTON_SIZE + PADDING * 2.0, |wh, ctx| {
                    ctx.translate((PADDING, (wh.height - SETTINGS_BUTTON_SIZE) / 2.0))
                        .add(
                            Button::new(
                                Wh::new(SETTINGS_BUTTON_SIZE, SETTINGS_BUTTON_SIZE),
                                &|| set_modal(Some(Modal::Settings)),
                                &|wh, _text_color, ctx| {
                                    ctx.add(
                                        Icon::new(IconKind::Config)
                                            .size(IconSize::Custom { size: px(40.) })
                                            .wh(wh),
                                    );
                                },
                            )
                            .variant(ButtonVariant::Text),
                        );
                }),
            ])(Wh::new(wh.width, TOP_BAR_HEIGHT), ctx);
        });

        ctx.translate((-BG_OVERSIZE_H, -BG_OVERSIZE_V))
            .mouse_cursor(MouseCursor::Standard(StandardCursor::Default))
            .add(PaperContainerBackground {
                width: wh.width + BG_OVERSIZE_H * 2.0,
                height: TOP_BAR_HEIGHT + BG_OVERSIZE_V * 2.0,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Sticky,
                color: palette::SURFACE_CONTAINER_HIGHEST,
                outline_color: None,
                shadow: true,
                arrow: None,
            })
            .attach_event(|event| match event {
                Event::MouseDown { event }
                | Event::MouseUp { event }
                | Event::MouseMove { event }
                    if event.is_local_xy_in() =>
                {
                    event.stop_propagation();
                }
                Event::Wheel { event } if event.is_local_xy_in() => {
                    event.stop_propagation();
                }
                _ => {}
            });
    }
}

struct StageText<'a> {
    stage: usize,
    height: Px,
    locale: &'a Locale,
}
impl<'a> Component for StageText<'a> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            stage,
            height,
            locale,
        } = self;

        ctx.add(memoized_text(&stage, |mut builder| {
            builder
                .headline()
                .size(FontSize::Medium)
                .color(palette::WHITE)
                .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                .l10n(crate::l10n::ui::TopBarText::Stage, locale)
                .text(format!(" {stage}"))
                .render_left_center(height)
        }));
    }
}

struct HealthText {
    current_hp: f32,
    max_hp: f32,
    height: Px,
}
impl Component for HealthText {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            current_hp,
            max_hp,
            height,
        } = self;
        ctx.add(WithHoverArea {
            component_key: "health_text",
            component: memoized_text(&(current_hp, max_hp), |mut builder| {
                builder
                    .headline()
                    .size(FontSize::Medium)
                    .color(palette::RED)
                    .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                    .with_style(|builder| {
                        builder
                            .size(FontSize::Custom { size: px(40.) })
                            .icon(IconKind::Health);
                    })
                    .text(format!(" {current_hp}/{max_hp}"))
                    .render_left_center(height)
            }),
            placement: crate::tooltip::TooltipPlacement::Below,
            content: Word(crate::l10n::word::Word::Health),
        });
    }
}

struct ShieldText {
    shield: f32,
    height: Px,
}
impl Component for ShieldText {
    fn render(self, ctx: &RenderCtx) {
        let Self { shield, height } = self;
        ctx.add(WithHoverArea {
            component_key: "shield_text",
            component: memoized_text(&shield, |mut builder| {
                if shield > 0.0 {
                    builder
                        .headline()
                        .size(FontSize::Medium)
                        .color(palette::GREEN)
                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                        .with_style(|builder| {
                            builder
                                .size(FontSize::Custom { size: px(40.) })
                                .icon(IconKind::Shield);
                        })
                        .text(format!(" {shield}"))
                        .render_left_center(height)
                } else {
                    builder.render_left_top()
                }
            }),
            placement: crate::tooltip::TooltipPlacement::Below,
            content: Word(crate::l10n::word::Word::Shield),
        });
    }
}

struct GoldText {
    gold: usize,
    height: Px,
}
impl Component for GoldText {
    fn render(self, ctx: &RenderCtx) {
        let Self { gold, height } = self;
        ctx.add(WithHoverArea {
            component_key: "gold_text",
            component: memoized_text(&gold, |mut builder| {
                builder
                    .headline()
                    .size(FontSize::Medium)
                    .color(palette::YELLOW)
                    .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                    .with_style(|builder| {
                        builder
                            .size(FontSize::Custom { size: px(40.) })
                            .icon(IconKind::Gold);
                    })
                    .text(format!(" {gold}"))
                    .render_left_center(height)
            }),
            placement: crate::tooltip::TooltipPlacement::Below,
            content: Word(crate::l10n::word::Word::Gold),
        });
    }
}

struct DiceText {
    left_dice: usize,
    max_dice: usize,
    height: Px,
}
impl Component for DiceText {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            left_dice,
            max_dice,
            height,
        } = self;
        ctx.add(WithHoverArea {
            component_key: "dice_text",
            component: memoized_text(&(left_dice, max_dice), |mut builder| {
                builder
                    .headline()
                    .size(FontSize::Medium)
                    .color(palette::BLUE)
                    .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                    .with_style(|builder| {
                        builder
                            .size(FontSize::Custom { size: px(40.) })
                            .icon(IconKind::Refresh);
                    })
                    .text(format!(" {left_dice}/{max_dice}"))
                    .render_left_center(height)
            }),
            placement: crate::tooltip::TooltipPlacement::Below,
            content: Word(crate::l10n::word::Word::Dice),
        });
    }
}

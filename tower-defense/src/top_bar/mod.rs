mod game_speed_indicator;

use crate::game_state::{Modal, set_modal, use_game_state};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
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
            let has_shield = shield > 0.0;
            let gold = game_state.gold;
            let left_dice = game_state.left_dice;
            let max_dice = game_state.max_dice_chance();

            table::horizontal([
                table::ratio_no_clip(1, |wh, ctx| {
                    ctx.add(memoized_text(
                        &(
                            locale, stage, current_hp, max_hp, shield, gold, left_dice, max_dice,
                        ),
                        move |mut builder| {
                            builder
                                .headline()
                                .size(FontSize::Medium)
                                .text("  ")
                                .with_style(|builder| {
                                    builder
                                        .color(palette::WHITE)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .l10n(crate::l10n::ui::TopBarText::Stage, &locale)
                                        .text(format!(" {stage}"));
                                })
                                .text("      ")
                                .with_style(|builder| {
                                    builder
                                        .color(palette::RED)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .size(FontSize::Custom { size: px(40.) })
                                        .icon(IconKind::Health);
                                })
                                .with_style(|builder| {
                                    builder
                                        .color(palette::RED)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .text(format!(" {:.0}/{:.0}", current_hp, max_hp));
                                });

                            if has_shield {
                                builder
                                    .color(palette::RED)
                                    .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                    .text(" (")
                                    .icon(IconKind::Shield)
                                    .text(format!(" {:.0}", shield))
                                    .text(")");
                            }

                            builder
                                .text("      ")
                                .with_style(|builder| {
                                    builder
                                        .color(palette::YELLOW)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .size(FontSize::Custom { size: px(40.) })
                                        .icon(IconKind::Gold);
                                })
                                .with_style(|builder| {
                                    builder
                                        .color(palette::YELLOW)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .text(format!(" {gold}"));
                                })
                                .text("      ")
                                .with_style(|builder| {
                                    builder
                                        .color(palette::BLUE)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .size(FontSize::Custom { size: px(40.) })
                                        .icon(IconKind::Refresh);
                                })
                                .with_style(|builder| {
                                    builder
                                        .color(palette::BLUE)
                                        .stroke(TOP_BAR_TEXT_STROKE_WIDTH, palette::DARK_CHARCOAL)
                                        .text(format!(" {left_dice}/{max_dice}"));
                                })
                                .render_left_center(wh.height)
                        },
                    ));
                }),
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

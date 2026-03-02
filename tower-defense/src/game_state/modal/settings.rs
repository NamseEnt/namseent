use crate::game_state::set_modal;
use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::TopBarText;
use crate::sound::{self, SoundGroup};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::{
    palette,
    typography::{self, memoized_text},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(8.);
const VOLUME_STEP: f32 = 0.05;
const VOLUME_ROW_HEIGHT: Px = px(40.);
const VOLUME_ROW_GAP: Px = px(8.);
const VOLUME_BUTTON_WIDTH: Px = px(32.);
const VOLUME_VALUE_WIDTH: Px = px(56.);

pub struct SettingsModal;

impl Component for SettingsModal {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        let game_state = crate::game_state::use_game_state(ctx);
        let sound_state = sound::use_sound_state(ctx);
        let volume_settings = &sound_state.volume_settings;
        let modal_wh = Wh::new(400.px(), 300.px());
        let modal_xy = ((screen_wh - modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            // 모달 창
            let ctx = ctx.translate(modal_xy);
            ctx.compose(|ctx| {
                table::vertical([
                    table::fixed(
                        TITLE_HEIGHT,
                        table::horizontal([
                            table::fixed(PADDING, |_, _| {}),
                            table::ratio(1, |wh, ctx| {
                                ctx.add(memoized_text((), |mut builder| {
                                    builder
                                        .headline()
                                        .size(typography::FontSize::Medium)
                                        .text(game_state.text().ui(TopBarText::Settings))
                                        .render_left_center(wh.height)
                                }));
                            }),
                            table::fixed(64.px(), |wh, ctx| {
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
                    table::ratio(
                        1,
                        table::padding(PADDING, |wh, ctx| {
                            ctx.add(AutoScrollViewWithCtx {
                                wh,
                                scroll_bar_width: PADDING,
                                content: |ctx| {
                                    let content_width = wh.width - PADDING * 3.0;
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        0.px(),
                                        "Master",
                                        volume_settings.master,
                                        &|| sound::adjust_master_volume(-VOLUME_STEP),
                                        &|| sound::adjust_master_volume(VOLUME_STEP),
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP,
                                        "Effects",
                                        volume_settings.sfx,
                                        &|| {
                                            sound::adjust_group_volume(
                                                SoundGroup::Sfx,
                                                -VOLUME_STEP,
                                            )
                                        },
                                        &|| {
                                            sound::adjust_group_volume(SoundGroup::Sfx, VOLUME_STEP)
                                        },
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        (VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP) * 2.0,
                                        "UI",
                                        volume_settings.ui,
                                        &|| {
                                            sound::adjust_group_volume(SoundGroup::Ui, -VOLUME_STEP)
                                        },
                                        &|| sound::adjust_group_volume(SoundGroup::Ui, VOLUME_STEP),
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        (VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP) * 3.0,
                                        "Ambient",
                                        volume_settings.ambient,
                                        &|| {
                                            sound::adjust_group_volume(
                                                SoundGroup::Ambient,
                                                -VOLUME_STEP,
                                            )
                                        },
                                        &|| {
                                            sound::adjust_group_volume(
                                                SoundGroup::Ambient,
                                                VOLUME_STEP,
                                            )
                                        },
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        (VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP) * 4.0,
                                        "Music",
                                        volume_settings.music,
                                        &|| {
                                            sound::adjust_group_volume(
                                                SoundGroup::Music,
                                                -VOLUME_STEP,
                                            )
                                        },
                                        &|| {
                                            sound::adjust_group_volume(
                                                SoundGroup::Music,
                                                VOLUME_STEP,
                                            )
                                        },
                                    );
                                },
                            });
                        }),
                    ),
                ])(modal_wh, ctx);
            });

            ctx.add(rect(RectParam {
                rect: Wh::new(modal_wh.width, TITLE_HEIGHT).to_rect(),
                style: palette::title_background_style(),
            }));

            ctx.add(rect(RectParam {
                rect: modal_wh.to_rect(),
                style: palette::modal_box_style(),
            }));
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

        ctx.add(
            simple_rect(
                screen_wh,
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(0, 0, 0, 128),
            )
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                set_modal(None);
                event.stop_propagation();
            }),
        );
    }
}

fn render_volume_row(
    ctx: &ComposeCtx,
    width: Px,
    y: Px,
    label: &'static str,
    volume: f32,
    on_minus: &dyn Fn(),
    on_plus: &dyn Fn(),
) {
    let value_percent = (volume * 100.0).round() as i32;
    let controls_width = VOLUME_BUTTON_WIDTH * 2.0 + VOLUME_VALUE_WIDTH + VOLUME_ROW_GAP * 2.0;
    let label_width = (width - controls_width).max(80.px());

    ctx.translate((0.px(), y)).compose(|ctx| {
        ctx.translate((0.px(), 0.px()))
            .add(memoized_text((), |mut builder| {
                builder
                    .paragraph()
                    .size(typography::FontSize::Medium)
                    .text(label)
                    .render_left_center(VOLUME_ROW_HEIGHT)
            }));

        let controls_x = label_width;

        ctx.translate((controls_x, 0.px())).add(
            Button::new(
                Wh::new(VOLUME_BUTTON_WIDTH, VOLUME_ROW_HEIGHT),
                on_minus,
                &|wh, _text_color, ctx| {
                    ctx.add(memoized_text((), |mut builder| {
                        builder
                            .headline()
                            .size(typography::FontSize::Medium)
                            .text("-")
                            .render_center(wh)
                    }));
                },
            )
            .variant(ButtonVariant::Outlined),
        );

        ctx.translate((controls_x + VOLUME_BUTTON_WIDTH + VOLUME_ROW_GAP, 0.px()))
            .add(memoized_text(&value_percent, |mut builder| {
                builder
                    .paragraph()
                    .size(typography::FontSize::Medium)
                    .text(format!("{}%", value_percent))
                    .render_left_center(VOLUME_ROW_HEIGHT)
            }));

        ctx.translate((
            controls_x + VOLUME_BUTTON_WIDTH + VOLUME_ROW_GAP + VOLUME_VALUE_WIDTH + VOLUME_ROW_GAP,
            0.px(),
        ))
        .add(
            Button::new(
                Wh::new(VOLUME_BUTTON_WIDTH, VOLUME_ROW_HEIGHT),
                on_plus,
                &|wh, _text_color, ctx| {
                    ctx.add(memoized_text((), |mut builder| {
                        builder
                            .headline()
                            .size(typography::FontSize::Medium)
                            .text("+")
                            .render_center(wh)
                    }));
                },
            )
            .variant(ButtonVariant::Outlined),
        );
    });
}

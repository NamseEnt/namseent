use crate::game_state::set_modal;
use crate::icon::{Icon, IconKind, IconSize};
use crate::l10n::ui::{SettingsText, TopBarText};
use crate::sound::{self, SoundGroup};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
    slider::Slider,
    typography::{self, memoized_text},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(8.);
const VOLUME_ROW_HEIGHT: Px = px(40.);
const VOLUME_ROW_GAP: Px = px(8.);
const VOLUME_SLIDER_WIDTH: Px = px(200.);
const VOLUME_SLIDER_HEIGHT: Px = px(24.);
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
                                        game_state.text().settings(SettingsText::MasterVolume),
                                        volume_settings.master,
                                        &|v| sound::set_master_volume(v),
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP,
                                        game_state.text().settings(SettingsText::EffectsVolume),
                                        volume_settings.sfx,
                                        &|v| sound::set_group_volume(SoundGroup::Sfx, v),
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        (VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP) * 2.0,
                                        game_state.text().settings(SettingsText::UiVolume),
                                        volume_settings.ui,
                                        &|v| sound::set_group_volume(SoundGroup::Ui, v),
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        (VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP) * 3.0,
                                        game_state.text().settings(SettingsText::AmbientVolume),
                                        volume_settings.ambient,
                                        &|v| sound::set_group_volume(SoundGroup::Ambient, v),
                                    );
                                    render_volume_row(
                                        &ctx,
                                        content_width,
                                        (VOLUME_ROW_HEIGHT + VOLUME_ROW_GAP) * 4.0,
                                        game_state.text().settings(SettingsText::MusicVolume),
                                        volume_settings.music,
                                        &|v| sound::set_group_volume(SoundGroup::Music, v),
                                    );
                                },
                            });
                        }),
                    ),
                ])(modal_wh, ctx);
            });

            ctx.add(PaperContainerBackground {
                width: modal_wh.width,
                height: TITLE_HEIGHT,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Sticky,
                color: palette::SURFACE_CONTAINER_HIGH,
                shadow: false,
                arrow: None,
            });

            ctx.add(PaperContainerBackground {
                width: modal_wh.width,
                height: modal_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Sticky,
                color: palette::SURFACE_CONTAINER,
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

        ctx.add(
            simple_rect(
                screen_wh,
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(0, 0, 0, 128),
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

fn render_volume_row(
    ctx: &ComposeCtx,
    width: Px,
    y: Px,
    label: &'static str,
    volume: f32,
    on_change: &dyn Fn(f32),
) {
    let value_percent = (volume * 100.0).round() as i32;
    let controls_width = VOLUME_SLIDER_WIDTH + VOLUME_VALUE_WIDTH + VOLUME_ROW_GAP;
    let label_width = (width - controls_width).max(80.px());

    ctx.translate((0.px(), y)).compose(|ctx| {
        ctx.translate((0.px(), 0.px()))
            .add(memoized_text((), |mut builder| {
                builder
                    .headline()
                    .size(typography::FontSize::Small)
                    .text(label)
                    .render_left_center(VOLUME_ROW_HEIGHT)
            }));

        let controls_x = label_width;

        ctx.translate((controls_x, (VOLUME_ROW_HEIGHT - VOLUME_SLIDER_HEIGHT) * 0.5))
            .add(Slider::new(
                Wh::new(VOLUME_SLIDER_WIDTH, VOLUME_SLIDER_HEIGHT),
                volume,
                on_change,
            ));

        ctx.translate((controls_x + VOLUME_SLIDER_WIDTH + VOLUME_ROW_GAP, 0.px()))
            .add(memoized_text(&value_percent, |mut builder| {
                builder
                    .paragraph()
                    .size(typography::FontSize::Medium)
                    .text(format!("{}%", value_percent))
                    .render_left_center(VOLUME_ROW_HEIGHT)
            }));
    });
}

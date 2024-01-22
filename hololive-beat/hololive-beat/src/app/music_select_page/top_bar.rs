use crate::app::{
    music::{MusicMetadata, MusicSpeedMap},
    music_select_page::speed_dropdown::SpeedDropdown,
    setting_overlay::open_setting_overlay,
    theme::THEME,
};
use namui::prelude::*;
use namui_prebuilt::{simple_rect, table::hooks::*, typography};

#[component]
pub struct TopBar<'a> {
    pub wh: Wh<Px>,
    pub music: Option<&'a MusicMetadata>,
    pub music_speed_map: Option<&'a MusicSpeedMap>,
}

impl Component for TopBar<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            music,
            music_speed_map,
        } = self;

        const PADDING: Px = px(8.0);

        let (rotation_start_time, set_rotation_start_time) = ctx.state(namui::system::time::now);

        ctx.effect("load font", || {
            namui::spawn(async move {
                namui::typeface::register_typeface(
                    "Fontspring-Demo-hemi_head_rg",
                    &namui::file::bundle::read(
                        "bundle:font/Demo-Hemi Head/Demo_Fonts/Fontspring-Demo-hemi_head_rg.otf",
                    )
                    .await
                    .unwrap(),
                );
            });
        });

        let music_id_sig = ctx.track_eq(&music.as_ref().map(|music| music.id.clone()));
        ctx.effect("Reset text rotation start time ", || {
            music_id_sig.on_effect();
            set_rotation_start_time.set(namui::system::time::now());
        });

        let (group, artist, title) = match music {
            Some(music) => (
                music
                    .groups
                    .iter()
                    .map(|group| group.to_string())
                    .collect::<Vec<_>>()
                    .join(" × "),
                music
                    .artists
                    .iter()
                    .map(|artist| artist.to_string())
                    .collect::<Vec<_>>()
                    .join(" × "),
                music.title.to_string(),
            ),
            None => (String::new(), String::new(), String::new()),
        };

        ctx.compose(|ctx| {
            padding(PADDING, |wh, ctx| {
                horizontal([
                    ratio(1, |wh, ctx| {
                        let font = Font {
                            size: 80.int_px(),
                            name: THEME.font_name.to_string(),
                        };
                        let paint = Paint::new(Color::WHITE);
                        let group_glyph = namui::font::group_glyph(&font, &paint);
                        let dt = namui::system::time::now() - *rotation_start_time;
                        let speed = Per::new((-100).px(), 1.sec());

                        #[derive(Clone, Copy)]
                        struct Item<'a> {
                            text: &'a str,
                            glow_color: Color,
                            width: Px,
                        }

                        const TITLE_GLOW_COLOR: Color = Color::from_u8(0x72, 0xB2, 0xFF, 255);
                        const ARTIST_GLOW_COLOR: Color = Color::from_u8(0xFF, 0xCB, 0x72, 255);
                        const GROUP_GLOW_COLOR: Color = Color::from_u8(0xDC, 0x57, 0xDA, 255);
                        const TITLE_PADDING: Px = px(32.0);

                        let items = [
                            Item {
                                text: &title,
                                glow_color: TITLE_GLOW_COLOR,
                                width: group_glyph.width(&title),
                            },
                            Item {
                                text: &artist,
                                glow_color: ARTIST_GLOW_COLOR,
                                width: group_glyph.width(&artist),
                            },
                            Item {
                                text: &group,
                                glow_color: GROUP_GLOW_COLOR,
                                width: group_glyph.width(&group),
                            },
                        ]
                        .repeat(2);

                        let total_width_including_padding =
                            items.iter().map(|item| item.width).sum::<Px>()
                                + TITLE_PADDING * items.len();

                        let mut left = (speed * dt).floor();

                        for Item {
                            text,
                            glow_color,
                            width,
                        } in items
                        {
                            let rem_euclid_right = (left + width + TITLE_PADDING)
                                .as_f32()
                                .rem_euclid(total_width_including_padding.as_f32())
                                .px();
                            let left_from_right = rem_euclid_right - (width + TITLE_PADDING);

                            ctx.add(typography::effect::glow(
                                text,
                                font.clone(),
                                Xy::new(left_from_right, wh.height / 2.0),
                                paint.clone(),
                                TextAlign::Left,
                                TextBaseline::Middle,
                                Blur::Normal {
                                    sigma: Blur::convert_radius_to_sigma(4.0),
                                },
                                8.px(),
                                glow_color,
                            ));

                            left += width + TITLE_PADDING;
                        }
                    }),
                    fixed(192.px(), |wh, ctx| {
                        ctx.add(SpeedDropdown {
                            wh,
                            music_id: music.map(|music| music.id.as_str()),
                            music_speed_map,
                        });
                    }),
                    fixed(160.px(), |wh, ctx| {
                        ctx.add(SettingButton { wh });
                    }),
                ])(wh, ctx);
            })(wh, ctx);
        });

        ctx.component(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            Color::from_u8(0, 0, 0, 128),
        ));

        ctx.done()
    }
}

#[component]
struct SettingButton {
    wh: Wh<Px>,
}
impl Component for SettingButton {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;

        ctx.component(
            image(ImageParam {
                rect: Rect::zero_wh(wh),
                source: ImageSource::Url {
                    url: Url::parse("bundle:ui/setting.png").unwrap(),
                },
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            })
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !matches!(event.button, Some(MouseButton::Left)) {
                    return;
                }
                if !event.is_local_xy_in() {
                    return;
                }
                open_setting_overlay();
            }),
        );

        ctx.done()
    }
}

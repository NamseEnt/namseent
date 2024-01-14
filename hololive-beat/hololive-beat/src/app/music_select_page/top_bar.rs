use crate::app::{
    music::{MusicMetadata, MusicSpeedMap},
    music_select_page::speed_dropdown::SpeedDropdown,
    setting_overlay::open_setting_overlay,
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
                            name: "Fontspring-Demo-hemi_head_rg".to_string(),
                        };
                        let paint = Paint::new(Color::WHITE);
                        let group_glyph = namui::font::group_glyph(&font, &paint);

                        let title_width = group_glyph.width(&title);
                        let artist_width = group_glyph.width(&artist);

                        ctx.add(typography::effect::glow(
                            title,
                            font.clone(),
                            Xy::new(0.px(), wh.height / 2.0),
                            paint.clone(),
                            TextAlign::Left,
                            TextBaseline::Middle,
                            Blur::Normal {
                                sigma: Blur::convert_radius_to_sigma(12.0),
                            },
                            8.px(),
                            Color::from_u8(255, 0, 255, 255),
                        ));

                        ctx.add(typography::effect::glow(
                            artist,
                            font.clone(),
                            Xy::new(title_width + PADDING * 4, wh.height / 2.0),
                            paint.clone(),
                            TextAlign::Left,
                            TextBaseline::Middle,
                            Blur::Normal {
                                sigma: Blur::convert_radius_to_sigma(12.0),
                            },
                            8.px(),
                            Color::from_u8(255, 184, 76, 255),
                        ));

                        ctx.add(typography::effect::glow(
                            group,
                            font.clone(),
                            Xy::new(title_width + artist_width + PADDING * 8, wh.height / 2.0),
                            paint.clone(),
                            TextAlign::Left,
                            TextBaseline::Middle,
                            Blur::Normal {
                                sigma: Blur::convert_radius_to_sigma(12.0),
                            },
                            8.px(),
                            Color::from_u8(40, 40, 255, 255),
                        ));
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
                open_setting_overlay();
            }),
        );

        ctx.done()
    }
}

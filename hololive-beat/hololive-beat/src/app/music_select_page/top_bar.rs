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
                    fixed(160.px(), |wh, ctx| {
                        ctx.add(typography::text_fit(
                            wh.height,
                            group,
                            Color::WHITE,
                            PADDING,
                        ));
                    }),
                    ratio(1, |wh, ctx| {
                        ctx.add(typography::text_fit(
                            wh.height,
                            artist,
                            Color::WHITE,
                            PADDING,
                        ));
                    }),
                    ratio(3, |wh, ctx| {
                        ctx.add(typography::text_fit(
                            wh.height,
                            title,
                            Color::WHITE,
                            PADDING,
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

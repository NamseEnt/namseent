mod music_carousel;
mod speed_dropdown;
mod top_bar;

use self::{music_carousel::MusicCarousel, top_bar::TopBar};
use super::{
    theme::THEME,
    drummer::Drummer,
    music::{MusicMetadata, MusicSpeedMap},
    play_state::start_game,
};
use namui::prelude::*;
use namui_prebuilt::{simple_rect, table::hooks::*};

#[component]
pub struct MusicSelectPage<'a> {
    pub wh: Wh<Px>,
    pub musics: &'a Vec<MusicMetadata>,
    pub music_speed_map: Option<&'a MusicSpeedMap>,
}

impl Component for MusicSelectPage<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            musics,
            music_speed_map,
        } = self;

        let (selected, _) = ctx.state(|| 0);

        let selected_music = musics.get(*selected);

        ctx.component(TopBar {
            wh: Wh::new(wh.width, 128.px()),
            music: selected_music,
            music_speed_map,
        });

        ctx.compose(|ctx| {
            vertical([
                ratio(2, |_, _| {}),
                ratio(10, |wh, ctx| {
                    let half_height = wh.height * 0.5;
                    let wh = Wh::new(wh.width, half_height);
                    ctx.translate((0.px(), half_height)).add(Decoration { wh });

                    ctx.add(MusicCarousel {
                        wh,
                        musics,
                        selected: *selected,
                    });
                }),
            ])(wh, ctx);
        });

        ctx.compose(|ctx| {
            let Some(music) = selected_music else {
                return;
            };
            ctx.add(image(ImageParam {
                rect: Rect::zero_wh(wh),
                source: ImageSource::Url {
                    url: music.thumbnail_url(),
                },
                style: ImageStyle {
                    fit: ImageFit::Cover,
                    paint: Some(
                        Paint::new(Color::grayscale_alpha_f01(1.0, 0.5)).set_image_filter(
                            ImageFilter::Blur {
                                sigma_xy: Xy::new(4.0, 4.0),
                                tile_mode: None,
                                input: None,
                                crop_rect: None,
                            },
                        ),
                    ),
                },
            }));
        });

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), THEME.background).attach_event(|event| {
                let Event::KeyDown { event } = event else {
                    return;
                };
                if !matches!(event.code, Code::Enter) {
                    return;
                }
                let Some(music) = selected_music else {
                    return;
                };
                start_game(music);
            }),
        );

        ctx.done()
    }
}

#[component]
struct Decoration {
    pub wh: Wh<Px>,
}
impl Component for Decoration {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;

        let title_rect = {
            let width = wh.width * 0.4;
            let height = width * 2 / 3;
            Rect::Xywh {
                x: 0.px(),
                y: wh.height - height,
                width,
                height,
            }
        };

        let drummer_wh = {
            let width = wh.width * 0.6;
            Wh::new(width, width * 0.5)
        };

        ctx.component(image(ImageParam {
            rect: title_rect,
            source: ImageSource::Url {
                url: Url::parse("bundle:ui/title.png").unwrap(),
            },
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));
        ctx.compose(|ctx| {
            ctx.translate((wh.width - drummer_wh.width, wh.height - drummer_wh.height))
                .add(Drummer { wh: drummer_wh });
        });

        ctx.done()
    }
}

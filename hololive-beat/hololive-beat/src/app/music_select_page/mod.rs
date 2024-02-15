mod muisc_preview;
mod music_carousel;
mod speed_dropdown;
mod top_bar;

use self::{muisc_preview::MusicPreview, music_carousel::MusicCarousel, top_bar::TopBar};
use super::{
    drummer::Drummer,
    music::{MusicMetadata, MusicSpeedMap},
    play_state::start_game,
    setting_overlay::open_setting_overlay,
};
use keyframe::{ease, functions::EaseOutCubic, num_traits::Signed};
use namui::{prelude::*, time::since_start};
use namui_prebuilt::table::hooks::*;

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

        let (selected, set_selected) = ctx.state(DelayedSelection::default);
        let (snare, set_snare) = ctx.state::<Option<MediaHandle>>(|| None);
        let (cymbals, set_cymbals) = ctx.state::<Option<MediaHandle>>(|| None);

        let selected_music = selected.get_selected_music(musics);

        let play_snare = || {
            if let Some(snare) = snare.as_ref() {
                snare.clone_independent().unwrap().play().unwrap();
            }
        };
        let play_cymbals = || {
            if let Some(cymbals) = cymbals.as_ref() {
                cymbals.clone_independent().unwrap().play().unwrap();
            }
        };

        ctx.effect("load ui audio", || {
            let snare = namui::system::media::new_media(
                &namui::system::file::bundle::to_real_path("bundle:ui/audio/snare.mp3").unwrap(),
            )
            .unwrap();
            set_snare.set(Some(snare));

            let cymbals = namui::system::media::new_media(
                &namui::system::file::bundle::to_real_path("bundle:ui/audio/cymbals.mp3").unwrap(),
            )
            .unwrap();
            set_cymbals.set(Some(cymbals));
        });

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
                        selected: selected.interpolated_selection(),
                    });
                }),
            ])(wh, ctx);
        });

        ctx.component(MusicPreview {
            wh,
            music: selected_music,
        });

        ctx.on_raw_event(|event| {
            let RawEvent::KeyDown { event } = event else {
                return;
            };
            match event.code {
                Code::Escape => {
                    open_setting_overlay();
                    play_snare();
                }
                Code::Enter => {
                    play_cymbals();
                    let Some(music) = selected_music else {
                        return;
                    };
                    start_game(music);
                }
                Code::ArrowLeft => {
                    set_selected.mutate(|selected| selected.select(selected.selected - 1.0));
                    play_snare();
                }
                Code::ArrowRight => {
                    set_selected.mutate(|selected| selected.select(selected.selected + 1.0));
                    play_snare();
                }
                _ => {
                    if let Some(snare) = snare.as_ref() {
                        snare.clone_independent().unwrap().play().unwrap();
                    }
                }
            }
        });

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
                x: wh.width - width,
                y: wh.height - height,
                width,
                height,
            }
        };

        let drummer_wh = {
            let width = wh.width * 0.6;
            Wh::new(width, width * 0.7)
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
            ctx.translate((0.px(), wh.height - (drummer_wh.height * 0.75)))
                .add(Drummer { wh: drummer_wh });
        });

        ctx.done()
    }
}

#[derive(Debug)]
struct DelayedSelection {
    selected: f32,
    last_selected: (Duration, f32),
    speed: Per<f32, Duration>,
}
impl DelayedSelection {
    fn new(delay: Duration) -> Self {
        Self {
            selected: 0.0,
            last_selected: (Duration::default(), 0.0),
            speed: Per::new(1.0, delay),
        }
    }

    fn select(&mut self, index: f32) {
        let last_selected = self.interpolated_selection();
        self.selected = index;
        self.last_selected = (since_start(), last_selected);
    }

    fn interpolated_selection(&self) -> f32 {
        let (last_changed_at, last_selected) = self.last_selected;
        let elapsed = since_start() - last_changed_at;
        ease(
            EaseOutCubic,
            last_selected,
            self.selected,
            self.speed * elapsed,
        )
    }

    fn get_selected_music<'a>(&self, musics: &'a Vec<MusicMetadata>) -> Option<&'a MusicMetadata> {
        let index = {
            let modulo = self.selected % musics.len() as f32;
            match self.selected.is_positive() {
                true => modulo as usize,
                false => (musics.len() as f32 + modulo) as usize,
            }
        };
        musics.get(index)
    }
}
impl Default for DelayedSelection {
    fn default() -> Self {
        Self::new(Duration::from_millis(300))
    }
}

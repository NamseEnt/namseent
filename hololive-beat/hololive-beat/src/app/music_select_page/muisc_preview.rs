use crate::app::{
    music::MusicMetadata,
    play_state::{PlayState, PLAY_STATE_ATOM},
};
use namui::{prelude::*, time::sleep};
use namui_prebuilt::simple_rect;
use std::ops::Deref;

#[component]
pub struct MusicPreview<'a> {
    pub wh: Wh<Px>,
    pub music: Option<&'a MusicMetadata>,
}
impl Component for MusicPreview<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, music } = self;

        // WARN: may have performance issue
        let music = ctx.track_eq(&music.cloned());
        let (state, _set_state) = ctx.atom(&PLAY_STATE_ATOM);
        let (video, set_video) = ctx.state::<Option<MediaHandle>>(|| None);
        let (audio, set_audio) = ctx.state::<Option<MediaHandle>>(|| None);

        {
            let video = video.deref();
            let audio = audio.deref();
            ctx.effect("change music", || {
                let Some(music) = music.deref() else {
                    return;
                };
                let preview_start_at = music.preview_start_at.sec();
                let preview_end_at = music.preview_end_at.sec();

                // TODO: Fade out
                if let Some(video) = video {
                    video.stop().unwrap();
                }
                if let Some(audio) = audio {
                    audio.stop().unwrap();
                }

                let new_video = music.load_video();
                let new_audio = music.load_audio();

                new_video.seek_to(preview_start_at).unwrap();
                new_audio.seek_to(preview_start_at).unwrap();
                new_video.play().unwrap();
                new_audio.play().unwrap();

                set_video.set(Some(new_video.clone()));
                set_audio.set(Some(new_audio.clone()));

                namui::spawn(async move {
                    let new_video = new_video;
                    let new_audio = new_audio;
                    while new_video.is_playing() && new_audio.is_playing() {
                        sleep(1.sec()).unwrap().await;
                        if new_audio.playback_duration() < preview_end_at {
                            continue;
                        }
                        new_video.seek_to(preview_start_at).unwrap();
                        new_audio.seek_to(preview_start_at).unwrap();
                        new_video.play().unwrap();
                        new_audio.play().unwrap();
                    }
                });
            });
        }

        {
            let video = video.deref();
            let audio = audio.deref();
            ctx.effect("stop music if game started", || {
                let state = state.deref();
                if matches!(state, PlayState::Idle) {
                    return;
                }
                // TODO: Fade out
                if let Some(video) = video {
                    video.stop().unwrap();
                }
                if let Some(audio) = audio {
                    audio.stop().unwrap();
                }
            });
        }

        ctx.compose(|ctx| {
            let Some(video) = video.deref() else {
                return;
            };
            let Some(image) = video.get_image() else {
                return;
            };

            ctx.add(ImageDrawCommand {
                rect: Rect::zero_wh(wh),
                source: ImageSource::ImageHandle {
                    image_handle: image,
                },
                fit: ImageFit::Cover,
                paint: Some(
                    Paint::new(Color::grayscale_alpha_f01(1.0, 0.3)).set_image_filter(
                        ImageFilter::Blur {
                            sigma_xy: Xy::single(Blur::convert_radius_to_sigma(8.0)),
                            tile_mode: None,
                            input: None,
                            crop_rect: None,
                        },
                    ),
                ),
            });
        });

        ctx.component(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK));

        ctx.done()
    }
}

mod mini_map;
mod track;

use mini_map::MiniMap;
use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::Arc;
use track::Track;

pub fn main() {
    namui::start(|| App)
}

#[namui::component]
struct App;

impl Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let screen_wh: Wh<Px> = namui::screen::size().into_type();
        const SAMPLE_RATE: u32 = 44100;
        let (audio, set_audio) = ctx.state(|| None);
        let (window_size, set_window_size) = ctx.state(|| SAMPLE_RATE);
        let (zoom_range, set_zoom_range) = ctx.state(|| 0..*window_size as usize);

        ctx.effect("load raw audio", || {
            namui::spawn(async move {
                let raw_audio: media::audio::RawAudio = namui::media::audio::RawAudio::load(
                    &&namui::system::file::bundle::to_real_path("bundle:resources/snare.opus")
                        .unwrap(),
                    Some(SAMPLE_RATE),
                )
                .await
                .unwrap();

                println!("audio loaded: {:?}", raw_audio);

                set_audio.set(Some(Arc::new(raw_audio)));
            });
        });

        ctx.compose(|ctx| {
            let Some(audio) = audio.as_ref() else {
                return;
            };

            let audio_length = audio.channels[0].len();

            table::hooks::vertical([
                table::hooks::fixed(120.px(), |wh, ctx| {
                    ctx.add(MiniMap {
                        wh,
                        length: audio_length,
                        range: (*zoom_range).clone(),
                    });
                }),
                table::hooks::ratio(
                    1,
                    table::hooks::vertical((0..5).map(|_| {
                        table::hooks::ratio(1, |wh, ctx| {
                            ctx.add(Track {
                                wh,
                                audio: audio.clone(),
                                range: (*zoom_range).clone(),
                            });
                        })
                    })),
                ),
            ])(screen_wh, ctx);

            ctx.add(
                simple_rect(screen_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                    .attach_event(|event| {
                        if let Event::Wheel { event } = event {
                            set_zoom_range.mutate({
                                let window_size = *window_size;
                                move |range| {
                                    let delta = event.delta_xy.y / 120.0 * window_size as f32;
                                    range.start = (range.start as f32 - delta).max(0.0) as usize;
                                    range.end = (range.end as f32 - delta).clamp(
                                        range.start as f32 + window_size as f32,
                                        audio_length as f32,
                                    ) as usize;
                                }
                            });
                        }
                    }),
            );
        });

        ctx.done()
    }
}

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
        let (window_size, set_window_size) = ctx.state(|| SAMPLE_RATE as usize);
        let (start_sample_index, set_start_sample_index) = ctx.state(|| 0_usize);

        let range = *start_sample_index..(*start_sample_index + *window_size);

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

                set_window_size.set(raw_audio.channels[0].len());
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
                        range: range.clone(),
                    });
                }),
                table::hooks::ratio(
                    1,
                    table::hooks::vertical((0..5).map(|_| {
                        table::hooks::ratio(1, |wh, ctx| {
                            ctx.add(Track {
                                wh,
                                audio: audio.clone(),
                                range: range.clone(),
                            });
                        })
                    })),
                ),
            ])(screen_wh, ctx);

            ctx.add(
                simple_rect(screen_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                    .attach_event(|event| {
                        if let Event::Wheel { event } = event {
                            if namui::keyboard::ctrl_press() {
                                // zoom

                                let zoom_delta = 1.0 + event.delta_xy.y / 10.0;
                                let next_window_size = (zoom_delta * *window_size as f32) as usize;

                                set_window_size.set(next_window_size);

                                let cursor_x_ratio = event.local_xy().x / screen_wh.width;

                                let cursor_sample_index = *start_sample_index as f32
                                    + (*window_size as f32 * cursor_x_ratio);

                                let next_start_sample_index = cursor_sample_index
                                    - zoom_delta * (*window_size as f32 * cursor_x_ratio);

                                set_start_sample_index.set(
                                    next_start_sample_index
                                        .clamp(0.0, audio_length as f32 - next_window_size as f32)
                                        as usize,
                                );
                            } else {
                                // move

                                let delta = (event.delta_xy.y / 10.0) * *window_size as f32;
                                set_start_sample_index.set(
                                    ((*start_sample_index as f32 - delta) as usize)
                                        .clamp(0, audio_length - *window_size),
                                );
                            }
                        }
                    }),
            );
        });

        ctx.done()
    }
}

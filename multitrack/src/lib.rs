mod mini_map;
mod track;

use mini_map::MiniMap;
use namui::prelude::*;
use namui_prebuilt::table;
use track::Track;

pub fn main() {
    namui::start(|| App)
}

#[namui::component]
struct App;

impl Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (audio, set_audio) = ctx.state(|| None);
        let sample_rate = 44100;

        ctx.effect("load raw audio", || {
            namui::spawn(async move {
                let raw_audio: media::audio::RawAudio = namui::media::audio::RawAudio::load(
                    &&namui::system::file::bundle::to_real_path("bundle:resources/snare.opus")
                        .unwrap(),
                    Some(sample_rate),
                )
                .await
                .unwrap();

                println!("audio loaded: {:?}", raw_audio);

                set_audio.set(Some(raw_audio));
            });
        });

        let audio_length = 44100;
        let zoom_range = 0..44100 * 100;

        ctx.compose(|ctx| {
            table::hooks::vertical([
                table::hooks::fixed(120.px(), |wh, ctx| {
                    ctx.add(MiniMap {
                        wh,
                        length: audio_length,
                        range: zoom_range.clone(),
                    });
                }),
                table::hooks::ratio(
                    1,
                    table::hooks::vertical((0..5).map(|_| {
                        table::hooks::ratio(1, |wh, ctx| {
                            let Some(audio) = audio.as_ref() else {
                                return;
                            };
                            ctx.add(Track {
                                wh,
                                audio,
                                range: zoom_range.clone(),
                            });
                        })
                    })),
                ),
            ])(namui::screen::size().into_type(), ctx)
        });

        ctx.done()
    }
}

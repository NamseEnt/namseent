mod mini_map;

use mini_map::MiniMap;
use namui::prelude::*;
use namui_prebuilt::table;

pub fn main() {
    namui::start(|| App)
}

#[namui::component]
struct App;

impl Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        // let (audio, set_audio) = ctx.state(|| None);
        // let (waveform_path, set_waveform_path) = ctx.state(Path::new);
        // let sample_rate = 44100;
        // let one_block_px = 1.px();
        // let duration_per_one_block = namui::Duration::from_millis(100);
        // let samples_in_one_block = sample_rate as f32 * duration_per_one_block.as_secs_f32();
        // let width = 500.px();
        // let height = 100.px();

        // ctx.effect("load raw audio", || {
        //     namui::spawn(async move {
        //         let raw_audio: media::audio::RawAudio = namui::media::audio::RawAudio::load(
        //             &&namui::system::file::bundle::to_real_path("bundle:resources/snare.opus")
        //                 .unwrap(),
        //             Some(sample_rate),
        //         )
        //         .await
        //         .unwrap();

        //         set_audio.set(Some(raw_audio));
        //     });
        // });

        // ctx.effect("update waveform_path", || {
        //     let Some(audio) = audio.as_ref().as_ref() else {
        //         return;
        //     };

        //     let mut path = namui::Path::new();
        //     let first_channel = audio.channels.first().unwrap();
        //     for block_index in 0..((width / one_block_px).as_f32() as usize) {
        //         let Some(sample_of_block) = first_channel
        //             .iter()
        //             .skip((block_index as f32 * samples_in_one_block) as usize)
        //             .take(samples_in_one_block as usize)
        //             .map(|sample| sample.abs())
        //             .max_by(|a, b| a.partial_cmp(b).unwrap())
        //         else {
        //             break;
        //         };

        //         let sample_wave_height = height * sample_of_block;
        //         let x = one_block_px * block_index as f32;

        //         path = path.line_to(x, sample_wave_height);
        //     }

        //     set_waveform_path.set(path);
        // });

        // ctx.component(namui::path(
        //     (*waveform_path).clone(),
        //     Paint::new(Color::RED)
        //         .set_style(PaintStyle::Stroke)
        //         .set_stroke_width(1.px()),
        // ));

        let audio_length = 44100;
        let zoom_range = 4410..8820;

        ctx.compose(|ctx| {
            table::hooks::vertical([table::hooks::fixed(120.px(), |wh, ctx| {
                ctx.add(MiniMap {
                    wh,
                    length: audio_length,
                    range: zoom_range,
                });
            })])(namui::screen::size().into_type(), ctx)
        });

        ctx.done()
    }
}

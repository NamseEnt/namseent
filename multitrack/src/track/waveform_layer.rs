use crate::PtrEqArc;
use namui::{media::audio::RawAudio, prelude::*};

#[namui::component]
pub struct WaveformLayer {
    pub wh: Wh<Px>,
    pub audio: PtrEqArc<RawAudio>,
    pub visual_range: std::ops::Range<usize>,
}

impl Component for WaveformLayer {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            audio,
            visual_range,
        } = self;

        let (waveform_path, set_waveform_path) = ctx.state(Path::new);
        let wh = ctx.track_eq(&wh);
        let visual_range = ctx.track_eq(&visual_range);
        let audio = ctx.track_eq(&audio);

        ctx.effect("calculate waveform", || {
            let mut path = namui::Path::new();
            let step = (visual_range.len() as f32 / wh.width.as_f32()).floor() as usize;
            // TODO: Handle when step < 0.
            for (channel_index, steps) in audio
                .channels
                .iter()
                .map(|channel| {
                    channel
                        .iter()
                        .skip(visual_range.start)
                        .take(visual_range.len())
                        .step_by(step)
                })
                .enumerate()
            {
                match channel_index {
                    0 => {
                        for (x, normalized_y) in steps.enumerate() {
                            let y = wh.height / 2.0 - wh.height / 2.0 * *normalized_y;
                            path = path.line_to(px(x as f32), y);
                        }
                    }
                    1 => {
                        for (x, normalized_y) in steps.enumerate().rev() {
                            let y = wh.height / 2.0 + wh.height / 2.0 * *normalized_y;
                            path = path.line_to(px(x as f32), y);
                        }
                    }
                    _ => break,
                }
            }

            set_waveform_path.set(path.line_to(0.px(), 0.px()));
        });

        ctx.component(namui::path(
            (*waveform_path).clone(),
            Paint::new(Color::RED).set_style(PaintStyle::Fill),
        ));
        ctx.component(namui::path(
            (*waveform_path).clone(),
            Paint::new(Color::RED)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(1.px()),
        ));

        ctx.done()
    }
}

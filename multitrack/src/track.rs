use namui::{media::audio::RawAudio, prelude::*};
use namui_prebuilt::simple_rect;
use std::sync::Arc;

#[namui::component]
pub struct Track {
    pub wh: Wh<Px>,
    pub audio: Arc<RawAudio>,
    pub range: std::ops::Range<usize>,
}

impl Component for Track {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, range, audio } = self;

        let (waveform_path, set_waveform_path) = ctx.state(Path::new);
        let wh = ctx.track_eq(&wh);
        let range = ctx.track_eq(&range);

        ctx.effect("calculate waveform", || {
            let mut path = namui::Path::new();
            let step = (range.len() as f32 / wh.width.as_f32()).floor() as usize;
            for (channel_index, steps) in audio
                .channels
                .iter()
                .map(|channel| {
                    channel
                        .as_slice()
                        .get((*range).clone())
                        .unwrap_or(&[])
                        .chunks(step)
                        .map(|chunk| chunk.first().copied().unwrap_or(0.0))
                })
                .enumerate()
            {
                match channel_index {
                    0 => {
                        for (x, normalized_y) in steps.enumerate() {
                            let y = wh.height / 2.0 - wh.height / 2.0 * normalized_y;
                            path = path.line_to(px(x as f32), y);
                        }
                    }
                    1 => {
                        for (x, normalized_y) in steps.enumerate().rev() {
                            let y = wh.height / 2.0 + wh.height / 2.0 * normalized_y;
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

        let background = simple_rect(
            *wh,
            Color::grayscale_f01(0.6),
            1.px(),
            Color::grayscale_f01(0.5),
        );

        ctx.component(background);

        ctx.done()
    }
}

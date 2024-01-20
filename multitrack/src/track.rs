use crate::PtrEqArc;
use namui::{media::audio::RawAudio, prelude::*};
use namui_prebuilt::simple_rect;

#[namui::component]
pub struct Track {
    pub wh: Wh<Px>,
    pub audio: PtrEqArc<RawAudio>,
    pub visual_range: std::ops::Range<usize>,
    pub selection_range: Option<std::ops::Range<usize>>,
}

impl Component for Track {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            audio,
            visual_range,
            selection_range,
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
                        .as_slice()
                        .get((*visual_range).clone())
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

        ctx.component(selection_range.as_ref().map(|selection_range| {
            let left_sample_index = selection_range.start.max(visual_range.start);
            let right_sample_index = selection_range.end.min(visual_range.end);

            let left_x = wh.width
                * ((left_sample_index.saturating_sub(visual_range.start)) as f32
                    / visual_range.len() as f32);
            let width_px = (wh.width
                * ((right_sample_index.saturating_sub(left_sample_index)) as f32
                    / visual_range.len() as f32))
                .min(wh.width);

            namui::translate(
                left_x,
                0.px(),
                simple_rect(
                    Wh::new(width_px, wh.height),
                    Color::from_u8(134, 74, 249, 255),
                    1.px(),
                    Color::from_u8(134, 74, 249, 255),
                ),
            )
        }));

        ctx.component(simple_rect(
            *wh,
            Color::grayscale_f01(0.6),
            1.px(),
            Color::grayscale_f01(0.5),
        ));

        ctx.done()
    }
}

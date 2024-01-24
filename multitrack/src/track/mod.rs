mod note_layer;
mod waveform_layer;

use self::{note_layer::NoteLayer, waveform_layer::WaveformLayer};
use crate::PtrEqArc;
use namui::{media::audio::RawAudio, prelude::*};
use namui_prebuilt::simple_rect;

#[namui::component]
pub struct Track {
    pub wh: Wh<Px>,
    pub audio: PtrEqArc<RawAudio>,
    pub notes: PtrEqArc<Vec<Duration>>,
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
            notes,
        } = self;

        ctx.component(NoteLayer {
            wh,
            visual_range: visual_range.clone(),
            notes,
            sample_rate: audio.audio_config.sample_rate as usize,
        });

        ctx.component(WaveformLayer {
            wh,
            audio,
            visual_range: visual_range.clone(),
        });

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
            wh,
            Color::grayscale_f01(0.6),
            1.px(),
            Color::grayscale_f01(0.5),
        ));

        ctx.done()
    }
}

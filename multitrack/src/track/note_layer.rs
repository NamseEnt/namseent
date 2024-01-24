use crate::PtrEqArc;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

#[namui::component]
pub struct NoteLayer {
    pub wh: Wh<Px>,
    pub visual_range: std::ops::Range<usize>,
    pub notes: PtrEqArc<Vec<Duration>>,
    pub sample_rate: usize,
}

impl Component for NoteLayer {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            visual_range,
            notes,
            sample_rate,
        } = self;

        let visual_duration_start =
            Duration::from_secs_f32(visual_range.start as f32 / sample_rate as f32);
        let visual_duration_end =
            Duration::from_secs_f32(visual_range.end as f32 / sample_rate as f32);
        let visual_duration_length = visual_duration_end - visual_duration_start;

        ctx.compose(|ctx| {
            for &note in notes.iter() {
                if note < visual_duration_start || visual_duration_end < note {
                    continue;
                }

                let x = wh.width * ((note - visual_duration_start) / visual_duration_length);

                ctx.translate(Xy::new(x, 0.px())).add_with_key(
                    note.as_secs_f32().to_string(),
                    simple_rect(
                        Wh::new(2.px(), wh.height),
                        Color::TRANSPARENT,
                        0.px(),
                        Color::GREEN,
                    ),
                );
            }
        });

        ctx.done()
    }
}

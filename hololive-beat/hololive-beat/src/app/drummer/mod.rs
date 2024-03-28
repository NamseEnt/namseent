use keyframe::{ease, functions::EaseOutCubic};
use namui::{math::num::traits::Pow, prelude::*, time::since_start};
use std::f32::consts::PI;

const DRUM: &str = "bundle:ui/drummer/drum.png";
const CHARACTER: [&str; 3] = [
    "bundle:ui/drummer/1.png",
    "bundle:ui/drummer/2.png",
    "bundle:ui/drummer/3.png",
];

const RATIO: f32 = 1.386_409_8;

#[component]
pub struct Drummer {
    pub wh: Wh<Px>,
}

impl Component for Drummer {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let (last_pressed, set_last_pressed) = ctx.state(Duration::default);
        let drum = ctx.image(DRUM);
        let characters = CHARACTER
            .iter()
            .map(|url| ctx.image(url))
            .collect::<Vec<_>>();

        let elapsed = since_start() - *last_pressed;
        let frame_index = calculate_frame_index(elapsed);
        let (character_scale, drum_scale) = calculate_scale(elapsed);
        let image_wh = if wh.width / wh.height > RATIO {
            Wh::new(wh.height * RATIO, wh.height)
        } else {
            Wh::new(wh.width, wh.height / RATIO)
        };
        let image_offset = Xy::new(image_wh.width / -2, 16.px() - image_wh.height);

        ctx.compose(|ctx| {
            let Some(Ok(drum)) = drum.as_ref() else {
                return;
            };
            ctx.translate((wh.width / 2, wh.height))
                .scale(drum_scale)
                .add(ImageDrawCommand {
                    rect: Rect::from_xy_wh(image_offset, image_wh),
                    source: drum.src.clone(),
                    fit: ImageFit::Contain,
                    paint: None,
                });
        });

        ctx.compose(|ctx| {
            let Some(Ok(character)) = characters[frame_index].as_ref() else {
                return;
            };
            ctx.translate((wh.width / 2, wh.height))
                .scale(character_scale)
                .add(ImageDrawCommand {
                    rect: Rect::from_xy_wh(image_offset, image_wh),
                    source: character.src.clone(),
                    fit: ImageFit::Contain,
                    paint: None,
                });
        });

        ctx.on_raw_event(|event| {
            let RawEvent::KeyDown { .. } = event else {
                return;
            };
            set_last_pressed.set(since_start());
        });
    }
}

fn calculate_frame_index(duration: Duration) -> usize {
    let animation_duration = 0.2.sec();
    if duration > animation_duration {
        return 0;
    }
    let progress = (duration / animation_duration).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return 0;
    }
    (ease(EaseOutCubic, 0.0, 3.0, progress) as usize) % 3
}

fn calculate_scale(duration: Duration) -> (Xy<f32>, Xy<f32>) {
    let animation_duration = 0.3.sec();
    if duration > animation_duration * 3 {
        return (Xy::single(1.0), Xy::single(1.0));
    }
    let progress = (duration / animation_duration) as f32;

    let character_x = (1.0_f32 / ((3.0 * progress).pow(4) + 16.0_f32))
        * f32::cos(3.0 * PI * (progress - PI / 128.0).pow(2) + (PI / 2.0));
    let character_y = (1.0_f32 / ((3.0 * progress).pow(4) + 8.0_f32))
        * f32::cos(3.0 * PI * progress.pow(2) - (PI / 2.0));
    let drum_damper = 1.0_f32 - 1.0_f32 / (4.0_f32 * (progress - 0.4).pow(2) + 1.0_f32);
    (
        Xy::new(character_x + 1.0_f32, character_y + 1.0_f32),
        Xy::new(
            drum_damper * character_x + 1.0_f32,
            drum_damper * character_y + 1.0_f32,
        ),
    )
}

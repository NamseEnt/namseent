use crate::card::Engraving;
use crate::theme::palette;
use namui::*;

const SIDE_RATIO: f32 = 1.0 / 2.0;
const OFFSET_RATIO: f32 = 1.0 / 12.0;
const OPACITY: f32 = 0.75;

pub(crate) fn render_engraving_overlay(
    ctx: &RenderCtx,
    wh: Wh<Px>,
    engraving: Option<Engraving>,
    opacity: f32,
) {
    let Some(engraving) = engraving else {
        return;
    };

    let alpha = (255.0 * OPACITY * opacity.clamp(0.0, 1.0)).round() as u8;
    if alpha == 0 {
        return;
    }

    let clip_path = Path::new().add_rrect(wh.to_rect(), palette::ROUND, palette::ROUND);

    ctx.clip(clip_path, ClipOp::Intersect)
        .add(namui::image(ImageParam {
            rect: engraving_rect(wh),
            image: engraving.thumbnail(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: Some(Paint::new(Color::WHITE.with_alpha(alpha))),
            },
        }));
}

fn engraving_rect(wh: Wh<Px>) -> Rect<Px> {
    let side = wh.width * SIDE_RATIO;
    let offset = wh.width * OFFSET_RATIO;
    Rect::from_xy_wh(
        Xy::new(-offset, wh.height - side + offset),
        Wh::new(side, side),
    )
}

use crate::card::Engraving;
use namui::*;

const THUMBNAIL_SIZE: Px = px(28.);
const THUMBNAIL_OFFSET: Px = px(18.);
const OPACITY: f32 = 0.75;

pub(super) struct PopupEngravings {
    pub(super) wh: Wh<Px>,
    pub(super) engravings: Vec<Engraving>,
}

pub(super) fn area_height(engraving_count: usize) -> Px {
    if engraving_count == 0 {
        0.px()
    } else {
        THUMBNAIL_SIZE
    }
}

impl Component for PopupEngravings {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, engravings } = self;
        let alpha = (255.0 * OPACITY).round() as u8;
        let clip_path = Path::new().add_rrect(wh.to_rect(), px(4.), px(4.));

        let ctx = ctx.clip(clip_path, ClipOp::Intersect);
        for (index, engraving) in engravings.into_iter().enumerate() {
            ctx.translate((THUMBNAIL_OFFSET * index as f32, 0.px()))
                .add(image(ImageParam {
                    rect: Rect::Xywh {
                        x: 0.px(),
                        y: 0.px(),
                        width: THUMBNAIL_SIZE,
                        height: THUMBNAIL_SIZE,
                    },
                    image: engraving.thumbnail(),
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: Some(Paint::new(Color::WHITE.with_alpha(alpha))),
                    },
                }));
        }
    }
}

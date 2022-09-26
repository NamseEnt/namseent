use crate::*;

impl RenderingTree {
    pub fn padding(self, padding: impl Padding) -> RenderingTree {
        let ltrb = padding.to_ltrb();
        let bounding_box = self.get_bounding_box();
        render([
            rect(RectParam {
                rect: match bounding_box {
                    Some(bounding_box) => Rect::Xywh {
                        x: (bounding_box.x() + ltrb.left).min(0.px()),
                        y: (bounding_box.y() + ltrb.top).min(0.px()),
                        width: bounding_box.width() + ltrb.left + ltrb.right,
                        height: bounding_box.height() + ltrb.top + ltrb.bottom,
                    },
                    None => Rect::default(),
                },
                style: RectStyle {
                    stroke: None,
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    round: None,
                },
            }),
            translate(ltrb.left, ltrb.top, self),
        ])
    }
}

pub trait Padding {
    fn to_ltrb(&self) -> Ltrb<Px>;
}

impl Padding for Rect<Px> {
    fn to_ltrb(&self) -> Ltrb<Px> {
        self.as_ltrb()
    }
}

impl Padding for Px {
    fn to_ltrb(&self) -> Ltrb<Px> {
        Ltrb {
            left: *self,
            top: *self,
            right: *self,
            bottom: *self,
        }
    }
}

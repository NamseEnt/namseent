use namui::*;

pub mod generated;

pub fn source_rect(left: f32, top: f32, width: f32, height: f32) -> Rect<Px> {
    Rect::Xywh {
        x: px(left),
        y: px(top),
        width: px(width),
        height: px(height),
    }
}

pub fn centered_sprite(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    color: Option<Color>,
) -> ImageSprite {
    let sprite_width = src_rect.width().as_f32();
    let sprite_height = src_rect.height().as_f32();
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos: scale,
            ssin: 0.0,
            tx: cx - px(scale * sprite_width / 2.0),
            ty: cy - px(scale * sprite_height / 2.0),
        },
        color,
    }
}

pub fn centered_rotated_sprite(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    angle_rad: f32,
    color: Option<Color>,
) -> ImageSprite {
    centered_rotated_sprite_with_sin_cos(
        src_rect,
        cx,
        cy,
        scale,
        angle_rad.cos(),
        angle_rad.sin(),
        color,
    )
}

pub(crate) fn centered_rotated_sprite_with_sin_cos(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    cos_angle: f32,
    sin_angle: f32,
    color: Option<Color>,
) -> ImageSprite {
    let sprite_width = src_rect.width().as_f32();
    let sprite_height = src_rect.height().as_f32();
    let scos = scale * cos_angle;
    let ssin = scale * sin_angle;
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos,
            ssin,
            tx: cx - px(scos * sprite_width / 2.0 - ssin * sprite_height / 2.0),
            ty: cy - px(ssin * sprite_width / 2.0 + scos * sprite_height / 2.0),
        },
        color,
    }
}

#[derive(Clone, Copy)]
pub(crate) struct LineSpriteGeometry {
    start_x: Px,
    start_y: Px,
    length: f32,
    thickness: f32,
    cos_angle: f32,
    sin_angle: f32,
}

impl LineSpriteGeometry {
    pub(crate) fn new(
        start_x: Px,
        start_y: Px,
        end_x: Px,
        end_y: Px,
        thickness: f32,
    ) -> Option<Self> {
        let dx = (end_x - start_x).as_f32();
        let dy = (end_y - start_y).as_f32();
        let length = (dx * dx + dy * dy).sqrt();
        if length < 0.001 || thickness < 0.001 {
            return None;
        }

        let angle = dy.atan2(dx);
        Some(Self {
            start_x,
            start_y,
            length,
            thickness,
            cos_angle: angle.cos(),
            sin_angle: angle.sin(),
        })
    }

    pub(crate) fn sprite(
        self,
        src_base_rect: Rect<Px>,
        color: Option<Color>,
    ) -> Option<ImageSprite> {
        self.sprite_with_thickness(src_base_rect, self.thickness, color)
    }

    pub(crate) fn sprite_with_thickness(
        self,
        src_base_rect: Rect<Px>,
        thickness: f32,
        color: Option<Color>,
    ) -> Option<ImageSprite> {
        let base_height = src_base_rect.height().as_f32();
        let base_width = src_base_rect.width().as_f32();
        if base_height < 0.001 || thickness < 0.001 {
            return None;
        }

        let scale = thickness / base_height;
        let src_width = (self.length / scale).min(base_width);
        let scos = scale * self.cos_angle;
        let ssin = scale * self.sin_angle;
        let half_height = base_height / 2.0;
        Some(ImageSprite {
            src_rect: source_rect(
                src_base_rect.left().as_f32(),
                src_base_rect.top().as_f32(),
                src_width,
                base_height,
            ),
            xform: RSXform {
                scos,
                ssin,
                tx: self.start_x + px(ssin * half_height),
                ty: self.start_y - px(scos * half_height),
            },
            color,
        })
    }
}

pub fn line_sprite_from_rect(
    src_base_rect: Rect<Px>,
    start_x: Px,
    start_y: Px,
    end_x: Px,
    end_y: Px,
    thickness: f32,
    color: Option<Color>,
) -> Option<ImageSprite> {
    LineSpriteGeometry::new(start_x, start_y, end_x, end_y, thickness)
        .and_then(|geometry| geometry.sprite(src_base_rect, color))
}

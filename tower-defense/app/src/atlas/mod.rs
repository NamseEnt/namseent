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
    let sprite_width = src_rect.width().as_f32();
    let sprite_height = src_rect.height().as_f32();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();
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

pub fn line_sprite_from_rect(
    src_base_rect: Rect<Px>,
    start_x: Px,
    start_y: Px,
    end_x: Px,
    end_y: Px,
    thickness: f32,
    color: Option<Color>,
) -> Option<ImageSprite> {
    let dx = (end_x - start_x).as_f32();
    let dy = (end_y - start_y).as_f32();
    let length = (dx * dx + dy * dy).sqrt();
    let base_height = src_base_rect.height().as_f32();
    let base_width = src_base_rect.width().as_f32();
    if length < 0.001 || thickness < 0.001 {
        return None;
    }
    let angle = dy.atan2(dx);
    let scale = thickness / base_height;
    let src_width = (length / scale).min(base_width);
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    let scos = scale * cos_angle;
    let ssin = scale * sin_angle;
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
            tx: start_x + px(ssin * half_height),
            ty: start_y - px(scos * half_height),
        },
        color,
    })
}

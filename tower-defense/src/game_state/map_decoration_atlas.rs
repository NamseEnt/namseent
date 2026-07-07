use namui::*;

fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect<Px> {
    Rect::Xywh {
        x: px(x),
        y: px(y),
        width: px(w),
        height: px(h),
    }
}

pub fn centered_sprite(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    color: Option<Color>,
) -> ImageSprite {
    let sw = src_rect.width().as_f32();
    let sh = src_rect.height().as_f32();
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos: scale,
            ssin: 0.0,
            tx: cx - px(scale * sw / 2.0),
            ty: cy - px(scale * sh / 2.0),
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
    let sw = src_rect.width().as_f32();
    let sh = src_rect.height().as_f32();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    let scos = scale * cos_a;
    let ssin = scale * sin_a;
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos,
            ssin,
            tx: cx - px(scos * sw / 2.0 - ssin * sh / 2.0),
            ty: cy - px(ssin * sw / 2.0 + scos * sh / 2.0),
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
    let base_h = src_base_rect.height().as_f32();
    let base_w = src_base_rect.width().as_f32();
    if length < 0.001 || thickness < 0.001 {
        return None;
    }
    let angle = dy.atan2(dx);
    let scale = thickness / base_h;
    let src_w = (length / scale).min(base_w);
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let scos = scale * cos_a;
    let ssin = scale * sin_a;
    let half_h = base_h / 2.0;
    Some(ImageSprite {
        src_rect: Rect::Xywh {
            x: src_base_rect.left(),
            y: src_base_rect.top(),
            width: px(src_w),
            height: px(base_h),
        },
        xform: RSXform {
            scos,
            ssin,
            tx: start_x + px(ssin * half_h),
            ty: start_y - px(scos * half_h),
        },
        color,
    })
}

pub fn decoration_rect(kind: crate::game_state::background::DecorationKind) -> Rect<Px> {
    use crate::game_state::background::DecorationKind;
    match kind {
        DecorationKind::Bush => rect(0.0, 0.0, 128.0, 192.0),
        DecorationKind::Club => rect(128.0, 0.0, 128.0, 192.0),
        DecorationKind::Dia => rect(256.0, 0.0, 128.0, 192.0),
        DecorationKind::Flower => rect(384.0, 0.0, 128.0, 192.0),
        DecorationKind::Heart => rect(512.0, 0.0, 128.0, 192.0),
        DecorationKind::Mushroom => rect(640.0, 0.0, 128.0, 192.0),
        DecorationKind::Rock => rect(768.0, 0.0, 128.0, 192.0),
        DecorationKind::Spade => rect(896.0, 0.0, 128.0, 192.0),
    }
}

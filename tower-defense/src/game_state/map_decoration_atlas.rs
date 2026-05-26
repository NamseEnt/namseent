use namui::*;

fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect<Px> {
    Rect::Xywh {
        x: px(x),
        y: px(y),
        width: px(w),
        height: px(h),
    }
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

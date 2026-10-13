use namui::*;

pub const ATTACK_HIGH_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(1.0),
    y: px(199.0),
    width: px(64.0),
    height: px(32.0),
};

pub const ATTACK_HIGH_TRAIL: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(1.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(34.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(67.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(100.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(133.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(166.0),
        width: px(128.0),
        height: px(32.0),
    },
];

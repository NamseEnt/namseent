use namui::*;

pub const ATTACK_HIGH_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(259.0),
    y: px(294.0),
    width: px(64.0),
    height: px(32.0),
};

pub const ATTACK_HIGH_TRAIL: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(131.0),
        y: px(196.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(261.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(261.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(261.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(294.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(294.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_ONE_PAIR_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(324.0),
    y: px(294.0),
    width: px(32.0),
    height: px(32.0),
};

pub const ATTACK_ONE_PAIR_MUZZLE_RING: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(131.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(66.0),
        y: px(131.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(131.0),
        y: px(131.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(196.0),
        y: px(131.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(261.0),
        y: px(131.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(196.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(66.0),
        y: px(196.0),
        width: px(64.0),
        height: px(64.0),
    },
];

pub const ATTACK_ONE_PAIR_TRAIL: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(1.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(1.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(1.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(66.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(66.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(66.0),
        width: px(128.0),
        height: px(64.0),
    },
];

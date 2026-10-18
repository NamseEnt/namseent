use namui::*;

pub const ATTACK_HIGH_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(1.0),
    y: px(294.0),
    width: px(64.0),
    height: px(32.0),
};

pub const ATTACK_HIGH_TRAIL: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(66.0),
        y: px(294.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(131.0),
        y: px(294.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(196.0),
        y: px(294.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(261.0),
        y: px(294.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(326.0),
        y: px(294.0),
        width: px(64.0),
        height: px(32.0),
    },
];

pub const ATTACK_ONE_PAIR_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(456.0),
    y: px(294.0),
    width: px(32.0),
    height: px(32.0),
};

pub const ATTACK_ONE_PAIR_MUZZLE_RING: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(259.0),
        y: px(66.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(324.0),
        y: px(66.0),
        width: px(64.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(389.0),
        y: px(66.0),
        width: px(64.0),
        height: px(64.0),
    },
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
        x: px(388.0),
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
];

pub const ATTACK_TWO_PAIR_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(391.0),
    y: px(294.0),
    width: px(64.0),
    height: px(32.0),
};

pub const ATTACK_TWO_PAIR_MUZZLE_RING: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(261.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(294.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(327.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(360.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(393.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(426.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
];

pub const ATTACK_TWO_PAIR_TRAIL: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(166.0),
        y: px(196.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(295.0),
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
        x: px(388.0),
        y: px(261.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_TWO_PAIR_TRAIL_RING: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(459.0),
        y: px(131.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(196.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(34.0),
        y: px(196.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(67.0),
        y: px(196.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(100.0),
        y: px(196.0),
        width: px(32.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(133.0),
        y: px(196.0),
        width: px(32.0),
        height: px(64.0),
    },
];

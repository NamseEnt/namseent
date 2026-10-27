use namui::*;

pub const ATTACK_COMMON_EMBER_00: Rect<Px> = Rect::Xywh {
    x: px(430.0),
    y: px(1073.0),
    width: px(16.0),
    height: px(16.0),
};

pub const ATTACK_COMMON_EMBER_01: Rect<Px> = Rect::Xywh {
    x: px(447.0),
    y: px(1073.0),
    width: px(16.0),
    height: px(16.0),
};

pub const ATTACK_COMMON_EMBER_02: Rect<Px> = Rect::Xywh {
    x: px(464.0),
    y: px(1073.0),
    width: px(16.0),
    height: px(16.0),
};

pub const ATTACK_COMMON_STAR_FLARE: Rect<Px> = Rect::Xywh {
    x: px(1.0),
    y: px(843.0),
    width: px(64.0),
    height: px(64.0),
};

pub const ATTACK_COMMON_BEAM_CORE: [Rect<Px>; 3] = [
    Rect::Xywh {
        x: px(331.0),
        y: px(259.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(460.0),
        y: px(259.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(589.0),
        y: px(259.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_BULLET_TRAIL_00: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(34.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(67.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(100.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(133.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(166.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
];

pub const ATTACK_COMMON_BULLET_TRAIL_01: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(259.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(324.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(389.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(454.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(519.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(584.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(649.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_BULLET_TRAIL_02: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(199.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(232.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(265.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(298.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(331.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(364.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
    Rect::Xywh {
        x: px(397.0),
        y: px(1073.0),
        width: px(32.0),
        height: px(16.0),
    },
];

pub const ATTACK_COMMON_DIRECTIONAL_FLASH_00: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(389.0),
        y: px(843.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(518.0),
        y: px(843.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(647.0),
        y: px(843.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(776.0),
        y: px(843.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_DIRECTIONAL_FLASH_01: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(259.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(908.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_DIRECTIONAL_FLASH_02: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(130.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_DIRECTIONAL_FLASH_03: [Rect<Px>; 4] = [
    Rect::Xywh {
        x: px(775.0),
        y: px(941.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_DIRECTIONAL_VOLUME_SMOKE_00: [Rect<Px>; 11] = [
    Rect::Xywh {
        x: px(718.0),
        y: px(259.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(388.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_DIRECTIONAL_VOLUME_SMOKE_01: [Rect<Px>; 9] = [
    Rect::Xywh {
        x: px(388.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(453.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_ENERGY_BALL_ENERGY_BALL: [Rect<Px>; 4] = [
    Rect::Xywh {
        x: px(646.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(518.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_ENERGY_RING_00: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(775.0),
        y: px(130.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(808.0),
        y: px(130.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(841.0),
        y: px(130.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(874.0),
        y: px(130.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(907.0),
        y: px(130.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(34.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
];

pub const ATTACK_COMMON_ENERGY_RING_01: [Rect<Px>; 4] = [
    Rect::Xywh {
        x: px(67.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(100.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(133.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(166.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
];

pub const ATTACK_COMMON_ENERGY_RING_02: [Rect<Px>; 4] = [
    Rect::Xywh {
        x: px(199.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(232.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(265.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(298.0),
        y: px(259.0),
        width: px(32.0),
        height: px(128.0),
    },
];

pub const ATTACK_COMMON_LIGHTNING_BOLT_00: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(259.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(583.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_LIGHTNING_BOLT_01: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_LIGHTNING_BOLT_02: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(646.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(648.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_RESIDUAL_SPARK_00: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(714.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(779.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(844.0),
        y: px(1007.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(66.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_RESIDUAL_SPARK_01: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(131.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(196.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(261.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(326.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(391.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_RESIDUAL_SPARK_02: [Rect<Px>; 5] = [
    Rect::Xywh {
        x: px(456.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(521.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(586.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(651.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(716.0),
        y: px(1040.0),
        width: px(64.0),
        height: px(32.0),
    },
];

pub const ATTACK_COMMON_VAPOR_RING_00: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(66.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(83.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(100.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(117.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(134.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(151.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(168.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_VAPOR_RING_01: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(185.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(202.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(219.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(236.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(253.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(270.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
];

pub const ATTACK_COMMON_VAPOR_RING_02: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(287.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(304.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(321.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(338.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(355.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(372.0),
        y: px(843.0),
        width: px(16.0),
        height: px(64.0),
    },
];

pub const ATTACK_FULL_HOUSE_PROJECTILE: [Rect<Px>; 4] = [
    Rect::Xywh {
        x: px(388.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(713.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_HIGH_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(781.0),
    y: px(1040.0),
    width: px(64.0),
    height: px(32.0),
};

pub const ATTACK_ONE_PAIR_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(911.0),
    y: px(1040.0),
    width: px(32.0),
    height: px(32.0),
};

pub const ATTACK_ROYAL_STRAIGHT_FLUSH_BEAM_CORE: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(778.0),
        width: px(128.0),
        height: px(64.0),
    },
];

pub const ATTACK_ROYAL_STRAIGHT_FLUSH_ENERGY_CHARGE_LINE_00: [Rect<Px>; 6] = [
    Rect::Xywh {
        x: px(388.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(775.0),
        y: px(974.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(1007.0),
        width: px(128.0),
        height: px(32.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(1007.0),
        width: px(128.0),
        height: px(32.0),
    },
];

pub const ATTACK_ROYAL_STRAIGHT_FLUSH_MAGIC_STAR_FLARE_00: [Rect<Px>; 7] = [
    Rect::Xywh {
        x: px(772.0),
        y: px(1.0),
        width: px(128.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(1.0),
        y: px(130.0),
        width: px(128.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(130.0),
        y: px(130.0),
        width: px(128.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(259.0),
        y: px(130.0),
        width: px(128.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(388.0),
        y: px(130.0),
        width: px(128.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(517.0),
        y: px(130.0),
        width: px(128.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(646.0),
        y: px(130.0),
        width: px(128.0),
        height: px(128.0),
    },
];

pub const ATTACK_ROYAL_STRAIGHT_FLUSH_MUZZLE_FLASH: [Rect<Px>; 3] = [
    Rect::Xywh {
        x: px(1.0),
        y: px(1.0),
        width: px(256.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(258.0),
        y: px(1.0),
        width: px(256.0),
        height: px(128.0),
    },
    Rect::Xywh {
        x: px(515.0),
        y: px(1.0),
        width: px(256.0),
        height: px(128.0),
    },
];

pub const ATTACK_TWO_PAIR_PROJECTILE: Rect<Px> = Rect::Xywh {
    x: px(846.0),
    y: px(1040.0),
    width: px(64.0),
    height: px(32.0),
};

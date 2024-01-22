use namui::Color;

pub const THEME: Theme = Theme {
    blue: Color::from_u8(37, 117, 235, 255),
    yellow: Color::from_u8(249, 167, 53, 255),
    purple: Color::from_u8(197, 44, 195, 255),
    red: Color::from_u8(207, 39, 61, 255),
    background: Color::grayscale_u8(128),
    primary: ColorVariant {
        main: Color::from_u8(0x5E, 0xDC, 0xEE, 255),
        dark: Color::from_u8(0x50, 0xbd, 0xcc, 255),
        darker: Color::from_u8(0x0e, 0x22, 0x26, 255),
    },
    font_name: "Fontspring-Demo-hemi_head_rg",
    text: Color::WHITE,
};

pub struct Theme {
    pub blue: Color,
    pub yellow: Color,
    pub purple: Color,
    pub red: Color,
    pub background: Color,
    pub primary: ColorVariant,
    pub font_name: &'static str,
    pub text: Color,
}

pub struct ColorVariant {
    pub main: Color,
    pub dark: Color,
    pub darker: Color,
}

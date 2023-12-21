use namui::Color;

pub const THEME: Theme = Theme {
    background: Palette {
        main: Color::from_u8(36, 37, 42, 255),
        contrast_text: Color::from_u8(0xf9, 0xfb, 0xff, 255),
    },
    surface: Palette {
        main: Color::from_u8(0x45, 0x46, 0x4c, 255),
        contrast_text: Color::from_u8(0xf9, 0xfb, 0xff, 255),
    },
    primary: Palette {
        main: Color::from_u8(45, 85, 255, 255),
        contrast_text: Color::from_u8(0xf9, 0xfb, 0xff, 255),
    },
};

pub struct Palette {
    pub main: Color,
    pub contrast_text: Color,
}

pub struct Theme {
    pub background: Palette,
    pub surface: Palette,
    pub primary: Palette,
}

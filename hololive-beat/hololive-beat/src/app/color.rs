use namui::Color;

pub const THEME: Theme = Theme {
    blue: Color::from_u8(37, 117, 235, 255),
    yellow: Color::from_u8(249, 167, 53, 255),
    purple: Color::from_u8(197, 44, 195, 255),
    red: Color::from_u8(207, 39, 61, 255),
    background: Color::grayscale_u8(128),
};

pub struct Theme {
    pub blue: Color,
    pub yellow: Color,
    pub purple: Color,
    pub red: Color,
    pub background: Color,
}

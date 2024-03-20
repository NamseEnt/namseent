use crate::*;

#[type_derives(-serde::Deserialize)]
pub struct TextDrawCommand {
    pub text: String,
    pub font: Font,
    pub x: Px,
    pub y: Px,
    pub paint: Paint,
    pub align: TextAlign,
    pub baseline: TextBaseline,
    pub max_width: Px,
    pub line_height_percent: Percent,
    pub underline: Option<Paint>,
}

impl TextDrawCommand {
    pub fn line_height_px(&self) -> Px {
        self.font.size.into_px() * self.line_height_percent
    }
}

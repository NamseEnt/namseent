use crate::*;

#[type_derives()]
pub struct TextDrawCommand {
    pub text: String,
    pub font: Font,
    pub x: Px,
    pub y: Px,
    pub paint: Paint,
    pub align: TextAlign,
    pub baseline: TextBaseline,
    pub max_width: Option<Px>,
    pub line_height_percent: Percent,
    pub underline: Option<Paint>,
}

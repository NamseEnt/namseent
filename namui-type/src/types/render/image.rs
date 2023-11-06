use crate::*;

#[type_derives()]
pub struct Image {
    pub wh: Wh<Px>,
    pub src: ImageSource,
}

#[type_derives(Copy)]
pub struct ImageInfo {
    pub alpha_type: AlphaType,
    pub color_type: ColorType,
    pub height: Px,
    pub width: Px,
}

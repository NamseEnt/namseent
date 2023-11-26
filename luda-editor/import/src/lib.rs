use namui_type::{Angle, Percent, Xywh};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Input {
    pub pages: Vec<Page>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Page {
    pub images: Vec<Image>,
    pub texts: Vec<Text>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Image {
    pub url: String,
    pub xywh: Xywh<Percent>,
    pub rotate: Angle,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Text {
    pub content: String,
    pub font: Font,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct Font {
    pub size: usize,
    pub weight: usize,
    pub family: String,
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
}

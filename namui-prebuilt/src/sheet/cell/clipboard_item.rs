use namui::prelude::*;

#[derive(Debug, Clone)]
pub enum ClipboardItem {
    Empty,
    Text(String),
    Image(ImageSource),
}

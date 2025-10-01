use namui::*;

#[derive(Debug, bincode::Decode, bincode::Encode, Clone)]
pub enum ClipboardItem {
    Empty,
    Text(String),
    Image(ImageSource),
}

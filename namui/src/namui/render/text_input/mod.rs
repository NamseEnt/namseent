use crate::namui;
mod render;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Selection {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Clone, Debug)]
pub struct TextInput {
    pub(crate) text: String,
    pub(crate) selection: Option<Selection>,
    pub(crate) id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_fill_color: namui::Color,
    pub border_color: namui::Color,
    pub border_width: f32,
    pub text_align: namui::TextAlign,
    pub text_baseline: namui::TextBaseline,
    pub font_type: namui::FontType,
    pub text_style: namui::TextStyle,
}

impl TextInput {
    pub fn new(
        text: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        background_fill_color: namui::Color,
        border_color: namui::Color,
        border_width: f32,
        text_align: namui::TextAlign,
        text_baseline: namui::TextBaseline,
        font_type: namui::FontType,
        text_style: namui::TextStyle,
    ) -> TextInput {
        TextInput {
            text,
            selection: None,
            id: namui::nanoid(),
            x,
            y,
            width,
            height,
            background_fill_color,
            border_color,
            border_width,
            text_align,
            text_baseline,
            font_type,
            text_style,
        }
    }
}

pub mod text_input_event {
    pub(crate) struct StateChanged {
        pub(crate) text: String,
    }
    pub(crate) struct SelectionChanged {
        pub(crate) selection: Option<super::Selection>,
        pub(crate) id: String,
    }
}

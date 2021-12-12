use crate::engine;
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
    pub background_fill_color: engine::Color,
    pub border_color: engine::Color,
    pub border_width: f32,
    pub text_align: engine::TextAlign,
    pub text_baseline: engine::TextBaseline,
    pub font_type: engine::FontType,
    pub text_style: engine::TextStyle,
}

impl TextInput {
    pub fn new(
        text: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        background_fill_color: engine::Color,
        border_color: engine::Color,
        border_width: f32,
        text_align: engine::TextAlign,
        text_baseline: engine::TextBaseline,
        font_type: engine::FontType,
        text_style: engine::TextStyle,
    ) -> TextInput {
        TextInput {
            text,
            selection: None,
            id: engine::nanoid(),
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

impl engine::Update for TextInput {
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<text_input_event::SelectionChanged>() {
            if event.id != self.id {
                return;
            }

            self.selection = event.selection;
            engine::log(format!("selection changed: {:?}", self.selection));
        }
    }
}

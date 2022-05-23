use crate::{
    namui::{self, manager::TextInputCustomData},
    namui::{manager::TextInputEvent, *},
    RectParam, TextParam,
};
use std::ops::Range;
mod draw_texts_divided_by_selection;
mod get_selection_on_mouse_down;

#[derive(Clone, Debug)]
pub struct TextInput {
    pub(crate) selection: Option<Range<usize>>,
    pub(crate) id: String,
    pub(crate) is_focused: bool,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub rect_param: RectParam,
    pub text_param: TextParam,
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
            selection: None,
            id: namui::nanoid(),
            is_focused: false,
        }
    }
}

impl TextInput {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let custom_props = props.clone();
        (render![
            namui::rect(props.rect_param),
            self.draw_texts_divided_by_selection(props.text_param),
        ])
        .with_custom(TextInputCustomData {
            text_input: self.clone(),
            props: custom_props,
        })
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<TextInputEvent>() {
            match event {
                TextInputEvent::Focus(focus) => {
                    if focus.id == self.id {
                        self.is_focused = true;
                        self.selection = focus.selection.clone();
                    }
                }
                TextInputEvent::Blur => self.is_focused = false,
            }
        }
    }
}

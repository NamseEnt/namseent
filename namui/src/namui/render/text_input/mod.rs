use crate::{
    namui::{self, *},
    RectParam, TextParam,
};
use std::ops::Range;
mod draw_caret;
mod draw_texts_divided_by_selection;
mod get_selection_on_mouse_down;

pub type Selection = Option<Range<usize>>;

#[derive(Clone, Debug)]
pub struct TextInput {
    pub(crate) selection: Selection,
    pub(crate) id: String,
    pub(crate) is_focused: bool,
}
#[derive(Clone, Debug)]
pub struct Props {
    pub rect_param: RectParam,
    pub text_param: TextParam,
}
#[derive(Clone)]
pub struct TextInputCustomData {
    pub text_input: TextInput,
    pub props: Props,
}
pub enum Event {
    Focus(TextInputFocus),
    Blur,
    TextUpdated(TextUpdated),
    SelectionUpdated(SelectionUpdated),
}
pub struct TextInputFocus {
    pub id: String,
    pub selection: Selection,
}
pub struct TextUpdated {
    pub id: String,
    pub text: String,
    pub selection: Selection,
}
pub struct SelectionUpdated {
    pub id: String,
    pub selection: Selection,
}
impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
            selection: None,
            id: crate::nanoid(),
            is_focused: false,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl TextInput {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let custom_props = props.clone();
        (render![
            namui::rect(props.rect_param),
            self.draw_texts_divided_by_selection(props.text_param.clone()),
            self.draw_caret(&props.text_param),
        ])
        .with_custom(TextInputCustomData {
            text_input: self.clone(),
            props: custom_props,
        })
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Focus(focus) => {
                    if focus.id == self.id {
                        self.is_focused = true;
                        self.selection = focus.selection.clone();
                    } else {
                        self.is_focused = false;
                        self.selection = None;
                    }
                }
                Event::Blur => {
                    self.is_focused = false;
                    self.selection = None;
                }
                Event::SelectionUpdated(selection_updated) => {
                    if selection_updated.id == self.id {
                        self.selection = selection_updated.selection.clone();
                    }
                }
                Event::TextUpdated(text_updated) => {
                    if text_updated.id == self.id {
                        self.selection = text_updated.selection.clone();
                    }
                }
                _ => {}
            }
        }
    }
}

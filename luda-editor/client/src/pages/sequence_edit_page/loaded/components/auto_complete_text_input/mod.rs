mod decomposed_string;
mod render;
mod update;

use namui::{prelude::*, text_input::KeyDownEvent};

pub struct AutoCompleteTextInput {
    text_input: TextInput,
    over_item_index: Option<usize>,
}

pub struct Props<
    OnTextChange: Fn(String) + 'static,
    OnEditDone: Fn() + 'static,
    OnKeyDown: Fn(&KeyDownEvent) + 'static,
> {
    pub wh: Wh<Px>,
    pub text: String,
    pub candidates: Vec<String>,
    pub on_text_change: OnTextChange,
    pub on_edit_done: OnEditDone,
    pub on_key_down: OnKeyDown,
}

pub enum Event {}

enum InternalEvent {
    ArrowUpDown { next_index: Option<usize> },
    UpdateItemIndex { over_item_index: Option<usize> },
}

const MAX_SUGGESTIONS: usize = 4;

impl AutoCompleteTextInput {
    pub fn new() -> Self {
        Self {
            text_input: TextInput::new(),
            over_item_index: None,
        }
    }
    pub fn focus(&mut self) {
        self.text_input.focus();
    }

    pub(crate) fn text_input_id(&self) -> Uuid {
        self.text_input.get_id()
    }

    pub(crate) fn blur(&self) {
        self.text_input.blur();
    }
}

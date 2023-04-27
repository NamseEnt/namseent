mod render;
mod update;

use crate::components::context_menu::ContextMenu;

use super::*;
use namui::prelude::*;
use rpc::data::Cut;

pub struct CutEditor {
    selected_target: Option<ClickTarget>,
    character_name_input: auto_complete_text_input::AutoCompleteTextInput,
    text_input: text_input::TextInput,
    image_wysiwyg_editor: wysiwyg_editor::WysiwygEditor,
    context_menu: Option<ContextMenu>,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub cut: Option<&'a Cut>,
    pub cuts: &'a Vec<Cut>,
    pub is_focused: bool,
    pub project_id: Uuid,
}

pub enum Event {
    ChangeCharacterName {
        name: String,
        cut_id: Uuid,
    },
    ChangeCutLine {
        text: String,
        cut_id: Uuid,
    },
    MoveCutRequest {
        cut_id: Uuid,
        to_prev: bool,
        focused: bool,
    },
    Click {
        target: ClickTarget,
    },
    AddNewImage {
        png_bytes: Vec<u8>,
        cut_id: Uuid,
    },
    AddNewCg {
        psd_bytes: Vec<u8>,
        psd_name: String,
        cut_id: Uuid,
    },
}

enum InternalEvent {
    EscapeKeyDown,
    MouseRightButtonDown { global_xy: Xy<Px> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickTarget {
    CharacterName,
    CutText,
}

impl CutEditor {
    pub fn new() -> Self {
        Self {
            selected_target: None,
            character_name_input: auto_complete_text_input::AutoCompleteTextInput::new(),
            text_input: text_input::TextInput::new(),
            image_wysiwyg_editor: wysiwyg_editor::WysiwygEditor::new(),
            context_menu: None,
        }
    }

    pub fn focus_character_name(&mut self) {
        self.focus(ClickTarget::CharacterName);
    }
}

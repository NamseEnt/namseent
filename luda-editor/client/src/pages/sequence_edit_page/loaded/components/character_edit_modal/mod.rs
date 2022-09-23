mod render;
mod update;

use crate::{components::*, storage::get_character_main_image_url};
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct CharacterEditModal {
    cut_id: namui::Uuid,
    character_list_view: list_view::ListView,
    character_id: Option<Uuid>,
    context_menu: Option<context_menu::ContextMenu>,
    editing_text_mode: Option<EditingTextMode>,
    text_input: TextInput,
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub characters: &'a Vec<Character>,
}

pub enum Event {
    CharacterSelected {
        cut_id: namui::Uuid,
        character_id: namui::Uuid,
    },
    AddCharacterClicked,
    CharacterNameChanged {
        character_id: namui::Uuid,
        name: String,
    },
    Close,
}

enum InternalEvent {
    CharacterRightClicked {
        character_id: namui::Uuid,
        mouse_global_xy: Xy<Px>,
        name: String,
    },
    CharacterNameEditClicked {
        character_id: namui::Uuid,
        name: String,
    },
}

enum EditingTextMode {
    CharacterName {
        character_id: namui::Uuid,
        text: String,
    },
}

impl CharacterEditModal {
    pub fn new(cut_id: namui::Uuid, character_id: Option<Uuid>) -> CharacterEditModal {
        CharacterEditModal {
            cut_id,
            character_list_view: list_view::ListView::new(),
            character_id,
            context_menu: None,
            editing_text_mode: None,
            text_input: TextInput::new(),
        }
    }
}

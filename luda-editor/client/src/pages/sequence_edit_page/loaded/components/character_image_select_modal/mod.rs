mod render;
mod update;

use crate::{
    components::*,
    storage::{get_character_image_url, get_character_main_image_url},
};
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct CharacterEditModal {
    cut_id: String,
    character_list_view: list_view::ListView,
    face_expression_list_view: list_view::ListView,
    character_id: Option<String>,
    face_expression_id: Option<String>,
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
        cut_id: String,
        character_id: String,
    },
    FaceExpressionSelected {
        cut_id: String,
        character_id: String,
        face_expression_id: String,
    },
    AddCharacterClicked,
    AddFaceExpressionClicked,
    CharacterNameChanged {
        character_id: String,
        name: String,
    },
    FaceExpressionNameChanged {
        face_expression_id: String,
        name: String,
    },
    Close,
}

enum InternalEvent {
    CharacterRightClicked {
        character_id: String,
        mouse_global_xy: Xy<Px>,
        name: String,
    },
    CharacterNameEditClicked {
        character_id: String,
        name: String,
    },
    FaceExpressionRightClicked {
        mouse_global_xy: Xy<Px>,
        name: String,
        character_id: String,
        face_expression_id: String,
    },
    FaceExpressionNameEditClicked {
        face_expression_id: String,
        name: String,
    },
}

enum EditingTextMode {
    CharacterName {
        character_id: String,
        text: String,
    },
    FaceExpressionName {
        face_expression_id: String,
        text: String,
    },
}

impl CharacterEditModal {
    pub fn new(
        cut_id: String,
        character_id: Option<String>,
        face_expression_id: Option<String>,
    ) -> CharacterEditModal {
        CharacterEditModal {
            cut_id,
            character_list_view: list_view::ListView::new(),
            face_expression_list_view: list_view::ListView::new(),
            character_id,
            face_expression_id,
            context_menu: None,
            editing_text_mode: None,
            text_input: TextInput::new(),
        }
    }
}

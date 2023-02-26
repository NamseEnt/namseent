mod render;
mod update;

use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::Label;

pub struct ImageEditModal {
    image: Option<File>,
    label_text_input: TextInput,
    label_text: String,
    label_list: Vec<Label>,
    purpose: ModalPurpose,
    project_id: namui::Uuid,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Close,
    Error(String),
}

enum InternalEvent {
    ImageChanged { image: File },
    DonePressed,
    LabelInputEnterPressed,
}

#[derive(Clone, Copy)]
pub enum ModalPurpose {
    Add,
    #[allow(dead_code)]
    Edit,
}

impl ImageEditModal {
    pub fn new(purpose: ModalPurpose, project_id: namui::Uuid) -> ImageEditModal {
        ImageEditModal {
            image: None,
            label_text_input: TextInput::new(),
            label_text: String::new(),
            label_list: Vec::new(),
            purpose,
            project_id,
        }
    }
}

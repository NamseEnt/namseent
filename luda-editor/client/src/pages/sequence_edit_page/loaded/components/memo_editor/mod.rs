mod render;
mod update;

use namui::prelude::*;
pub use render::*;
use rpc::data::Memo;
pub use update::*;

pub struct MemoEditor {
    cut_id: Uuid,
    text_input: TextInput,
    text: String,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    OpenMemoEditor { cut_id: Uuid },
    CloseMemoEditor,
    AddCutMemo { cut_id: Uuid, memo: Memo },
}

enum InternalEvent {
    ChangeText(String),
    SaveButtonClicked,
}

impl MemoEditor {
    pub fn new(cut_id: Uuid) -> Self {
        Self {
            cut_id,
            text_input: TextInput::new(),
            text: "".to_string(),
        }
    }
}

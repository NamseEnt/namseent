mod render;
mod update;
use namui::prelude::*;

pub struct CharacterEditor {}

#[derive(Clone, Copy)]
pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    MouseDownOutsideCharacterEditor,
    OpenCharacterEditor,
}

impl CharacterEditor {
    pub fn new() -> Self {
        let image_picker = Self {};
        image_picker
    }
}

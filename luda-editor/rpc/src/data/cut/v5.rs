use super::*;
use crate::data::{Memo, ScreenGraphic};
use crate::Uuid;

#[migration::version(5)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_name: String,
    pub screen_graphics: Vec<ScreenGraphic>,
    pub memos: Vec<Memo>,
}

impl Cut {
    pub fn migrate(previous: v4::Cut) -> Self {
        Self {
            id: previous.id(),
            line: previous.line,
            character_name: previous.character_name,
            screen_graphics: previous.screen_graphics,
            memos: vec![],
        }
    }
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_name: String::new(),
            screen_graphics: Vec::new(),
            memos: Vec::new(),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn duplicate(&self, id: Uuid) -> Self {
        Self {
            id,
            line: self.line.clone(),
            character_name: self.character_name.clone(),
            screen_graphics: self.screen_graphics.clone(),
            memos: self.memos.clone(),
        }
    }
}

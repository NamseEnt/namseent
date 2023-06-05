use super::*;
use crate::data::ScreenGraphic;
use crate::Uuid;

#[migration::version(5)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    pub id: Uuid,

    /// The text that the character speaks in this cut.    
    pub line: String,
    pub character_name: String,
    /// IndexId is not sg's id. it's index id.
    pub screen_graphics: Vec<(IndexId, ScreenGraphic)>,
}

type IndexId = Uuid;

impl Cut {
    pub fn migrate(previous: v4::Cut) -> Self {
        Self {
            id: previous.id(),
            line: previous.line,
            character_name: previous.character_name,
            screen_graphics: previous
                .screen_graphics
                .into_iter()
                .map(|sg| (Uuid::new_v4(), sg))
                .collect(),
        }
    }
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_name: String::new(),
            screen_graphics: vec![],
        }
    }
    pub fn duplicate(&self, id: Uuid) -> Self {
        Self {
            id,
            line: self.line.clone(),
            character_name: self.character_name.clone(),
            screen_graphics: self.screen_graphics.clone(),
        }
    }
}

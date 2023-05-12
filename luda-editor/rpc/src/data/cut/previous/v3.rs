use super::*;
use crate::data::ScreenImage;
use crate::Uuid;

#[migration::version(3)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_name: String,
    pub screen_images: Vec<ScreenImage>,
}

impl Cut {
    pub fn migrate(previous: v2::Cut) -> Self {
        Self {
            id: previous.id(),
            line: previous.line,
            character_name: "".to_string(),
            screen_images: previous
                .screen_images
                .into_iter()
                .filter_map(|image| {
                    image.map(|image| ScreenImage {
                        id: image.id,
                        circumscribed: image.circumscribed.clone(),
                    })
                })
                .collect(),
        }
    }
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_name: String::new(),
            screen_images: Vec::new(),
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
            screen_images: self.screen_images.clone(),
        }
    }
}

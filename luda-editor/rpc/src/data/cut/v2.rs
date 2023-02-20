use super::*;
use crate::data::Circumscribed;
use ::uuid::Uuid;
use namui_type::*;

pub type ScreenImages = [Option<ScreenImage>; 5];

#[migration::version(2)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_name: String,
    pub screen_images: ScreenImages,
}

impl Cut {
    pub fn migrate(previous: v1::Cut) -> Self {
        Self {
            id: previous.id(),
            line: previous.line,
            character_name: "".to_string(),
            screen_images: previous.screen_images.map(|image| {
                image.map(|image| ScreenImage {
                    id: image.id,
                    circumscribed: image.circumscribed.clone(),
                })
            }),
        }
    }
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_name: String::new(),
            screen_images: [None, None, None, None, None],
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenImage {
    pub id: Uuid,
    pub circumscribed: Circumscribed<Percent>,
}
impl ScreenImage {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            circumscribed: Circumscribed {
                center_xy: Xy::new(50.percent(), 50.percent()),
                radius: 50.percent(),
            },
        }
    }
}

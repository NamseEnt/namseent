#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(feature = "client")]
use namui::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectSharedData {
    id: Uuid,
    pub characters: Vec<Character>,
}
impl ProjectSharedData {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            characters: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    id: Uuid,
    pub name: String,
    pub cuts: Vec<Cut>,
}
impl Sequence {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            cuts: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Cut {
    id: Uuid,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_id: Option<Uuid>,
    pub screen_images: [Option<ScreenImage>; 5],
}

impl Cut {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_id: None,
            screen_images: [None, None, None, None, None],
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    id: Uuid,
    pub name: String,
}

#[allow(dead_code)]
impl Character {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            name: String::new(),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Eq, Ord,
)]
pub struct Label {
    pub key: String,
    pub value: String,
}

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd, Eq, Ord,
)]
pub struct ImageWithLabels {
    pub id: Uuid,
    pub url: String,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenImage {
    pub id: Uuid,
    #[cfg(feature = "client")]
    pub circumscribed: Circumscribed<Percent>,
}
impl Default for ScreenImage {
    fn default() -> Self {
        Self {
            id: Default::default(),
            #[cfg(feature = "client")]
            circumscribed: Circumscribed {
                center_xy: Xy::new(50.percent(), 50.percent()),
                radius: 50.percent(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "client")]
pub struct Circumscribed<T> {
    /// (0,0) : left top, (1,1) : right bottom
    pub center_xy: Xy<T>,
    /// 1.0 = 100% of the screen's radius
    pub radius: T,
}

// TODO: implement migration

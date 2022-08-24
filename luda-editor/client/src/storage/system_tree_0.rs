use crdt::*;
use namui::prelude::*;

#[history(version = 0)]
pub struct SystemTree {
    pub sequence: Single<Sequence>,
}
impl SystemTree {
    pub fn new(sequence_id: String, name: String) -> Self {
        Self {
            sequence: Single::new(Sequence::new(sequence_id, name)),
        }
    }
}

#[history]
pub struct Sequence {
    id: String,
    pub name: String,
    pub cuts: List<Cut>,
}
impl Sequence {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            cuts: List::new([]),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[history]
pub struct Cut {
    id: String,
    pub image_clips: List<ImageClip>,
    /// The text that the character speaks in this cut.
    pub line: String,
}

impl Cut {
    pub fn new() -> Self {
        Self {
            id: nanoid(),
            image_clips: List::new([]),
            line: String::new(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[history]
pub struct ImageClip {
    id: String,
    pub duration: Time,
    pub images: List<Image>,
}

#[allow(dead_code)]
impl ImageClip {
    pub fn new(duration: Time) -> Self {
        Self {
            id: nanoid(),
            duration,
            images: List::new([]),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[history]
pub struct Image {
    id: String,
    pub image_path: Option<String>,
    /// against the screen size
    pub circumscribed: Circumscribed,
}

#[allow(dead_code)]
impl Image {
    pub fn new(image_path: Option<String>, circumscribed: Circumscribed) -> Self {
        Self {
            id: nanoid(),
            image_path,
            circumscribed,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Circumscribed {
    pub center: Xy<Percent>,
    pub radius: Percent,
}

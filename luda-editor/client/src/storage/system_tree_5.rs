use crdt::List;
use editor_core::*;
use namui::prelude::*;

#[history(version = 5)]
#[derive(Debug, Clone)]
pub struct SystemTree {
    pub sequence_list: List<Sequence>,
}

#[history]
#[derive(Debug, Clone)]
pub struct Sequence {
    id: String,
    pub name: String,
    pub cuts: List<Cut>,
}
impl Sequence {
    pub fn new(name: String) -> Self {
        Self {
            id: nanoid(),
            name,
            cuts: List::new([]),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[history]
#[derive(Debug, Clone)]
pub struct Cut {
    id: String,
    pub image_clips: List<ImageClip>,
}

impl Cut {
    pub fn new() -> Self {
        Self {
            id: nanoid(),
            image_clips: List::new([]),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[history]
#[derive(Debug, Clone)]
pub struct ImageClip {
    id: String,
    pub duration: Time,
    pub images: List<Image>,
}

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
#[derive(Debug, Clone)]
pub struct Image {
    id: String,
    pub image_path: Option<String>,
    /// against the screen size
    pub circumscribed: Circumscribed,
}

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

pub fn migrate(prev: super::system_tree_4::SystemTree) -> SystemTree {
    SystemTree {
        sequence_list: prev
            .sequence_list
            .iter()
            .map(|sequence| Sequence {
                id: sequence.id().to_string(),
                name: sequence.name.clone(),
                cuts: sequence
                    .cuts
                    .iter()
                    .map(|cut| Cut {
                        id: cut.id().to_string(),
                        image_clips: cut
                            .image_clips
                            .iter()
                            .map(|clip| ImageClip {
                                id: clip.id().to_string(),
                                duration: clip.duration,
                                images: clip
                                    .images
                                    .iter()
                                    .map(|image| Image {
                                        id: image.id().to_string(),
                                        image_path: image.image_path.clone(),
                                        circumscribed: Circumscribed {
                                            center: image.center_xy_percent,
                                            radius: image.wh_percent.length(),
                                        },
                                    })
                                    .collect(),
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

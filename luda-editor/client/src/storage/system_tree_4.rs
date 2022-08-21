use crdt::List;
use editor_core::*;
use namui::prelude::*;

#[history(version = 4)]
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
    pub center_xy_percent: Xy<Percent>,
    pub wh_percent: Wh<Percent>,
}

impl Image {
    pub fn new(
        image_path: Option<String>,
        center_xy_percent: Xy<Percent>,
        wh_percent: Wh<Percent>,
    ) -> Self {
        Self {
            id: nanoid(),
            image_path,
            center_xy_percent,
            wh_percent,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

pub fn migrate(prev: super::system_tree_3::SystemTree) -> SystemTree {
    SystemTree {
        sequence_list: prev
            .sequence_list
            .iter()
            .map(|sequence| Sequence {
                id: sequence.id.clone(),
                name: sequence.name.clone(),
                cuts: sequence
                    .cuts
                    .iter()
                    .map(|cut| Cut {
                        id: cut.id.clone(),
                        image_clips: List::new([]),
                    })
                    .collect(),
            })
            .collect(),
    }
}

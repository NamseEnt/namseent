mod cg;
mod cut;
mod cut_update_action;
mod sequence_update_action;

pub use cg::*;
pub use cut::*;
pub use cut_update_action::*;
use namui_type::{Percent, PercentExt, Uuid, Xy};
pub use sequence_update_action::*;
use std::collections::HashSet;

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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    pub id: Uuid,
    pub name: String,
    pub cuts: Vec<Cut>,
}
impl Sequence {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            cuts: vec![],
        }
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

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Circumscribed<T> {
    /// (0,0) : left top, (1,1) : right bottom
    pub center_xy: Xy<T>,
    /// 1.0 = 100% of the screen's radius
    pub radius: T,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ScreenGraphic {
    Image(ScreenImage),
    Cg(ScreenCg),
}
impl ScreenGraphic {
    pub fn circumscribed_mut(&mut self) -> &mut Circumscribed<Percent> {
        match self {
            Self::Image(screen_image) => &mut screen_image.circumscribed,
            Self::Cg(screen_cg) => &mut screen_cg.circumscribed,
        }
    }
    pub fn circumscribed(&self) -> Circumscribed<Percent> {
        match self {
            Self::Image(screen_image) => screen_image.circumscribed,
            Self::Cg(screen_cg) => screen_cg.circumscribed,
        }
    }
    pub fn set_circumscribed(&mut self, circumscribed: Circumscribed<Percent>) {
        match self {
            Self::Image(screen_image) => screen_image.circumscribed = circumscribed,
            Self::Cg(screen_cg) => screen_cg.circumscribed = circumscribed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScreenCg {
    pub id: Uuid,
    pub name: String,
    pub parts: Vec<ScreenCgPart>,
    pub circumscribed: Circumscribed<Percent>,
}
impl ScreenCg {
    pub fn new(cg_file: &CgFile) -> Self {
        Self {
            id: cg_file.id,
            name: cg_file.name.clone(),
            parts: cg_file
                .parts
                .iter()
                .map(|part| ScreenCgPart::new(&part.name, part.selection_type))
                .collect(),
            circumscribed: Circumscribed {
                center_xy: Xy::new(50.percent(), 50.percent()),
                radius: 50.percent(),
            },
        }
    }
    pub fn part(&self, part_name: &str) -> Option<&ScreenCgPart> {
        self.parts.iter().find(|part| part.name() == part_name)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ScreenCgPart {
    Single {
        name: String,
        variant_name: Option<String>,
    },
    Multi {
        name: String,
        variant_names: HashSet<String>,
    },
    AlwaysOn {
        name: String,
    },
}
impl ScreenCgPart {
    pub fn new(name: &str, selection_type: PartSelectionType) -> Self {
        match selection_type {
            PartSelectionType::Single => Self::Single {
                name: name.to_string(),
                variant_name: None,
            },
            PartSelectionType::Multi => Self::Multi {
                name: name.to_string(),
                variant_names: HashSet::new(),
            },
            PartSelectionType::AlwaysOn => Self::AlwaysOn {
                name: name.to_string(),
            },
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::Single { name, .. } => name,
            Self::Multi { name, .. } => name,
            Self::AlwaysOn { name } => name,
        }
    }
    pub fn is_not_selected(&self) -> bool {
        match self {
            ScreenCgPart::Single { variant_name, .. } => variant_name.is_none(),
            ScreenCgPart::Multi { variant_names, .. } => variant_names.is_empty(),
            ScreenCgPart::AlwaysOn { .. } => false,
        }
    }

    pub fn is_variant_selected(&self, variant_name: &str) -> bool {
        match self {
            ScreenCgPart::Multi { variant_names, .. } => variant_names.contains(variant_name),
            ScreenCgPart::Single {
                variant_name: self_variant_name,
                ..
            } => self_variant_name
                .as_ref()
                .map(|n| n == variant_name)
                .unwrap_or(false),
            ScreenCgPart::AlwaysOn { .. } => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Memo {
    pub id: Uuid,
    pub content: String,
    pub cut_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
}
impl Memo {
    pub fn new(
        id: Uuid,
        content: impl ToString,
        cut_id: Uuid,
        user_id: Uuid,
        user_name: impl ToString,
    ) -> Self {
        Self {
            id,
            content: content.to_string(),
            cut_id,
            user_id,
            user_name: user_name.to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Page<Item> {
    pub items: Vec<Item>,
    pub next_page_key: Option<String>,
}

mod cg;
mod cut;
mod cut_update_action;
mod screen_graphic;
mod sequence_update_action;

pub use cg::*;
pub use cut::*;
pub use cut_update_action::*;
use namui_type::{Percent, PercentExt, Uuid, Xy};
pub use screen_graphic::*;
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq)]
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
    pub fn unselect(&mut self) {
        match self {
            ScreenCgPart::Single {
                name: _,
                variant_name,
            } => {
                *variant_name = None;
            }
            ScreenCgPart::Multi {
                name: _,
                variant_names,
            } => {
                variant_names.clear();
            }
            ScreenCgPart::AlwaysOn { .. } => unreachable!(),
        }
    }
    pub fn turn_on(&mut self, cg_part_variant_name: String) {
        match self {
            ScreenCgPart::Single {
                name: _,
                variant_name,
            } => {
                *variant_name = Some(cg_part_variant_name);
            }
            ScreenCgPart::Multi {
                name: _,
                variant_names,
            } => {
                variant_names.insert(cg_part_variant_name);
            }
            ScreenCgPart::AlwaysOn { .. } => unreachable!(),
        }
    }
    pub fn turn_off(&mut self, cg_part_variant_name: String) {
        match self {
            ScreenCgPart::Single {
                name: _,
                variant_name,
            } => {
                *variant_name = None;
            }
            ScreenCgPart::Multi {
                name: _,
                variant_names,
            } => {
                variant_names.remove(&cg_part_variant_name);
            }
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

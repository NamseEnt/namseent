use super::previous::v0;
use crate::data::{CgFile, Circumscribed, ScreenCgPart};
use namui_type::*;

#[migration::version(1)]
#[derive(Debug, Clone, PartialEq)]
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
    pub fn rotation(&self) -> Angle {
        match self {
            Self::Image(screen_image) => screen_image.rotation,
            Self::Cg(screen_cg) => screen_cg.rotation,
        }
    }
    pub fn set_rotation(&mut self, rotation: Angle) {
        match self {
            Self::Image(screen_image) => screen_image.rotation = rotation,
            Self::Cg(screen_cg) => screen_cg.rotation = rotation,
        }
    }
    pub fn migrate(previous: v0::ScreenGraphic) -> Self {
        match previous {
            v0::ScreenGraphic::Image(image) => ScreenGraphic::Image(ScreenImage::migrate(image)),
            v0::ScreenGraphic::Cg(cg) => ScreenGraphic::Cg(ScreenCg::migrate(cg)),
        }
    }
}

#[migration::version(1)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenImage {
    pub id: Uuid,
    pub circumscribed: Circumscribed<Percent>,
    pub rotation: Angle,
}
impl ScreenImage {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            circumscribed: Circumscribed {
                center_xy: Xy::new(50.percent(), 50.percent()),
                radius: 50.percent(),
            },
            rotation: Angle::Degree(25.0),
        }
    }
    pub fn migrate(previous: v0::ScreenImage) -> Self {
        Self {
            id: previous.id,
            circumscribed: previous.circumscribed,
            rotation: Angle::Degree(0.0),
        }
    }
}

#[migration::version(1)]
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenCg {
    pub id: Uuid,
    pub name: String,
    pub parts: Vec<ScreenCgPart>,
    pub circumscribed: Circumscribed<Percent>,
    pub rotation: Angle,
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
            rotation: Angle::Degree(0.0),
        }
    }
    pub fn part(&self, part_name: &str) -> Option<&ScreenCgPart> {
        self.parts.iter().find(|part| part.name() == part_name)
    }
    pub fn migrate(previous: v0::ScreenCg) -> Self {
        Self {
            id: previous.id,
            name: previous.name,
            parts: previous.parts,
            circumscribed: previous.circumscribed,
            rotation: Angle::Degree(0.0),
        }
    }
}

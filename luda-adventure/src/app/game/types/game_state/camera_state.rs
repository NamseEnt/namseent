use crate::app::game::Tile;
use namui::{Uuid, Xy};

pub struct CameraState {
    pub subject: CameraSubject,
}

#[derive(Clone, Copy)]
pub enum CameraSubject {
    Object { id: Uuid },
    Xy { xy: Xy<Tile> },
}

impl CameraState {
    pub fn new() -> Self {
        Self::new_with_subject(CameraSubject::Xy { xy: Xy::zero() })
    }
    pub fn new_with_subject(subject: CameraSubject) -> Self {
        Self { subject }
    }
}

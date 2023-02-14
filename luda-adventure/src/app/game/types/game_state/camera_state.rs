use crate::app::game::Tile;
use namui::{Uuid, Xy};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CameraState {
    pub subject: CameraSubject,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
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

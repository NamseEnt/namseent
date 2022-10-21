use crate::app::game::Tile;
use namui::{Uuid, Xy};

pub struct CameraState {
    subject: CameraSubject,
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

    pub fn subject(&self) -> CameraSubject {
        self.subject
    }

    pub fn set_subject(&mut self, subject: CameraSubject) {
        self.subject = subject;
    }
}

use super::*;
use crate::simple_error_impl;
use namui_type::Uuid;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SequenceUpdateAction {
    InsertCut {
        cut: Cut,
        after_cut_id: Option<Uuid>,
    },
    RenameSequence {
        name: String,
    },
    DeleteCut {
        cut_id: Uuid,
    },
    SplitCutText {
        cut_id: Uuid,
        new_cut_id: Uuid,
        split_at: usize,
    },
    MoveCut(MoveCutAction),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MoveCutAction {
    pub cut_id: Uuid,
    pub after_cut_id: Option<Uuid>,
    _prevent_direct_creation: PhantomData<()>,
}
impl MoveCutAction {
    pub fn new(cut_id: Uuid, after_cut_id: Option<Uuid>) -> Result<Self, MoveCutActionCreateError> {
        if after_cut_id == Some(cut_id) {
            return Err(MoveCutActionCreateError::MoveAfterItself);
        }
        Ok(Self {
            cut_id,
            after_cut_id,
            _prevent_direct_creation: PhantomData,
        })
    }
}
impl From<MoveCutAction> for SequenceUpdateAction {
    fn from(value: MoveCutAction) -> Self {
        Self::MoveCut(value)
    }
}
simple_error_impl!(MoveCutActionCreateError);
#[derive(Debug)]
pub enum MoveCutActionCreateError {
    MoveAfterItself,
}

use super::*;
use crate::simple_error_impl;
use namui_type::Uuid;

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
    cut_id: Uuid,
    after_cut_id: Option<Uuid>,
}
impl MoveCutAction {
    pub fn new(cut_id: Uuid, after_cut_id: Option<Uuid>) -> Result<Self, MoveCutActionCreateError> {
        if after_cut_id == Some(cut_id) {
            return Err(MoveCutActionCreateError::MoveAfterItself);
        }
        Ok(Self {
            cut_id,
            after_cut_id,
        })
    }
    pub fn cut_id(&self) -> Uuid {
        self.cut_id
    }
    pub fn after_cut_id(&self) -> Option<Uuid> {
        self.after_cut_id
    }
}
impl Into<SequenceUpdateAction> for MoveCutAction {
    fn into(self) -> SequenceUpdateAction {
        SequenceUpdateAction::MoveCut(self)
    }
}
simple_error_impl!(MoveCutActionCreateError);
#[derive(Debug)]
pub enum MoveCutActionCreateError {
    MoveAfterItself,
}

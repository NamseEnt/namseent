use super::*;
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
    MoveCut {
        cut_id: Uuid,
        after_cut_id: Option<Uuid>,
    },
    SplitCutText {
        cut_id: Uuid,
        new_cut_id: Uuid,
        split_at: usize,
    },
}

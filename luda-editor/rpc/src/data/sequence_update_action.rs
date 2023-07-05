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
}

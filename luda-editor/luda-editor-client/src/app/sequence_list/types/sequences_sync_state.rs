use crate::app::types::Sequence;
use linked_hash_map::LinkedHashMap;
use namui::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub enum SequencesSyncStateDetail {
    Loading,
    Loaded {
        title_sequence_map: LinkedHashMap<String, Arc<Sequence>>,
    },
    Failed {
        error: String,
    },
}

#[derive(Clone)]
pub struct SequenceSyncState {
    pub started_at: Time,
    pub detail: SequencesSyncStateDetail,
}

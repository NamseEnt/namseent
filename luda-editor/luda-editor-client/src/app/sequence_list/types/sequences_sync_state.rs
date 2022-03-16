use crate::app::types::Sequence;
use linked_hash_map::LinkedHashMap;
use std::{sync::Arc, time::Duration};

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
    pub started_at: Duration,
    pub detail: SequencesSyncStateDetail,
}

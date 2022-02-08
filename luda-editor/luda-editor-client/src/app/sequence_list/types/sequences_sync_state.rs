use crate::app::types::Sequence;
use std::{collections::BTreeMap, sync::Arc, time::Duration};

#[derive(Clone)]
pub enum SequencesSyncStateDetail {
    Loading,
    Loaded {
        title_sequence_map: BTreeMap<String, Arc<Sequence>>,
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

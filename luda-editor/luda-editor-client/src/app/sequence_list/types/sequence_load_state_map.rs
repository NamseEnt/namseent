use crate::app::types::Sequence;
use std::{collections::HashMap, sync::Arc, time::Duration};

#[derive(Clone)]
pub enum SequenceLoadStateDetail {
    Loading,
    Loaded { sequence: Arc<Sequence> },
    Failed { error: String },
}

#[derive(Clone)]
pub struct SequenceLoadState {
    pub started_at: Duration,
    pub detail: SequenceLoadStateDetail,
}

pub type SequenceLoadStateMap = HashMap<String, SequenceLoadState>;

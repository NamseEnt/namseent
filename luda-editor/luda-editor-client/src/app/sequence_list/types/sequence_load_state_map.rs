use std::{collections::HashMap, time::Duration};

use crate::app::types::Sequence;

#[derive(Clone)]
pub enum SequenceLoadStateDetail {
    Loading,
    Loaded { sequence: Sequence },
    Failed { error: String },
}

#[derive(Clone)]
pub struct SequenceLoadState {
    pub started_at: Duration,
    pub detail: SequenceLoadStateDetail,
}

pub type SequenceLoadStateMap = HashMap<String, SequenceLoadState>;

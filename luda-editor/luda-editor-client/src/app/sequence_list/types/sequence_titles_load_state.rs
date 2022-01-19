use std::time::Duration;

#[derive(Clone)]
pub enum SequenceTitlesLoadStateDetail {
    Loading,
    Loaded { titles: Vec<String> },
    Failed { error: String },
}

#[derive(Clone)]
pub struct SequenceTitlesLoadState {
    pub started_at: Duration,
    pub detail: SequenceTitlesLoadStateDetail,
}

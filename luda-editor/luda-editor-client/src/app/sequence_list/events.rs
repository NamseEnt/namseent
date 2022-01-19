use super::types::{SequenceLoadState, SequenceTitlesLoadState};

pub enum SequenceListEvent {
    SequenceLoadStateUpdateEvent {
        path: String,
        state: Option<SequenceLoadState>,
    },
    SequenceLoadEvent {
        path: String,
    },
    SequenceTitlesLoadStateUpdateEvent {
        state: SequenceTitlesLoadState,
    },
    SequenceTitlesLoadEvent,
    ScrolledEvent {
        scroll_y: f32,
    },
}

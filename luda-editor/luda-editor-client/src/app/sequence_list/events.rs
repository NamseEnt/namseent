use super::types::SequenceLoadState;

pub enum SequenceListEvent {
    SequenceLoadStateUpdateEvent {
        path: String,
        state: Option<SequenceLoadState>,
    },
    SequenceLoadEvent {
        path: String,
    },
}

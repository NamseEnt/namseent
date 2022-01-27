use super::types::{SequenceLoadState, SequenceTitlesLoadState};

pub enum SequenceListEvent {
    SequenceLoadStateUpdateEvent {
        path: String,
        state: Option<SequenceLoadState>,
    },
    SequenceTitleButtonClickedEvent {
        path: String,
    },
    SequenceTitlesLoadStateUpdateEvent {
        state: SequenceTitlesLoadState,
    },
    SequenceReloadTitlesButtonClickedEvent,
    ScrolledEvent {
        scroll_y: f32,
    },
    PreviewSliderMovedEvent {
        path: String,
        progress: f32,
    },
}

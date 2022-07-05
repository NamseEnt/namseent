use super::types::SequenceSyncState;
use namui::prelude::*;

pub enum SequenceListEvent {
    SequenceTitleButtonClickedEvent {
        title: String,
    },
    /// NOTE: 'Loading local sequences' also use this event.
    SequencesSyncStateUpdateEvent {
        state: SequenceSyncState,
    },
    SyncSequencesButtonClickedEvent,
    ScrolledEvent {
        scroll_y: Px,
    },
    PreviewSliderMovedEvent {
        title: String,
        progress: f32,
    },
}

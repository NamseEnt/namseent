use super::types::{SequenceOpenState, SequenceSyncState};
use crate::app::types::Sequence;
use namui::prelude::*;
use std::sync::Arc;

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
    SequenceOpenButtonClickedEvent {
        title: String,
        sequence: Arc<Sequence>,
    },
    SequenceOpenStateChangedEvent {
        title: String,
        state: SequenceOpenState,
    },
}

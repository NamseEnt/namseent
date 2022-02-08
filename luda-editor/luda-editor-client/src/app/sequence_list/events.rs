use super::types::SequenceSyncState;
use crate::app::types::Sequence;
use std::{collections::BTreeMap, sync::Arc};

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
        scroll_y: f32,
    },
    PreviewSliderMovedEvent {
        title: String,
        progress: f32,
    },
}

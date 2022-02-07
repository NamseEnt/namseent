pub enum SheetSequenceSyncerStatus {
    Idle,
    Syncing,
    Failed(String),
    Successful,
}

pub(super) struct SheetSequenceSyncer {
    status: SheetSequenceSyncerStatus,
}

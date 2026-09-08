use crate::game_state::{GameState, mutate_game_state};
use std::sync::{Mutex, OnceLock};

pub struct SavedSnapshot {
    pub stage: usize,
    bytes: Vec<u8>,
}

fn snapshots_storage() -> &'static Mutex<Vec<SavedSnapshot>> {
    static STORAGE: OnceLock<Mutex<Vec<SavedSnapshot>>> = OnceLock::new();
    STORAGE.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn save_snapshot_from_state(game_state: &GameState) {
    let mut guard = snapshots_storage().lock().expect("snapshot mutex poisoned");
    guard.push(SavedSnapshot {
        stage: game_state.stage,
        bytes: crate::game_state::persistence::encode(game_state),
    });
}

pub fn save_current_snapshot() {
    mutate_game_state(|gs| {
        save_snapshot_from_state(gs);
    });
}

pub fn list_snapshots() -> Vec<(usize, usize)> {
    let guard = snapshots_storage().lock().expect("snapshot mutex poisoned");
    guard
        .iter()
        .enumerate()
        .map(|(idx, snap)| (idx, snap.stage))
        .collect()
}

pub fn restore_snapshot(index: usize) {
    mutate_game_state(move |gs| {
        let restored = snapshots_storage()
            .lock()
            .expect("snapshot mutex poisoned")
            .get(index)
            .map(|saved| saved.bytes.clone());

        if let Some(bytes) = restored {
            crate::game_state::persistence::load_into(gs, &bytes)
                .expect("debug snapshot should restore");
        }
    });
}

pub fn clear_snapshots() {
    let mut guard = snapshots_storage().lock().expect("snapshot mutex poisoned");
    guard.clear();
}

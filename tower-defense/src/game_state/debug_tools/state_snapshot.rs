use crate::game_state::{GameState, mutate_game_state};
use std::sync::{Mutex, OnceLock};

pub struct SavedSnapshot {
    pub stage: usize,
    pub state: GameState,
}

fn snapshots_storage() -> &'static Mutex<Vec<SavedSnapshot>> {
    static STORAGE: OnceLock<Mutex<Vec<SavedSnapshot>>> = OnceLock::new();
    STORAGE.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn save_snapshot_from_state(game_state: &GameState) {
    let mut guard = snapshots_storage().lock().expect("snapshot mutex poisoned");
    guard.push(SavedSnapshot {
        stage: game_state.stage,
        state: game_state.clone_for_debug(),
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
            .map(|saved| saved.state.clone_for_debug());

        if let Some(restored) = restored {
            *gs = restored;
        }
    });
}

pub fn clear_snapshots() {
    let mut guard = snapshots_storage().lock().expect("snapshot mutex poisoned");
    guard.clear();
}

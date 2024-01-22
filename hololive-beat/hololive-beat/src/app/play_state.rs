use super::{
    music::MusicMetadata,
    note::{Instrument, Note},
};
use crate::app::note::load_notes;
use futures::join;
use namui::{Atom, Duration, DurationExt, FullLoadOnceAudio, MediaHandle};
use std::collections::HashSet;

pub static PLAY_STATE_ATOM: Atom<PlayState> = Atom::uninitialized_new();

#[derive(Debug)]
pub enum PlayState {
    Idle,
    Loading {
        music: MusicMetadata,
    },
    Loaded {
        music: MusicMetadata,
        loaded: LoadedData,
        judge_context: JudgeContext,
        play_time_state: PlayTimeState,
    },
}
impl Default for PlayState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug)]
pub enum PlayTimeState {
    Playing {
        started_time: Duration,
        played_time: Duration,
    },
    Paused {
        played_time: Duration,
    },
    Ended,
}

pub fn start_game(music: &MusicMetadata) {
    PLAY_STATE_ATOM.mutate({
        let music = music.clone();
        move |state| {
            if !matches!(state, PlayState::Idle) {
                unreachable!("play state is not idle");
            }
            *state = PlayState::Loading {
                music: music.clone(),
            };
        }
    });

    let music = music.clone();
    namui::spawn(async move {
        let loaded = load_music(&music).await;
        let loaded_music_id = music.id.clone();
        PLAY_STATE_ATOM.mutate(move |state| {
            let PlayState::Loading { music } = state else {
                namui::log!("play state is not loading");
                return;
            };
            if music.id != loaded_music_id {
                namui::log!("music id is not matched. cancel loading");
                return;
            }

            loaded.music.play().unwrap();
            loaded.video.play().unwrap();

            *state = PlayState::Loaded {
                music: music.clone(),
                loaded,
                judge_context: JudgeContext::new(),
                play_time_state: PlayTimeState::Playing {
                    started_time: namui::time::since_start(),
                    played_time: Duration::from_secs(0),
                },
            };
        });
    });
}

pub fn pause_game(now: Duration) {
    PLAY_STATE_ATOM.mutate(move |state| {
        let PlayState::Loaded {
            loaded,
            play_time_state,
            ..
        } = state
        else {
            return;
        };
        let PlayTimeState::Playing {
            started_time,
            played_time,
        } = play_time_state
        else {
            return;
        };

        loaded.music.pause().unwrap();
        loaded.video.pause().unwrap();
        let played_time = now - *started_time + *played_time;

        *play_time_state = PlayTimeState::Paused { played_time };
    });
}

pub fn resume_game(now: Duration) {
    PLAY_STATE_ATOM.mutate(move |state| {
        let PlayState::Loaded {
            loaded,
            play_time_state,
            ..
        } = state
        else {
            return;
        };
        let PlayTimeState::Paused { played_time } = play_time_state else {
            return;
        };

        loaded.music.play().unwrap();
        loaded.video.play().unwrap();

        *play_time_state = PlayTimeState::Playing {
            started_time: now,
            played_time: *played_time,
        };
    });
}

pub fn stop_game() {
    PLAY_STATE_ATOM.mutate(move |state| {
        let PlayState::Loaded {
            loaded,
            play_time_state,
            ..
        } = state
        else {
            return;
        };

        loaded.music.stop().unwrap();
        loaded.video.stop().unwrap();

        *play_time_state = PlayTimeState::Ended;
    });
}

pub fn restart_game() {
    PLAY_STATE_ATOM.mutate(move |state| {
        let PlayState::Loaded {
            loaded,
            judge_context,
            play_time_state,
            ..
        } = state
        else {
            namui::log!("play state is not loaded");
            return;
        };

        loaded.music.seek_to(0.sec()).unwrap();
        loaded.video.seek_to(0.sec()).unwrap();
        loaded.music.play().unwrap();
        loaded.video.play().unwrap();

        *judge_context = JudgeContext::new();
        *play_time_state = PlayTimeState::Playing {
            started_time: namui::time::since_start(),
            played_time: Duration::from_secs(0),
        };
    });
}

#[derive(Debug)]
pub struct LoadedData {
    pub notes: Vec<Note>,
    pub note_sounds: Vec<FullLoadOnceAudio>,
    pub music: MediaHandle,
    pub video: MediaHandle,
}
async fn load_music(music: &MusicMetadata) -> LoadedData {
    let id = music.id.clone();
    let music = load_media(&format!("bundle:musics/{id}/{id}.opus"));
    let video = load_media(&format!("bundle:musics/{id}/{id}.mp4"));
    let (notes, kick, cymbals, snare) = join!(
        load_notes(&id),
        load_full_load_once_audio(format!("bundle:musics/{id}/kick.opus")),
        load_full_load_once_audio(format!("bundle:musics/{id}/cymbals.opus")),
        load_full_load_once_audio(format!("bundle:musics/{id}/snare.opus")),
    );

    let note_sounds = {
        notes
            .iter()
            .map(|note| {
                let instrument = match note.instrument {
                    Instrument::Kick => &kick,
                    Instrument::Snare => &snare,
                    Instrument::Cymbals => &cymbals,
                };
                instrument.slice(note.start_time..note.end_time).unwrap()
            })
            .collect()
    };

    LoadedData {
        notes,
        note_sounds,
        music,
        video,
    }
}
fn load_media(path: &str) -> MediaHandle {
    let path = namui::system::file::bundle::to_real_path(path).unwrap();
    namui::system::media::new_media(&path).unwrap()
}
async fn load_full_load_once_audio(path: String) -> FullLoadOnceAudio {
    let path = namui::system::file::bundle::to_real_path(path.as_str()).unwrap();
    namui::system::media::new_full_load_once_audio(&path)
        .await
        .unwrap()
}

#[derive(Debug)]
pub struct JudgeContext {
    pub perfect_count: usize,
    pub good_count: usize,
    pub miss_count: usize,
    pub max_combo: usize,
    pub combo: usize,
    pub last_judged_note_index: Option<usize>,
    pub judged_note_index: HashSet<usize>,
}
impl JudgeContext {
    pub fn new() -> Self {
        Self {
            perfect_count: 0,
            good_count: 0,
            miss_count: 0,
            max_combo: 0,
            combo: 0,
            last_judged_note_index: None,
            judged_note_index: HashSet::new(),
        }
    }
}

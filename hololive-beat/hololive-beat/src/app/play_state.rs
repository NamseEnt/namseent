use super::{
    music::MusicMetadata,
    note::{Instrument, Note},
    MUSIC_BEST_SCORE_MAP_ATOM,
};
use crate::app::note::load_notes;
use futures::join;
use namui::{Atom, Duration, DurationExt, FullLoadOnceAudio, MediaHandle};
use std::collections::HashSet;

pub const PERFECT_SCORE: usize = 97;
pub const GOOD_SCORE: usize = 71;

pub static PLAY_STATE_ATOM: Atom<PlayState> = Atom::uninitialized();

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
            judge_context,
            music,
            ..
        } = state
        else {
            return;
        };

        loaded.music.stop().unwrap();
        loaded.video.stop().unwrap();
        judge_context.calculate_rank();
        let score = judge_context.score;
        let id = music.id.clone();
        MUSIC_BEST_SCORE_MAP_ATOM.mutate(move |music_best_score_map| {
            let Some(music_best_score_map) = music_best_score_map else {
                panic!("music best score map is not loaded");
            };
            let best_score = music_best_score_map.get(&id).max(score);
            music_best_score_map.set(id, best_score);
            let music_best_score_map = music_best_score_map.clone();
            namui::spawn(async move {
                music_best_score_map.save().await;
            });
        });

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
    let video = music.load_video();
    let music = music.load_audio();
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
    pub score: usize,
    pub rank: Rank,
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
            score: 0,
            rank: Rank::D,
        }
    }

    fn calculate_rank(&mut self) {
        self.score = self.perfect_count * PERFECT_SCORE + self.good_count * GOOD_SCORE;
        let note_count = self.perfect_count + self.good_count + self.miss_count;
        let max_score = note_count * PERFECT_SCORE;
        let perfection_rate = self.score as f32 / max_score as f32;
        let rank = if perfection_rate >= 0.97 {
            Rank::S
        } else if perfection_rate >= 0.9 {
            Rank::A
        } else if perfection_rate >= 0.8 {
            Rank::B
        } else if perfection_rate >= 0.7 {
            Rank::C
        } else if perfection_rate >= 0.6 {
            Rank::D
        } else {
            Rank::F
        };
        self.rank = rank;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    S,
    A,
    B,
    C,
    D,
    F,
}
impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank_str = match self {
            Rank::S => "S",
            Rank::A => "A",
            Rank::B => "B",
            Rank::C => "C",
            Rank::D => "D",
            Rank::F => "F",
        };
        write!(f, "{}", rank_str)
    }
}

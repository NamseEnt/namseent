use super::*;

pub enum PlayingStatus {
    Playing {
        start_at: Time,
        playback_time_on_start: Time,
    },
    Paused {
        playback_time: Time,
    },
}

impl PlayingStatus {
    pub fn new() -> Self {
        PlayingStatus::Paused {
            playback_time: Time::Ms(0.0),
        }
    }
    pub fn get_playback_time(&self) -> Time {
        match self {
            &PlayingStatus::Playing {
                start_at,
                playback_time_on_start,
            } => {
                let now = Time::now();
                let elapsed = now - start_at;
                playback_time_on_start + elapsed
            }
            &PlayingStatus::Paused { playback_time } => playback_time,
        }
    }
    pub fn toggle_play(&mut self) {
        match self {
            PlayingStatus::Playing { .. } => {
                *self = PlayingStatus::Paused {
                    playback_time: self.get_playback_time(),
                }
            }
            PlayingStatus::Paused { playback_time } => {
                *self = PlayingStatus::Playing {
                    start_at: Time::now(),
                    playback_time_on_start: *playback_time,
                }
            }
        }
    }
    pub fn set_playback_time(&mut self, time: Time) {
        match self {
            PlayingStatus::Playing { .. } => {
                *self = PlayingStatus::Playing {
                    start_at: Time::now(),
                    playback_time_on_start: time,
                }
            }
            PlayingStatus::Paused { playback_time } => {
                *playback_time = time;
            }
        }
    }
}

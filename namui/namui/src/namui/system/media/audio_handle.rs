use super::audio_context::AudioCommand;
use namui_type::*;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize},
    Arc,
};

/// Q. What happens if you clone an AudioHandle and play it?
/// A. Let's say it's independent. If you don't want it to be independent, you have to wrap it in Arc.
///
/// Q. What happens if you play an already playing AudioHandle again?
/// A. Nothing happens. (If you play an already playing AudioHandle again, it is ignored.)
#[derive(Debug)]
pub(crate) struct AudioHandle {
    id: usize,
    audio_buffer_core_id: usize,
    audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
    play_state: PlayState,
    last_playback_playing: Option<Arc<AtomicBool>>,
}

#[derive(Debug)]
enum PlayState {
    Playing,
    Paused,
    Stopped,
}

impl AudioHandle {
    pub(crate) fn new(
        audio_buffer_core_id: usize,
        audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
    ) -> Self {
        Self {
            id: get_new_audio_handle_id(),
            audio_buffer_core_id,
            audio_command_tx,
            play_state: PlayState::Stopped,
            last_playback_playing: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        (match &self.play_state {
            PlayState::Playing => true,
            PlayState::Paused | PlayState::Stopped => false,
        }) && self
            .last_playback_playing
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn play(&mut self, start_at: Instant, start_offset: Duration) {
        if self.is_playing() {
            return;
        }

        self.play_state = PlayState::Playing;
        let last_playback_playing = Arc::new(AtomicBool::new(true));
        self.last_playback_playing = Some(last_playback_playing.clone());

        self.audio_command_tx
            .send(AudioCommand::Play {
                audio_handle_id: self.id,
                audio_buffer_core_id: self.audio_buffer_core_id,
                is_playing: last_playback_playing,
                start_at,
                start_offset,
            })
            .expect("failed to send AudioCommand::Play");
    }

    pub fn stop(&mut self) {
        if let PlayState::Playing = self.play_state {
            self.last_playback_playing = None;

            self.audio_command_tx
                .send(AudioCommand::Stop {
                    audio_handle_id: self.id,
                })
                .expect("failed to send AudioCommand::Stop");
        }
        self.play_state = PlayState::Stopped;
    }

    /// Current version of `pause` doesn't guarantee that the audio will be paused immediately.
    /// Also it doesn't guarantee that the audio will start from the same position when it is resumed.
    pub(crate) fn pause(&mut self) {
        let PlayState::Playing = self.play_state else {
            return;
        };
        self.play_state = PlayState::Paused;
        self.last_playback_playing = None;

        self.audio_command_tx
            .send(AudioCommand::Stop {
                audio_handle_id: self.id,
            })
            .expect("failed to send AudioCommand::Stop");
    }

    pub(crate) fn seek_to(&mut self, playback_duration: Duration) {
        let PlayState::Playing = self.play_state else {
            return;
        };
        self.audio_command_tx
            .send(AudioCommand::SeekTo {
                audio_handle_id: self.id,
                offset: playback_duration,
            })
            .expect("failed to send AudioCommand::Stop");
    }
}

fn get_new_audio_handle_id() -> usize {
    static AUDIO_HANDLE_ID: AtomicUsize = AtomicUsize::new(0);
    AUDIO_HANDLE_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

impl Drop for AudioHandle {
    fn drop(&mut self) {
        self.audio_command_tx
            .send(AudioCommand::DecreaseAudioRefCount {
                audio_buffer_core_id: self.audio_buffer_core_id,
            })
            .expect("failed to send DecreaseAudioRefCount");
    }
}

impl Clone for AudioHandle {
    fn clone(&self) -> Self {
        self.audio_command_tx
            .send(AudioCommand::IncreaseAudioRefCount {
                audio_buffer_core_id: self.audio_buffer_core_id,
            })
            .expect("failed to send IncreaseAudioRefCount");

        Self {
            id: get_new_audio_handle_id(),
            audio_buffer_core_id: self.audio_buffer_core_id,
            audio_command_tx: self.audio_command_tx.clone(),
            play_state: PlayState::Stopped,
            last_playback_playing: None,
        }
    }
}

use super::audio_context::AudioCommand;
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
    is_playing: bool,
    last_playback_playing: Option<Arc<AtomicBool>>,
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
            is_playing: false,
            last_playback_playing: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
            && self
                .last_playback_playing
                .as_ref()
                .unwrap()
                .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn play(&mut self) {
        if self.is_playing {
            return;
        }
        self.is_playing = true;
        let last_playback_playing = Arc::new(AtomicBool::new(true));
        self.last_playback_playing = Some(last_playback_playing.clone());

        self.audio_command_tx
            .send(AudioCommand::Play {
                audio_handle_id: self.id,
                audio_buffer_core_id: self.audio_buffer_core_id,
                is_playing: last_playback_playing,
            })
            .expect("failed to send AudioCommand::Play");
    }

    pub fn stop(&mut self) {
        if !self.is_playing {
            return;
        }
        self.is_playing = false;
        self.last_playback_playing = None;

        self.audio_command_tx
            .send(AudioCommand::Stop {
                audio_handle_id: self.id,
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
            is_playing: false,
            last_playback_playing: None,
        }
    }
}

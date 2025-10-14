use super::InterleavedAllSamples;
use crate::system::InitResult;
use std::sync::{Arc, Mutex, OnceLock};

pub(super) fn init() -> InitResult {
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Audio {
    audio_id: usize,
    _destroyer: Arc<AudioDestroyer>,
}

static AUDIO_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PLAYBACK_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Audio {
    pub(crate) fn new(all_samples: InterleavedAllSamples) -> Self {
        let interleaved_samples = all_samples.into_iter().flatten().collect::<Vec<f32>>();
        let planar_samples = {
            let mut left_samples = Vec::with_capacity(interleaved_samples.len() / 2);
            let mut right_samples = Vec::with_capacity(interleaved_samples.len() / 2);
            for i in 0..interleaved_samples.len() / 2 {
                left_samples.push(interleaved_samples[i * 2]);
                right_samples.push(interleaved_samples[i * 2 + 1]);
            }
            [left_samples, right_samples].concat()
        };
        let audio_id = AUDIO_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        unsafe {
            let samples_bytes = std::slice::from_raw_parts(
                planar_samples.as_ptr() as *const u8,
                planar_samples.len() * std::mem::size_of::<f32>(),
            );
            _audio_init(audio_id, samples_bytes.as_ptr(), samples_bytes.len());
        }
        Audio {
            audio_id,
            _destroyer: Arc::new(AudioDestroyer { audio_id }),
        }
    }
    pub fn play(&self) -> PlayHandle {
        self.play_impl(false)
    }
    pub fn play_repeat(&self) -> PlayHandle {
        self.play_impl(true)
    }
    pub fn play_and_forget(&self) {
        unsafe {
            _audio_play_and_forget(self.audio_id);
        }
    }
    fn play_impl(&self, repeat: bool) -> PlayHandle {
        let playback_id = PLAYBACK_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        unsafe {
            _audio_play(self.audio_id, playback_id, repeat);
        }
        PlayHandle { playback_id }
    }
}

#[derive(Debug)]
pub struct PlayHandle {
    playback_id: usize,
}

impl Drop for PlayHandle {
    fn drop(&mut self) {
        unsafe {
            _audio_playback_drop(self.playback_id);
        }
    }
}

#[derive(Debug, Clone)]
struct AudioDestroyer {
    audio_id: usize,
}

impl Drop for AudioDestroyer {
    fn drop(&mut self) {
        unsafe {
            _audio_drop(self.audio_id);
        }
    }
}

struct VolumeSetting {
    volume: f32,
    next_request_sequence_number: usize,
}
static VOLUME_SETTING: OnceLock<Arc<Mutex<VolumeSetting>>> = OnceLock::new();
pub(crate) fn set_volume(zero_to_one: f32) {
    let request_sequence_number = {
        let volume_setting = VOLUME_SETTING.get_or_init(|| {
            Arc::new(Mutex::new(VolumeSetting {
                volume: 1.0,
                next_request_sequence_number: 0,
            }))
        });
        let mut volume_setting = volume_setting.lock().unwrap();
        volume_setting.volume = zero_to_one;

        let request_sequence_number = volume_setting.next_request_sequence_number;
        volume_setting.next_request_sequence_number += 1;
        request_sequence_number
    };
    unsafe {
        _audio_context_volume_set(zero_to_one, request_sequence_number);
    }
}

pub(crate) fn volume() -> f32 {
    let volume_setting = VOLUME_SETTING.get_or_init(|| {
        Arc::new(Mutex::new(VolumeSetting {
            volume: 1.0,
            next_request_sequence_number: 0,
        }))
    });
    let volume_setting = volume_setting.lock().unwrap();
    volume_setting.volume
}

unsafe extern "C" {
    fn _audio_init(audio_id: usize, buffer_ptr: *const u8, buffer_len: usize);
    fn _audio_drop(audio_id: usize);
    fn _audio_play(audio_id: usize, playback_id: usize, repeat: bool);
    fn _audio_play_and_forget(audio_id: usize);
    fn _audio_playback_drop(playback_id: usize);
    fn _audio_context_volume_set(volume: f32, request_sequence_number: usize);
}

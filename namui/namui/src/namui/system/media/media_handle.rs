use super::{audio_context::AudioContext, media_struct::Media};
use anyhow::Result;
use namui_type::ImageHandle;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

/// MediaHandle is user-exposed handle of media.
/// It has mutex-locked Media.
///
/// Q. What happens if you clone an AudioHandle and play it?
/// A. Let's say it's independent. If you don't want it to be independent, you have to wrap it in Arc.
///
/// Q. What happens if you play an already playing AudioHandle again?
/// A. Nothing happens. (If you play an already playing AudioHandle again, it is ignored.)
#[derive(Debug)]
pub struct MediaHandle {
    media: Arc<Mutex<Media>>,
}

impl MediaHandle {
    pub(crate) fn new(audio_context: &AudioContext, path: &impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            media: Arc::new(Mutex::new(Media::new(audio_context, path)?)),
        })
    }
    pub fn play(&self) {
        self.media.lock().unwrap().play()
    }
    pub fn stop(&self) {
        self.media.lock().unwrap().stop()
    }
    pub fn pause(&self) {
        self.media.lock().unwrap().pause()
    }
    pub fn is_playing(&self) -> bool {
        self.media.lock().unwrap().is_playing()
    }
    pub fn get_image(&self) -> Result<Option<ImageHandle>> {
        self.media.lock().unwrap().get_image()
    }
    pub fn clone_independent(&self) -> Result<Self> {
        Ok(Self {
            media: Arc::new(Mutex::new(self.media.lock().unwrap().clone_independent()?)),
        })
    }
}

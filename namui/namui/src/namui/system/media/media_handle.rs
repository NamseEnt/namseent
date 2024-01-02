use super::{audio_context::AudioContext, media_struct::Media};
use anyhow::Result;
use namui_type::*;
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
#[derive(Debug, Clone)]
pub struct MediaHandle {
    media: Arc<Mutex<Media>>,
}

impl MediaHandle {
    pub(crate) fn new(audio_context: Arc<AudioContext>, path: &impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            media: Arc::new(Mutex::new(Media::new(audio_context, path)?)),
        })
    }
    pub fn play(&self) -> Result<()> {
        self.media.lock().unwrap().play()
    }
    pub fn stop(&self) -> Result<()> {
        self.media.lock().unwrap().stop()
    }
    pub fn pause(&self) -> Result<()> {
        self.media.lock().unwrap().pause()
    }
    /// If seek_to < 0, it will be set to 0.
    pub fn seek_to(&self, seek_to: Duration) -> Result<()> {
        self.media.lock().unwrap().seek_to(seek_to)
    }
    pub fn playback_duration(&self) -> Duration {
        self.media.lock().unwrap().playback_duration()
    }
    pub fn is_playing(&self) -> bool {
        self.media.lock().unwrap().is_playing()
    }
    pub fn get_image(&self) -> Option<ImageHandle> {
        // NOTE: Maybe lock blocks user hook loop.
        self.media.lock().unwrap().get_image()
    }
    pub fn clone_independent(&self) -> Result<Self> {
        Ok(Self {
            media: Arc::new(Mutex::new(self.media.lock().unwrap().clone_independent()?)),
        })
    }
    /// # Errors
    ///
    /// If you call this function before previous call is finished, it will return Err on previous call.
    pub async fn wait_for_preload(&self) -> Result<()> {
        let wait = { self.media.lock().unwrap().wait_for_preload()? };
        wait.await?;
        Ok(())
    }
}

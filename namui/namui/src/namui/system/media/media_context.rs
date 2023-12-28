use super::audio_context::AudioContext;
use crate::MediaHandle;
use anyhow::Result;
use std::path::Path;

pub struct MediaContext {
    audio_context: AudioContext,
}

impl MediaContext {
    pub fn new() -> Result<MediaContext> {
        Ok(MediaContext {
            audio_context: AudioContext::new()?,
        })
    }

    pub(crate) fn new_media(&self, path: &impl AsRef<Path>) -> Result<MediaHandle> {
        MediaHandle::new(&self.audio_context, path)
    }
}

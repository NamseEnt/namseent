use super::audio::AudioContext;
use crate::MediaHandle;
use anyhow::Result;
use std::{path::Path, sync::Arc};

pub struct MediaContext {
    audio_context: Arc<AudioContext>,
}

impl MediaContext {
    pub fn new() -> Result<MediaContext> {
        Ok(MediaContext {
            audio_context: AudioContext::new()?.into(),
        })
    }

    pub(crate) fn new_media(&self, path: &impl AsRef<Path>) -> Result<MediaHandle> {
        MediaHandle::new(self.audio_context.clone(), path)
    }
}

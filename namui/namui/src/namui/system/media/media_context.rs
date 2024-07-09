use super::{audio::AudioContext, core::MediaSource};
use crate::MediaHandle;
use anyhow::Result;
use std::sync::Arc;

pub struct MediaContext {
    pub(crate) audio_context: Arc<AudioContext>,
}

impl MediaContext {
    pub fn new() -> Result<MediaContext> {
        Ok(MediaContext {
            audio_context: AudioContext::new()?.into(),
        })
    }

    pub(crate) fn new_media(&self, source: MediaSource) -> Result<MediaHandle> {
        MediaHandle::new(self.audio_context.clone(), source)
    }

    pub(crate) fn set_volume(&self, zero_to_one: f32) {
        self.audio_context.set_volume(zero_to_one);
    }

    pub(crate) fn volume(&self) -> f32 {
        self.audio_context.volume()
    }
}

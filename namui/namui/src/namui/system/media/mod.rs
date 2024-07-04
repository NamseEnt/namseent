//! Multi-media excluding image. Go away image!
//! Video and Audio.

mod audio;
mod core;
mod media_context;
mod media_handle;
mod video;
mod with_instant;

use super::InitResult;
use anyhow::*;
pub use audio::*;
use core::MediaSource;
use media_context::MediaContext;
pub use media_handle::MediaHandle;
use std::sync::OnceLock;

const AUDIO_CHANNEL_BOUND: usize = 128;
const VIDEO_CHANNEL_BOUND: usize = 10;

static MEDIA_SYSTEM: OnceLock<MediaContext> = OnceLock::new();

// TODO: Restore Media system
pub(super) async fn init() -> InitResult {
    MEDIA_SYSTEM
        .set(MediaContext::new()?)
        .map_err(|_| anyhow!("Media system already initialized"))?;

    Ok(())
}

pub fn new_media(source: impl Into<MediaSource>) -> Result<MediaHandle> {
    MEDIA_SYSTEM.get().unwrap().new_media(source.into())
}

/// Volume will be clamped to 0.0 ~ 1.0 if it is out of range.
pub fn set_volume(zero_to_one: f32) {
    MEDIA_SYSTEM.get().unwrap().set_volume(zero_to_one);
}

/// Volume value range is 0.0 ~ 1.0.
pub fn volume() -> f32 {
    MEDIA_SYSTEM.get().unwrap().volume()
}

pub async fn new_full_load_once_audio(source: impl Into<MediaSource>) -> Result<FullLoadOnceAudio> {
    Ok(FullLoadOnceAudio::new(
        MEDIA_SYSTEM.get().unwrap().audio_context.clone(),
        source.into(),
    )
    .await?)
}

pub async fn new_full_load_repeat_audio(
    source: impl Into<MediaSource>,
) -> Result<FullLoadRepeatAudio> {
    Ok(FullLoadRepeatAudio::new(
        MEDIA_SYSTEM.get().unwrap().audio_context.clone(),
        source.into(),
    )
    .await?)
}

pub fn play_audio_consume(audio_consume: impl AudioConsume + 'static) -> Result<()> {
    MEDIA_SYSTEM
        .get()
        .unwrap()
        .audio_context
        .load_audio(audio_consume)
}

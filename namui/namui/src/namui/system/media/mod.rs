//! Multi-media excluding image. Go away image!
//! Video and Audio.

mod audio;
mod core;
mod media_context;
mod media_handle;
mod video;
mod with_instant;

use self::media_context::MediaContext;
pub use self::media_handle::MediaHandle;
use super::InitResult;
use anyhow::*;
use std::{path::Path, sync::OnceLock};

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

pub fn new_media(path: &impl AsRef<Path>) -> Result<MediaHandle> {
    MEDIA_SYSTEM.get().unwrap().new_media(path)
}

/// Volume will be clamped to 0.0 ~ 1.0 if it is out of range.
pub fn set_volume(zero_to_one: f32) {
    MEDIA_SYSTEM.get().unwrap().set_volume(zero_to_one);
}

/// Volume value range is 0.0 ~ 1.0.
pub fn volume() -> f32 {
    MEDIA_SYSTEM.get().unwrap().volume()
}

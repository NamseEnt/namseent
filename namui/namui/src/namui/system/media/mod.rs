//! Multi-media excluding image. Go away image!
//! Video and Audio.

mod audio_buffer_core;
mod context;
mod image_only_video;
mod media_handle;
mod media_struct;
mod synced_audio;

pub use self::media_handle::MediaHandle;
use super::InitResult;
use anyhow::*;
use context::*;
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct AudioConfig {
    sample_rate: u32,
    sample_format: ffmpeg_next::format::Sample,
    channel_layout: ffmpeg_next::channel_layout::ChannelLayout,
    sample_byte_size: usize,
    channel_count: usize,
}

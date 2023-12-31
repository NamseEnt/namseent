//! Multi-media excluding image. Go away image!
//! Video and Audio.

mod audio_buffer;
mod audio_buffer_core;
mod audio_context;
mod audio_handle;
mod audio_resampling;
mod flush_button;
mod image_only_video;
mod media_context;
mod media_handle;
mod media_struct;
mod open_media;
mod ref_counting;
mod synced_audio;
mod video_framer;
mod video_scaling;

use self::media_context::MediaContext;
pub use self::media_handle::MediaHandle;
use super::InitResult;
use anyhow::*;
use std::{path::Path, sync::OnceLock};

const AUDIO_CHANNEL_BOUND: usize = 128;
const VIDEO_CHANNEL_BOUND: usize = 20;

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
    channel_count: usize,
}

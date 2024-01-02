use super::audio_context::AudioContext;
use super::media_core::MediaCore;
use super::video_framer::VideoFramer;
use anyhow::Result;
use namui_type::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub(crate) struct Media {
    media_core: MediaCore,
    video: Option<VideoFramer>,
    audio_context: Arc<AudioContext>,
    path: PathBuf,
}

impl Media {
    pub(crate) fn new(audio_context: Arc<AudioContext>, path: &impl AsRef<Path>) -> Result<Media> {
        let (media_core, video, audio) = MediaCore::new(path, audio_context.output_config)?;

        if let Some(audio) = audio {
            audio_context.load_audio(audio)?;
        }

        Ok(Media {
            media_core,
            video,
            audio_context,
            path: path.as_ref().to_path_buf(),
        })
    }
    pub(crate) fn get_image(&mut self) -> Option<ImageHandle> {
        let Some(video) = &mut self.video else {
            return None;
        };

        video.get_image()
    }
    pub(crate) fn clone_independent(&self) -> Result<Self> {
        Self::new(self.audio_context.clone(), &self.path)
    }
}

impl std::ops::Deref for Media {
    type Target = MediaCore;

    fn deref(&self) -> &Self::Target {
        &self.media_core
    }
}

impl std::ops::DerefMut for Media {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.media_core
    }
}

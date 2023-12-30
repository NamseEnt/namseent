use super::audio_context::AudioContext;
use super::audio_handle::AudioHandle;
use super::image_only_video::ImageOnlyVideo;
use super::open_media::{open_media, OpenMediaFilter};
use anyhow::Result;
use namui_type::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Media {
    id: usize,
    video: Option<ImageOnlyVideo>,
    audio: Option<AudioHandle>,
    path: PathBuf,
    start_instant: Option<Instant>,
    anchor_playback_duration: Option<Duration>,
}

impl Media {
    pub(crate) fn new(audio_context: &AudioContext, path: &impl AsRef<Path>) -> Result<Media> {
        let id = generate_media_id();

        let (video_material, audio) =
            open_media(path, OpenMediaFilter::YesVideoYesAudio { audio_context })?;

        Ok(Media {
            id,
            video: video_material.map(|video| ImageOnlyVideo::new(path, video)),
            audio,
            path: path.as_ref().to_path_buf(),
            start_instant: None,
            anchor_playback_duration: None,
        })
    }
    pub(crate) fn play(&mut self, start_at: Instant) {
        self.start_instant = Some(start_at);
        if let Some(audio) = &mut self.audio {
            audio.play(start_at, self.anchor_playback_duration.unwrap_or_default());
        }

        if let Some(video) = &mut self.video {
            video.start(start_at, self.anchor_playback_duration.unwrap_or_default());
        }
    }
    pub(crate) fn stop(&mut self) {
        if let Some(audio) = &mut self.audio {
            audio.stop();
        }

        if let Some(video) = &mut self.video {
            video.stop();
        }
    }
    pub(crate) fn pause(&mut self) {
        if !self.is_playing() {
            return;
        }
        self.anchor_playback_duration = Some(
            (crate::time::now() - self.start_instant.unwrap())
                + self.anchor_playback_duration.unwrap_or_default(),
        );
        self.start_instant = None;

        if let Some(audio) = &mut self.audio {
            audio.pause();
        }

        if let Some(video) = &mut self.video {
            video.pause();
        }
    }
    /// It is not guaranteed to work well if `playback_duration` is negative.
    pub(crate) fn seek_to(&mut self, playback_duration: Duration) -> Result<()> {
        if let Some(audio) = &mut self.audio {
            audio.seek_to(playback_duration);
        }

        if let Some(video) = &mut self.video {
            video.seek_to(playback_duration)?;
        }

        self.anchor_playback_duration = Some(playback_duration);
        Ok(())
    }
    pub(crate) fn playback_duration(&self) -> Duration {
        let Some(start_instant) = self.start_instant else {
            return self.anchor_playback_duration.unwrap_or_default();
        };
        (crate::time::now() - start_instant) + self.anchor_playback_duration.unwrap_or_default()
    }
    pub(crate) fn is_playing(&self) -> bool {
        Some(true) == self.audio.as_ref().map(|audio| audio.is_playing())
            || Some(true) == self.video.as_ref().map(|video| video.is_playing())
    }
    pub(crate) fn get_image(&mut self) -> Result<Option<ImageHandle>> {
        let Some(video) = &mut self.video else {
            return Ok(None);
        };
        video.get_image()
    }
    pub(crate) fn clone_independent(&self) -> Result<Self> {
        let video = {
            if self.video.is_some() {
                let (video_material, _) = open_media(&self.path, OpenMediaFilter::YesVideoNoAudio)?;
                video_material.map(|video| ImageOnlyVideo::new(&self.path, video))
            } else {
                None
            }
        };

        Ok(Self {
            id: self.id,
            video,
            audio: self.audio.clone(),
            path: self.path.clone(),
            anchor_playback_duration: None,
            start_instant: None,
        })
    }
}

fn generate_media_id() -> usize {
    static MEDIA_ID: AtomicUsize = AtomicUsize::new(0);
    MEDIA_ID.fetch_add(1, Ordering::Relaxed)
}

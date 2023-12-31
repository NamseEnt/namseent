use super::audio_context::AudioContext;
use super::open_media::MediaCore;
use super::video_framer::VideoFramer;
use anyhow::Result;
use namui_type::*;
use std::path::Path;

#[derive(Debug)]
pub struct Media {
    media_core: MediaCore,
    start_instant: Option<Instant>,
    anchor_playback_duration: Option<Duration>,
    video: Option<VideoFramer>,
}

impl Media {
    pub(crate) fn new(audio_context: &AudioContext, path: &impl AsRef<Path>) -> Result<Media> {
        let (media_core, video, audio) = MediaCore::new(path, audio_context.output_config)?;

        if let Some(audio) = audio {
            audio_context.load_audio(audio)?;
        }

        // let video = open_video(path)?.map(|video| ImageOnlyVideo::new(path, video));
        // let audio = open_audio(path, audio_context)?;

        Ok(Media {
            media_core,
            start_instant: None,
            anchor_playback_duration: None,
            video,
        })
    }
    pub(crate) fn play(&mut self) -> Result<()> {
        self.media_core.play()
    }
    pub(crate) fn stop(&mut self) -> Result<()> {
        self.media_core.stop()
    }
    pub(crate) fn pause(&mut self) -> Result<()> {
        self.media_core.pause()
    }
    /// It is not guaranteed to work well if `playback_duration` is negative.
    pub(crate) fn seek_to(&mut self, playback_duration: Duration) -> Result<()> {
        self.media_core.seek_to(playback_duration)
    }
    pub(crate) fn playback_duration(&self) -> Duration {
        let Some(start_instant) = self.start_instant else {
            return self.anchor_playback_duration.unwrap_or_default();
        };
        (crate::time::now() - start_instant) + self.anchor_playback_duration.unwrap_or_default()
    }
    pub(crate) fn is_playing(&self) -> bool {
        todo!()
    }
    pub(crate) fn get_image(&mut self) -> Result<Option<ImageHandle>> {
        let Some(video) = &mut self.video else {
            return Ok(None);
        };

        video.get_image()
    }
    pub(crate) fn clone_independent(&self) -> Result<Self> {
        todo!()
    }
}

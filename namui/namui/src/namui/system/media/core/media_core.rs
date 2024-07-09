use super::MediaSource;
use crate::system::media::{
    audio::AudioContext,
    core::{open_media, DecodingThreadCommand},
    video::VideoFramer,
    with_instant::{WithInstant, WithInstantExt, WithNow},
};
use crate::*;
use anyhow::Result;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct MediaCore {
    command_tx: std::sync::mpsc::SyncSender<WithInstant<DecodingThreadCommand>>,
    playback_state: PlaybackState,
    media_duration: Duration,
    playback_duration_offset: Duration,
    audio_context: Arc<AudioContext>,
    video_framer: Option<VideoFramer>,
    source: MediaSource,
}

#[derive(Debug)]
enum PlaybackState {
    Playing { start_instant: Instant },
    Paused,
}

impl MediaCore {
    pub(crate) fn new(audio_context: Arc<AudioContext>, source: MediaSource) -> Result<Self> {
        let (command_tx, command_rx) = std::sync::mpsc::sync_channel(32);

        let (video_framer, audio_buffer, media_duration) =
            open_media(&source, command_rx, audio_context.output_config)?;

        if let Some(audio_buffer) = audio_buffer {
            audio_context.load_audio(audio_buffer)?;
        }

        Ok(Self {
            command_tx,
            playback_state: PlaybackState::Paused,
            media_duration,
            playback_duration_offset: Duration::default(),
            audio_context,
            video_framer,
            source,
        })
    }
    pub fn play(&mut self) -> Result<()> {
        let now = crate::time::now();

        self.playback_state = PlaybackState::Playing { start_instant: now };

        Ok(self
            .command_tx
            .send(DecodingThreadCommand::Play.with_instant(now))?)
    }
    pub fn stop(&mut self) -> Result<()> {
        self.playback_duration_offset = Duration::default();
        self.playback_state = PlaybackState::Paused;

        Ok(self
            .command_tx
            .send(DecodingThreadCommand::Stop.with_now())?)
    }
    pub fn pause(&mut self) -> Result<()> {
        self.playback_duration_offset = self.playback_duration();
        self.playback_state = PlaybackState::Paused;

        Ok(self
            .command_tx
            .send(DecodingThreadCommand::Pause.with_now())?)
    }
    pub fn seek_to(&mut self, seek_to: Duration) -> Result<()> {
        self.playback_duration_offset = seek_to.max(Duration::default());

        Ok(self
            .command_tx
            .send(DecodingThreadCommand::SeekTo { duration: seek_to }.with_now())?)
    }
    pub fn playback_duration(&mut self) -> Duration {
        match self.playback_state {
            PlaybackState::Playing { start_instant } => {
                let now = crate::time::now();
                let elapsed = now - start_instant;
                self.media_duration
                    .min(elapsed + self.playback_duration_offset)
            }
            PlaybackState::Paused => self.playback_duration_offset,
        }
    }
    pub fn is_playing(&mut self) -> bool {
        match self.playback_state {
            PlaybackState::Playing { start_instant } => {
                let now = crate::time::now();
                let elapsed = now - start_instant;
                elapsed + self.playback_duration_offset < self.media_duration
            }
            PlaybackState::Paused => false,
        }
    }
    pub fn wait_for_preload(
        &self,
    ) -> Result<impl std::future::Future<Output = Result<(), tokio::sync::oneshot::error::RecvError>>>
    {
        let (finish_tx, finish_rx) = tokio::sync::oneshot::channel();
        self.command_tx
            .send(DecodingThreadCommand::WaitForPreload { finish_tx }.with_now())?;

        Ok(finish_rx)
    }
    pub(crate) fn get_image(&mut self) -> Option<Image> {
        self.video_framer
            .as_mut()
            .and_then(|video_framer| video_framer.get_image())
    }
    pub(crate) fn clone_independent(&self) -> Result<Self> {
        Self::new(self.audio_context.clone(), self.source.clone())
    }
}

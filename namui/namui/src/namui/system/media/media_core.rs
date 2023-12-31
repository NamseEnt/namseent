use super::{
    audio_buffer::AudioBuffer,
    audio_resampling::start_audio_resampling,
    media_control::MediaController,
    video_framer::VideoFramer,
    video_scaling::start_video_scaling,
    with_instant::{WithInstant, WithInstantExt, WithNow},
};
use crate::media::{AudioConfig, AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND};
use anyhow::Result;
use namui_type::*;
use std::ops::Deref;

#[derive(Debug)]
pub(crate) struct MediaCore {
    command_tx: std::sync::mpsc::SyncSender<WithInstant<DecodingThreadCommand>>,
    playback_state: PlaybackState,
    media_duration: Duration,
    playback_duration_offset: Duration,
}

#[derive(Debug)]
enum PlaybackState {
    Playing { start_instant: Instant },
    Paused,
    Stopped,
}

impl MediaCore {
    pub(crate) fn new(
        path: &impl AsRef<std::path::Path>,
        audio_output_config: AudioConfig,
    ) -> Result<(Self, Option<VideoFramer>, Option<AudioBuffer>)> {
        let input_ctx = ffmpeg_next::format::input(&path)?;
        let (command_tx, command_rx) = std::sync::mpsc::sync_channel(32);

        let (video_framer, audio_buffer, media_duration) =
            open_media(input_ctx, command_rx, audio_output_config)?;

        Ok((
            Self {
                command_tx,
                playback_state: PlaybackState::Stopped,
                media_duration,
                playback_duration_offset: Duration::default(),
            },
            video_framer,
            audio_buffer,
        ))
    }
    pub fn play(&mut self) -> Result<()> {
        let now = crate::time::now();

        self.playback_state = PlaybackState::Playing { start_instant: now };

        Ok(self
            .command_tx
            .send(DecodingThreadCommand::Play.with_instant(now))?)
    }
    pub fn stop(&mut self) -> Result<()> {
        self.playback_state = PlaybackState::Stopped;

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
        self.playback_duration_offset = seek_to;
        Ok(self
            .command_tx
            .send(DecodingThreadCommand::SeekTo { duration: seek_to }.with_now())?)
    }
    pub fn playback_duration(&mut self) -> Duration {
        match self.playback_state {
            PlaybackState::Playing { start_instant } => {
                let now = crate::time::now();
                let elapsed = now - start_instant;
                self.media_duration.min(elapsed)
            }
            PlaybackState::Paused => self.playback_duration_offset,
            PlaybackState::Stopped => Duration::default(),
        }
    }
    pub fn is_playing(&mut self) -> bool {
        match self.playback_state {
            PlaybackState::Playing { start_instant } => {
                let now = crate::time::now();
                let elapsed = now - start_instant;
                elapsed < self.media_duration
            }
            PlaybackState::Paused | PlaybackState::Stopped => false,
        }
    }
}

fn open_media(
    input_ctx: ffmpeg_next::format::context::Input,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    audio_output_config: AudioConfig,
) -> Result<(Option<VideoFramer>, Option<AudioBuffer>, Duration)> {
    let mut video_framer = None;
    let mut audio_buffer = None;
    let mut duration: Option<Duration> = None;

    let mut media_controller = MediaController::new();

    let mut update_duration = |stream: &ffmpeg_next::format::stream::Stream| {
        duration = {
            let stream_duration = Duration::from_micros(stream.duration());
            if let Some(duration) = duration {
                Some(duration.max(stream_duration))
            } else {
                Some(stream_duration)
            }
        };
    };

    let decoding_streams = input_ctx
        .streams()
        .map(|stream| -> Result<Option<DecodingStream>> {
            match stream.parameters().medium() {
                ffmpeg_next::media::Type::Video => {
                    if video_framer.is_some() {
                        return Ok(None);
                    }

                    update_duration(&stream);

                    let context_decoder =
                        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

                    let decoder = context_decoder.decoder().video()?;
                    let fps = decoder.frame_rate().expect("frame_rate").into();
                    let wh = Wh::new(decoder.width(), decoder.height());
                    let pixel_type = decoder.format();

                    let (tx, rx) = std::sync::mpsc::sync_channel(VIDEO_CHANNEL_BOUND);
                    video_framer = Some(start_video_scaling(
                        rx,
                        media_controller.new_receiver(),
                        wh,
                        pixel_type,
                        fps,
                    ));

                    Ok(Some(DecodingStream::Video { decoder, tx }))
                }
                ffmpeg_next::media::Type::Audio => {
                    if audio_buffer.is_some() {
                        return Ok(None);
                    }

                    update_duration(&stream);

                    let context_decoder =
                        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

                    let decoder = context_decoder.decoder().audio()?;

                    let audio_input_config = AudioConfig {
                        sample_rate: decoder.rate(),
                        sample_format: decoder.format(),
                        channel_layout: decoder.channel_layout(),
                        channel_count: decoder.channel_layout().channels() as usize,
                    };

                    let (tx, rx) = std::sync::mpsc::sync_channel(AUDIO_CHANNEL_BOUND);

                    audio_buffer = Some(start_audio_resampling(
                        rx,
                        audio_input_config,
                        audio_output_config,
                        media_controller.new_receiver(),
                    ));

                    Ok(Some(DecodingStream::Audio { decoder, tx }))
                }
                _ => Ok(None),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    spawn_decoding_thread(input_ctx, decoding_streams, command_rx, media_controller);

    Ok((video_framer, audio_buffer, duration.unwrap_or_default()))
}

#[derive(Debug)]
enum DecodingThreadCommand {
    Play,
    Stop,
    Pause,
    SeekTo { duration: Duration },
}

fn spawn_decoding_thread(
    mut input_ctx: ffmpeg_next::format::context::Input,
    mut decoding_streams: Vec<Option<DecodingStream>>,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    media_controller: MediaController,
) {
    std::thread::spawn(move || {
        (move || -> Result<()> {
            let mut eof = false;
            loop {
                loop {
                    match command_rx.try_recv() {
                        Ok(command) => match command.deref() {
                            DecodingThreadCommand::Play => {
                                media_controller.start();
                            }
                            DecodingThreadCommand::Stop => {
                                media_controller.flush();
                                media_controller.stop();
                                input_ctx.seek(0, ..)?;
                                eof = false;
                            }
                            DecodingThreadCommand::Pause => {
                                media_controller.stop();
                            }
                            DecodingThreadCommand::SeekTo { duration } => {
                                media_controller.flush();
                                input_ctx.seek(duration.as_micros() as i64, ..)?;
                                eof = false;
                            }
                        },
                        Err(err) => match err {
                            std::sync::mpsc::TryRecvError::Empty => {
                                break;
                            }
                            std::sync::mpsc::TryRecvError::Disconnected => {
                                println!("command disconnected");
                                return Ok(());
                            }
                        },
                    }
                }

                match input_ctx.packets().next() {
                    Some((stream, packet)) => {
                        let Some(decoding_stream) = &mut decoding_streams[stream.index()] else {
                            continue;
                        };

                        decoding_stream.send_packet(&packet)?;
                        decoding_stream.receive_and_process_decoded_frames()?;
                    }
                    None => {
                        if !eof {
                            eof = true;
                            println!("EOF");
                            for decoding_stream in decoding_streams
                                .iter_mut()
                                .filter_map(|decoding_stream| decoding_stream.as_mut())
                            {
                                decoding_stream.send_eof()?;
                                decoding_stream.receive_and_process_decoded_frames()?;
                            }
                        }
                    }
                }
            }
        })()
        .unwrap()
    });
}

enum DecodingStream {
    Video {
        decoder: ffmpeg_next::decoder::Video,
        tx: std::sync::mpsc::SyncSender<WithInstant<ffmpeg_next::frame::Video>>,
    },
    Audio {
        decoder: ffmpeg_next::decoder::Audio,
        tx: std::sync::mpsc::SyncSender<WithInstant<ffmpeg_next::frame::Audio>>,
    },
}

impl DecodingStream {
    fn receive_and_process_decoded_frames(&mut self) -> Result<()> {
        loop {
            let mut decoded = unsafe { ffmpeg_next::Frame::empty() };

            let Ok(_) = self.receive_frame(&mut decoded) else {
                break;
            };

            match self {
                DecodingStream::Video { decoder: _, tx } => {
                    // TODO: scale or change pixel format if decoder.format() is not supported on skia.
                    let video = ffmpeg_next::frame::Video::from(decoded);
                    tx.send(video.with_now()).map_err(|_| {
                        anyhow::anyhow!("failed to send video frame to image only video")
                    })?;
                }
                DecodingStream::Audio { decoder: _, tx } => {
                    let audio = ffmpeg_next::frame::Audio::from(decoded);
                    tx.send(audio.with_now()).map_err(|_| {
                        anyhow::anyhow!("failed to send audio frame to synced audio")
                    })?;
                }
            }
        }

        Ok(())
    }
    fn receive_frame(&mut self, frame: &mut ffmpeg_next::Frame) -> Result<()> {
        match self {
            DecodingStream::Video { decoder, tx: _ } => decoder.receive_frame(frame)?,
            DecodingStream::Audio { decoder, tx: _ } => decoder.receive_frame(frame)?,
        }
        Ok(())
    }

    fn send_packet(&mut self, packet: &ffmpeg_next::Packet) -> Result<()> {
        match self {
            DecodingStream::Video { decoder, tx: _ } => decoder.send_packet(packet)?,
            DecodingStream::Audio { decoder, tx: _ } => decoder.send_packet(packet)?,
        }
        Ok(())
    }

    fn send_eof(&mut self) -> Result<()> {
        match self {
            DecodingStream::Video { decoder, tx: _ } => decoder.send_eof()?,
            DecodingStream::Audio { decoder, tx: _ } => decoder.send_eof()?,
        }
        Ok(())
    }
}

use super::with_instant::{WithInstant, WithNow};
use anyhow::Result;
use namui_type::*;

pub(crate) enum DecodingStream {
    Video {
        decoder: ffmpeg_next::decoder::Video,
        tx: std::sync::mpsc::SyncSender<WithInstant<ffmpeg_next::frame::Video>>,
        parameters: ffmpeg_next::codec::Parameters,
        seek_to: Option<Duration>,
        sent_count: usize,
        time_base: f64,
    },
    Audio {
        decoder: ffmpeg_next::decoder::Audio,
        tx: std::sync::mpsc::SyncSender<WithInstant<ffmpeg_next::frame::Audio>>,
        parameters: ffmpeg_next::codec::Parameters,
        seek_to: Option<Duration>,
        sent_count: usize,
        time_base: f64,
    },
}

impl DecodingStream {
    pub(crate) fn receive_and_process_decoded_frames(&mut self) -> Result<()> {
        loop {
            let mut decoded = unsafe { ffmpeg_next::Frame::empty() };

            let Ok(_) = self.receive_frame(&mut decoded) else {
                break;
            };

            if self.should_skip_by_seek(&decoded) {
                continue;
            }

            self.send_decoded_frame(decoded)?;
        }

        Ok(())
    }

    pub(crate) fn receive_frame(&mut self, frame: &mut ffmpeg_next::Frame) -> Result<()> {
        match self {
            DecodingStream::Video { decoder, .. } => decoder.receive_frame(frame)?,
            DecodingStream::Audio { decoder, .. } => decoder.receive_frame(frame)?,
        }
        Ok(())
    }

    pub(crate) fn send_packet(&mut self, packet: &ffmpeg_next::Packet) -> Result<()> {
        match self {
            DecodingStream::Video { decoder, .. } => decoder.send_packet(packet)?,
            DecodingStream::Audio { decoder, .. } => decoder.send_packet(packet)?,
        }
        Ok(())
    }

    pub(crate) fn send_eof(&mut self) -> Result<()> {
        match self {
            DecodingStream::Video { decoder, .. } => decoder.send_eof()?,
            DecodingStream::Audio { decoder, .. } => decoder.send_eof()?,
        }
        Ok(())
    }

    pub(crate) fn reset_decoder(&mut self) -> Result<()> {
        match self {
            DecodingStream::Video {
                decoder,
                parameters,
                sent_count,
                ..
            } => {
                decoder.flush();

                let context_decoder =
                    ffmpeg_next::codec::context::Context::from_parameters(parameters.clone())?;
                *decoder = context_decoder.decoder().video()?;

                *sent_count = 0;
            }
            DecodingStream::Audio {
                decoder,
                parameters,
                sent_count,
                ..
            } => {
                decoder.flush();

                let context_decoder =
                    ffmpeg_next::codec::context::Context::from_parameters(parameters.clone())?;
                *decoder = context_decoder.decoder().audio()?;

                *sent_count = 0;
            }
        }
        Ok(())
    }

    pub(crate) fn seek_to(&mut self, duration: Duration) {
        match self {
            DecodingStream::Video { seek_to, .. } => *seek_to = Some(duration),
            DecodingStream::Audio { seek_to, .. } => *seek_to = Some(duration),
        }
    }

    pub(crate) fn sent_count(&self) -> usize {
        match self {
            DecodingStream::Video { sent_count, .. } => *sent_count,
            DecodingStream::Audio { sent_count, .. } => *sent_count,
        }
    }

    fn should_skip_by_seek(&self, decoded: &ffmpeg_next::Frame) -> bool {
        let (time_base, seek_to) = match self {
            DecodingStream::Video {
                time_base, seek_to, ..
            } => (*time_base, *seek_to),
            DecodingStream::Audio {
                time_base, seek_to, ..
            } => (*time_base, *seek_to),
        };

        let Some(seek_to) = seek_to else {
            return false;
        };

        let timestamp_f64 = decoded.timestamp().unwrap() as f64 * time_base;
        let timestamp = Duration::from_secs_f64(timestamp_f64);

        timestamp <= seek_to
    }

    fn send_decoded_frame(&mut self, decoded: ffmpeg_next::Frame) -> Result<()> {
        match self {
            DecodingStream::Video { tx, sent_count, .. } => {
                // TODO: scale or change pixel format if decoder.format() is not supported on skia.
                let video = ffmpeg_next::frame::Video::from(decoded);
                tx.send(video.with_now()).map_err(|_| {
                    anyhow::anyhow!("failed to send video frame to image only video")
                })?;

                *sent_count += 1;
            }
            DecodingStream::Audio { tx, sent_count, .. } => {
                let audio = ffmpeg_next::frame::Audio::from(decoded);
                tx.send(audio.with_now())
                    .map_err(|_| anyhow::anyhow!("failed to send audio frame to synced audio"))?;

                *sent_count += 1;
            }
        }

        Ok(())
    }
}

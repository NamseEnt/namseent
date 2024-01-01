use super::with_instant::{WithInstant, WithNow};
use anyhow::Result;

pub(crate) enum DecodingStream {
    Video {
        decoder: ffmpeg_next::decoder::Video,
        tx: std::sync::mpsc::SyncSender<WithInstant<ffmpeg_next::frame::Video>>,
        parameters: ffmpeg_next::codec::Parameters,
    },
    Audio {
        decoder: ffmpeg_next::decoder::Audio,
        tx: std::sync::mpsc::SyncSender<WithInstant<ffmpeg_next::frame::Audio>>,
        parameters: ffmpeg_next::codec::Parameters,
    },
}

impl DecodingStream {
    pub(crate) fn receive_and_process_decoded_frames(&mut self) -> Result<()> {
        loop {
            let mut decoded = unsafe { ffmpeg_next::Frame::empty() };

            let Ok(_) = self.receive_frame(&mut decoded) else {
                break;
            };

            match self {
                DecodingStream::Video { tx, .. } => {
                    // TODO: scale or change pixel format if decoder.format() is not supported on skia.
                    let video = ffmpeg_next::frame::Video::from(decoded);
                    tx.send(video.with_now()).map_err(|_| {
                        anyhow::anyhow!("failed to send video frame to image only video")
                    })?;
                }
                DecodingStream::Audio { tx, .. } => {
                    let audio = ffmpeg_next::frame::Audio::from(decoded);
                    tx.send(audio.with_now()).map_err(|_| {
                        anyhow::anyhow!("failed to send audio frame to synced audio")
                    })?;
                }
            }
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
                ..
            } => {
                decoder.flush();

                let context_decoder =
                    ffmpeg_next::codec::context::Context::from_parameters(parameters.clone())?;
                *decoder = context_decoder.decoder().video()?;
            }
            DecodingStream::Audio {
                decoder,
                parameters,
                ..
            } => {
                decoder.flush();

                let context_decoder =
                    ffmpeg_next::codec::context::Context::from_parameters(parameters.clone())?;
                *decoder = context_decoder.decoder().audio()?;
            }
        }
        Ok(())
    }
}

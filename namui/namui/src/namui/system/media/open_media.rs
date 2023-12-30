use super::{
    audio_context::AudioContext, audio_handle::AudioHandle, image_only_video::VideoMaterials,
};
use crate::media::{
    audio_buffer_core::AudioBufferCore, AudioConfig, AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND,
};
use anyhow::Result;
use namui_type::*;

pub(crate) fn open_video(path: &impl AsRef<std::path::Path>) -> Result<Option<VideoMaterials>> {
    let path = path.as_ref();
    let ictx = ffmpeg_next::format::input(&path)?;

    for stream in ictx.streams() {
        if stream.parameters().medium() != ffmpeg_next::media::Type::Video {
            continue;
        }

        let context_decoder =
            ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

        let decoder = context_decoder.decoder().video()?;
        let (tx, rx) = crossbeam_channel::bounded(VIDEO_CHANNEL_BOUND);

        let fps = decoder.frame_rate().expect("frame_rate").into();
        let wh = Wh::new(decoder.width(), decoder.height());
        let pixel_type = decoder.format();

        let stream_index = stream.index();

        spawn_decoding_thread(ictx, DecodingStream::Video { decoder, tx }, stream_index);

        return Ok(Some(VideoMaterials {
            frame_rx: rx,
            fps,
            wh,
            pixel_type,
        }));
    }

    Ok(None)
}

pub(crate) fn open_audio(
    path: &impl AsRef<std::path::Path>,
    audio_context: &AudioContext,
) -> Result<Option<AudioHandle>> {
    let path = path.as_ref();
    let ictx = ffmpeg_next::format::input(&path)?;

    for stream in ictx.streams() {
        if stream.parameters().medium() != ffmpeg_next::media::Type::Audio {
            continue;
        }

        let context_decoder =
            ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

        let decoder = context_decoder.decoder().audio()?;

        let (tx, rx) = crossbeam_channel::bounded(AUDIO_CHANNEL_BOUND);

        let audio_buffer_core = AudioBufferCore::new(
            rx,
            AudioConfig {
                sample_rate: decoder.rate(),
                sample_format: decoder.format(),
                channel_layout: decoder.channel_layout(),
                sample_byte_size: decoder.format().bytes(),
                channel_count: decoder.channel_layout().channels() as usize,
            },
            audio_context.output_config,
        )?;

        let stream_index = stream.index();

        spawn_decoding_thread(ictx, DecodingStream::Audio { decoder, tx }, stream_index);

        return Ok(Some(audio_context.load_audio(audio_buffer_core)?));
    }

    Ok(None)
}

fn spawn_decoding_thread(
    mut ictx: ffmpeg_next::format::context::Input,
    mut decoding_stream: DecodingStream,
    stream_index: usize,
) {
    std::thread::spawn(move || {
        match (move || -> Result<()> {
            for (stream, packet) in ictx.packets() {
                if stream.index() != stream_index {
                    continue;
                }
                decoding_stream.send_packet(&packet)?;
                decoding_stream.receive_and_process_decoded_frames()?;
            }

            decoding_stream.send_eof()?;
            decoding_stream.receive_and_process_decoded_frames()?;

            Ok(())
        })() {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Fail on media decoding: {}.
It would not be error, because resource can be dropped during decoding.",
                    e
                );
            }
        }
    });
}

enum DecodingStream {
    Video {
        decoder: ffmpeg_next::decoder::Video,
        tx: crossbeam_channel::Sender<ffmpeg_next::frame::Video>,
    },
    Audio {
        decoder: ffmpeg_next::decoder::Audio,
        tx: crossbeam_channel::Sender<ffmpeg_next::frame::Audio>,
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
                    tx.send(video).map_err(|_| {
                        anyhow::anyhow!("failed to send video frame to image only video")
                    })?;
                }
                DecodingStream::Audio { decoder: _, tx } => {
                    let audio = ffmpeg_next::frame::Audio::from(decoded);
                    tx.send(audio).map_err(|_| {
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

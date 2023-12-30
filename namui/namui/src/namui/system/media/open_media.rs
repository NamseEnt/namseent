use super::{
    audio_context::AudioContext, audio_handle::AudioHandle, image_only_video::VideoMaterials,
};
use crate::media::{
    audio_buffer_core::AudioBufferCore, AudioConfig, AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND,
};
use anyhow::Result;
use namui_type::*;

pub(crate) enum OpenMediaFilter<'a> {
    YesVideoNoAudio,
    YesVideoYesAudio { audio_context: &'a AudioContext },
}

pub(crate) fn open_media(
    path: &impl AsRef<std::path::Path>,
    filter: OpenMediaFilter,
) -> Result<(Option<VideoMaterials>, Option<AudioHandle>)> {
    let path = path.as_ref();
    let mut ictx = ffmpeg_next::format::input(&path)?;

    let mut audio = None;
    let mut video = None;

    let mut stream_decoding_stream: Vec<Option<DecodingStream>> = ictx
        .streams()
        .map(|stream| -> Result<Option<_>> {
            let parameters = stream.parameters();
            let medium = parameters.medium();

            enum StreamMediaType {
                Video,
                Audio,
            }
            let stream_media_type = match medium {
                ffmpeg_next::media::Type::Video => StreamMediaType::Video,
                ffmpeg_next::media::Type::Audio => StreamMediaType::Audio,
                _ => {
                    return Ok(None);
                }
            };
            let context_decoder =
                ffmpeg_next::codec::context::Context::from_parameters(parameters)?;

            let decoding_stream: DecodingStream = match stream_media_type {
                StreamMediaType::Video => {
                    if video.is_some() {
                        eprintln!("Warning: only one video stream is supported.");
                        return Ok(None);
                    };

                    let decoder = context_decoder.decoder().video()?;
                    let (tx, rx) = crossbeam_channel::bounded(VIDEO_CHANNEL_BOUND);

                    let fps = decoder.frame_rate().expect("frame_rate").into();
                    let wh = Wh::new(decoder.width(), decoder.height());
                    let pixel_type = decoder.format();

                    video = Some(VideoMaterials {
                        frame_rx: rx,
                        fps,
                        wh,
                        pixel_type,
                    });

                    DecodingStream::Video { decoder, tx }
                }
                StreamMediaType::Audio => {
                    if audio.is_some() {
                        eprintln!("Warning: only one audio stream is supported.");
                        return Ok(None);
                    };

                    let OpenMediaFilter::YesVideoYesAudio { audio_context } = filter else {
                        return Ok(None);
                    };

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

                    audio = Some(audio_context.load_audio(audio_buffer_core)?);

                    DecodingStream::Audio { decoder, tx }
                }
            };

            Ok(Some(decoding_stream))
        })
        .collect::<Result<_>>()?;

    std::thread::spawn(move || {
        match (move || -> Result<()> {
            for (stream, packet) in ictx.packets() {
                let Some(decoding_stream) = &mut stream_decoding_stream[stream.index()] else {
                    continue;
                };
                decoding_stream.send_packet(&packet)?;
                decoding_stream.receive_and_process_decoded_frames()?;
            }

            for mut decoding_stream in stream_decoding_stream {
                let Some(decoding_stream) = &mut decoding_stream else {
                    continue;
                };
                decoding_stream.send_eof()?;
                decoding_stream.receive_and_process_decoded_frames()?;
            }

            Ok(())
        })() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Fail on media decoding: {}", e);
            }
        }
    });

    if matches!(filter, OpenMediaFilter::YesVideoNoAudio) {
        assert!(audio.is_none());
    }

    Ok((video, audio))
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

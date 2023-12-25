use super::{context::MediaContext, image_only_video::ImageOnlyVideo, synced_audio::SyncedAudio};
use crate::media::{AudioConfig, AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND};
use anyhow::Result;
use namui_type::{ImageHandle, Wh};
use std::path::Path;

#[derive(Debug)]
pub struct Media {
    pub(crate) video: Option<ImageOnlyVideo>,
    pub(crate) audio: Option<SyncedAudio>,
}

impl Media {
    pub(crate) fn new(media_context: &MediaContext, path: &impl AsRef<Path>) -> Result<Media> {
        let mut ictx = ffmpeg_next::format::input(path)?;

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

                        video = Some(ImageOnlyVideo::new(rx, fps, wh, pixel_type)?);

                        DecodingStream::Video { decoder, tx }
                    }
                    StreamMediaType::Audio => {
                        if audio.is_some() {
                            eprintln!("Warning: only one audio stream is supported.");
                            return Ok(None);
                        };

                        let decoder = context_decoder.decoder().audio()?;

                        let (tx, rx) = crossbeam_channel::bounded(AUDIO_CHANNEL_BOUND);

                        let output_sample_format = {
                            let packed = ffmpeg_next::format::sample::Type::Packed;
                            match media_context.audio_output_sample_format {
                                cpal::SampleFormat::I8 => unimplemented!(),
                                cpal::SampleFormat::I16 => ffmpeg_next::format::Sample::I16(packed),
                                cpal::SampleFormat::I32 => ffmpeg_next::format::Sample::I32(packed),
                                cpal::SampleFormat::I64 => ffmpeg_next::format::Sample::I64(packed),
                                cpal::SampleFormat::U8 => ffmpeg_next::format::Sample::U8(packed),
                                cpal::SampleFormat::U16 => unimplemented!(),
                                cpal::SampleFormat::U32 => unimplemented!(),
                                cpal::SampleFormat::U64 => unimplemented!(),
                                cpal::SampleFormat::F32 => ffmpeg_next::format::Sample::F32(packed),
                                cpal::SampleFormat::F64 => ffmpeg_next::format::Sample::F64(packed),
                                _ => todo!(),
                            }
                        };

                        audio = Some(SyncedAudio::new(
                            rx,
                            AudioConfig {
                                sample_rate: decoder.rate(),
                                sample_format: decoder.format(),
                                channel_layout: decoder.channel_layout(),
                                sample_byte_size: decoder.format().bytes(),
                                channel_count: decoder.channel_layout().channels() as usize,
                            },
                            AudioConfig {
                                sample_rate: media_context.audio_output_sample_rate,
                                sample_format: output_sample_format,
                                channel_layout: if media_context.audio_output_channel_count == 1 {
                                    ffmpeg_next::ChannelLayout::MONO
                                } else {
                                    ffmpeg_next::ChannelLayout::STEREO
                                },
                                sample_byte_size: media_context
                                    .audio_output_sample_format
                                    .sample_size(),
                                channel_count: media_context.audio_output_channel_count,
                            },
                        )?);

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

        Ok(Media { video, audio })
    }

    pub(crate) fn play(&mut self) -> Option<SyncedAudio> {
        let mut audio = self.audio.clone();
        if let Some(audio) = &mut audio {
            audio.start();
        }

        if let Some(video) = &mut self.video {
            video.start();
        }

        audio
    }
    pub fn get_image(&mut self) -> Result<Option<ImageHandle>> {
        let Some(video) = &mut self.video else {
            return Ok(None);
        };
        video.get_image()
    }
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

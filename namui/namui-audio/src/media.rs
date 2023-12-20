use anyhow::Result;
use std::path::Path;

pub struct Media {
    video_frame_rx: std::sync::mpsc::Receiver<ffmpeg_next::frame::Video>,
    audio_frame_rx: std::sync::mpsc::Receiver<ffmpeg_next::frame::Audio>,
}

impl Media {
    pub fn new(path: &impl AsRef<Path>) -> Result<Media> {
        let mut ictx = ffmpeg_next::format::input(path)?;
        let (video_frame_tx, video_frame_rx) =
            std::sync::mpsc::sync_channel::<ffmpeg_next::frame::Video>(60 * 1);
        let mut video_frame_tx = Some(video_frame_tx);

        let (audio_frame_tx, audio_frame_rx) =
            std::sync::mpsc::sync_channel::<ffmpeg_next::frame::Audio>(2048);
        let mut audio_frame_tx = Some(audio_frame_tx);

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
                        let Some(tx) = video_frame_tx.take() else {
                            eprintln!("Warning: only one video stream is supported.");
                            return Ok(None);
                        };

                        DecodingStream::Video {
                            decoder: context_decoder.decoder().video()?,
                            tx,
                        }
                    }
                    StreamMediaType::Audio => {
                        let Some(tx) = audio_frame_tx.take() else {
                            eprintln!("Warning: only one audio stream is supported.");
                            return Ok(None);
                        };

                        DecodingStream::Audio {
                            decoder: context_decoder.decoder().audio()?,
                            tx,
                        }
                    }
                };

                Ok(Some(decoding_stream))
            })
            .collect::<Result<_>>()?;

        let on_rayon = move || -> Result<()> {
            for (steram, packet) in ictx.packets() {
                let Some(decoding_stream) = &mut stream_decoding_stream[steram.index()] else {
                    continue;
                };
                decoding_stream.send_packet(&packet)?;
                decoding_stream.receive_and_process_decoded_frames()?;

                rayon::yield_local();
            }

            stream_decoding_stream
                .into_iter()
                .filter_map(|decoder| decoder)
                .map(|mut decoder| {
                    decoder
                        .send_eof()
                        .and_then(|_| decoder.receive_and_process_decoded_frames())
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(())
        };

        rayon::spawn(move || match on_rayon() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Fail on media decoding: {}", e);
            }
        });

        Ok(Media {
            video_frame_rx,
            audio_frame_rx,
        })
    }
}

enum DecodingStream {
    Video {
        decoder: ffmpeg_next::decoder::Video,
        tx: std::sync::mpsc::SyncSender<ffmpeg_next::frame::Video>,
    },
    Audio {
        decoder: ffmpeg_next::decoder::Audio,
        tx: std::sync::mpsc::SyncSender<ffmpeg_next::frame::Audio>,
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
                    tx.send(video).unwrap();
                }
                DecodingStream::Audio { decoder: _, tx } => {
                    let audio = ffmpeg_next::frame::Audio::from(decoded);
                    tx.send(audio).unwrap();
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

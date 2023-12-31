use super::{
    audio_buffer::AudioBuffer, audio_resampling::start_audio_resampling, flush_button::FlushButton,
    video_framer::VideoFramer,
};
use crate::media::{AudioConfig, AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND};
use anyhow::Result;
use namui_type::*;

#[derive(Debug)]
pub(crate) struct MediaCore {
    command_tx: rtrb::Producer<DecodingThreadCommand>,
}

impl MediaCore {
    pub(crate) fn new(
        path: &impl AsRef<std::path::Path>,
        audio_output_config: AudioConfig,
    ) -> Result<(Self, Option<VideoFramer>, Option<AudioBuffer>)> {
        let input_ctx = ffmpeg_next::format::input(&path)?;
        let (command_tx, command_rx) = rtrb::RingBuffer::new(32);

        let (video_framer, audio_buffer) = open_media(input_ctx, command_rx, audio_output_config)?;

        Ok((Self { command_tx }, video_framer, audio_buffer))
    }
    pub fn play(&mut self) -> Result<()> {
        Ok(self.command_tx.push(DecodingThreadCommand::Play)?)
    }
    pub fn stop(&mut self) -> Result<()> {
        Ok(self.command_tx.push(DecodingThreadCommand::Stop)?)
    }
    pub fn pause(&mut self) -> Result<()> {
        Ok(self.command_tx.push(DecodingThreadCommand::Pause)?)
    }
    pub fn seek_to(&mut self, seek_to: Duration) -> Result<()> {
        Ok(self
            .command_tx
            .push(DecodingThreadCommand::SeekTo { duration: seek_to })?)
    }

    /*
    copilot아 이 주석 블럭에서는 동작하지 마.

    재생, 멈춤, 일시정지, seek을 제공해야한다.

    # 오디오

    오디오는 재생스레드가 따로 있다. 재생을 하려면 거기에 버퍼를 넘겨야한다.
    멈춤을 한다면 버퍼를 아예 걷어내면 된다. 아니면 멈췄다고 스레드에 알려주거나.
    일시정지 하려면? 이 역시 아예 걷어내거나, 멈췄다고 스레드에 알려주면 된다.

    # 비디오

    이번에는 비디오를 비디오마다 별개의 스레드에서 skia에 로드하고,
    현재 시각에 맞는 영상을 mutex에 넣어놓는 작업을 할 것이다.

    아예 재생, 멈춤, 일시정지, 검색을 이 core에서 제공해주면 어떨까?

    그리고 바깥에다가는 audio context에 넣어야 할 물건, Mutex<image>를 외부에 건네주면 어떨까?
    */
}

fn open_media(
    input_ctx: ffmpeg_next::format::context::Input,
    command_rx: rtrb::Consumer<DecodingThreadCommand>,
    audio_output_config: AudioConfig,
) -> Result<(Option<VideoFramer>, Option<AudioBuffer>)> {
    let mut video_framer = None;
    let mut audio_buffer = None;

    let mut flush_button = FlushButton::new();

    let decoding_streams = input_ctx
        .streams()
        .map(|stream| -> Result<Option<DecodingStream>> {
            match stream.parameters().medium() {
                ffmpeg_next::media::Type::Video => {
                    if video_framer.is_some() {
                        return Ok(None);
                    }

                    let context_decoder =
                        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

                    let decoder = context_decoder.decoder().video()?;
                    let (tx, rx) = rtrb::RingBuffer::new(VIDEO_CHANNEL_BOUND);
                    video_framer = Some(VideoFramer::new(rx));

                    Ok(Some(DecodingStream::Video { decoder, tx }))
                }
                ffmpeg_next::media::Type::Audio => {
                    if audio_buffer.is_some() {
                        return Ok(None);
                    }

                    let context_decoder =
                        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

                    let decoder = context_decoder.decoder().audio()?;

                    let audio_input_config = AudioConfig {
                        sample_rate: decoder.rate(),
                        sample_format: decoder.format(),
                        channel_layout: decoder.channel_layout(),
                        channel_count: decoder.channel_layout().channels() as usize,
                    };

                    let (tx, rx) = rtrb::RingBuffer::new(AUDIO_CHANNEL_BOUND);

                    audio_buffer = Some(start_audio_resampling(
                        rx,
                        audio_input_config,
                        audio_output_config,
                        flush_button.new_receiver(),
                    ));

                    Ok(Some(DecodingStream::Audio { decoder, tx }))
                }
                _ => Ok(None),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    spawn_decoding_thread(input_ctx, decoding_streams, command_rx, flush_button);

    Ok((video_framer, audio_buffer))
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
    mut command_rx: rtrb::Consumer<DecodingThreadCommand>,
    flush_button: FlushButton,
) {
    std::thread::spawn(move || {
        match (move || -> Result<()> {
            loop {
                while !command_rx.is_abandoned() {
                    let Ok(command) = command_rx.pop() else {
                        break;
                    };

                    match command {
                        DecodingThreadCommand::Play => input_ctx.play()?,
                        DecodingThreadCommand::Stop => {
                            flush_button.press();
                            input_ctx.seek(0, ..)?;
                            input_ctx.pause()?;
                        }
                        DecodingThreadCommand::Pause => {
                            flush_button.press();
                            input_ctx.pause()?
                        }
                        DecodingThreadCommand::SeekTo { duration } => {
                            flush_button.press();
                            input_ctx.seek(duration.as_micros() as i64, ..)?
                        }
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
        tx: rtrb::Producer<ffmpeg_next::frame::Video>,
    },
    Audio {
        decoder: ffmpeg_next::decoder::Audio,
        tx: rtrb::Producer<ffmpeg_next::frame::Audio>,
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
                    tx.push(video).map_err(|_| {
                        anyhow::anyhow!("failed to send video frame to image only video")
                    })?;
                }
                DecodingStream::Audio { decoder: _, tx } => {
                    let audio = ffmpeg_next::frame::Audio::from(decoded);
                    tx.push(audio).map_err(|_| {
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

// pub(crate) fn open_video(path: &impl AsRef<std::path::Path>) -> Result<Option<VideoMaterials>> {
//     let path = path.as_ref();
//     let input_ctx = ffmpeg_next::format::input(&path)?;

//     for stream in input_ctx.streams() {
//         if stream.parameters().medium() != ffmpeg_next::media::Type::Video {
//             continue;
//         }

//         let context_decoder =
//             ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

//         let decoder = context_decoder.decoder().video()?;
//         let (tx, rx) = crossbeam_channel::bounded(VIDEO_CHANNEL_BOUND);

//         let fps = decoder.frame_rate().expect("frame_rate").into();
//         let wh = Wh::new(decoder.width(), decoder.height());
//         let pixel_type = decoder.format();

//         let stream_index = stream.index();

//         spawn_decoding_thread(
//             input_ctx,
//             DecodingStream::Video { decoder, tx },
//             stream_index,
//         );

//         return Ok(Some(VideoMaterials {
//             frame_rx: rx,
//             fps,
//             wh,
//             pixel_type,
//         }));
//     }

//     Ok(None)
// }

// pub(crate) fn open_audio(
//     path: &impl AsRef<std::path::Path>,
//     audio_context: &AudioContext,
// ) -> Result<Option<AudioHandle>> {
//     let path = path.as_ref();
//     let input_ctx = ffmpeg_next::format::input(&path)?;

//     for stream in input_ctx.streams() {
//         if stream.parameters().medium() != ffmpeg_next::media::Type::Audio {
//             continue;
//         }

//         let context_decoder =
//             ffmpeg_next::codec::context::Context::from_parameters(stream.parameters())?;

//         let decoder = context_decoder.decoder().audio()?;

//         let (tx, rx) = crossbeam_channel::bounded(AUDIO_CHANNEL_BOUND);

//         let audio_buffer_core = AudioBufferCore::new(
//             rx,
//             AudioConfig {
//                 sample_rate: decoder.rate(),
//                 sample_format: decoder.format(),
//                 channel_layout: decoder.channel_layout(),
//                 sample_byte_size: decoder.format().bytes(),
//                 channel_count: decoder.channel_layout().channels() as usize,
//             },
//             audio_context.output_config,
//         )?;

//         let stream_index = stream.index();

//         spawn_decoding_thread(
//             input_ctx,
//             DecodingStream::Audio { decoder, tx },
//             stream_index,
//         );

//         return Ok(Some(audio_context.load_audio(audio_buffer_core)?));
//     }

//     Ok(None)
// }

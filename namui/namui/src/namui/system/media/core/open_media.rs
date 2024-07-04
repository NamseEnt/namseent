use super::MediaSource;
use crate::media::{
    audio::{start_audio_resampling, AudioBuffer, AudioConfig},
    core::{spawn_decoding_thread, DecodingStream, DecodingThreadCommand, MediaController},
    video::{start_video_scaling, VideoFramer},
    with_instant::WithInstant,
    AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND,
};
use anyhow::Result;
use namui_type::*;

pub(crate) fn open_media(
    source: &MediaSource,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    audio_output_config: AudioConfig,
) -> Result<(Option<VideoFramer>, Option<AudioBuffer>, Duration)> {
    let input_ctx = source.create_input_context()?;

    let mut video_framer = None;
    let mut audio_buffer = None;
    let duration = Duration::from_secs_f64(
        input_ctx.duration() as f64 / f64::from(ffmpeg_next::ffi::AV_TIME_BASE),
    );

    let mut media_controller = MediaController::new();

    let decoding_streams = input_ctx
        .streams()
        .map(|stream| -> Result<Option<DecodingStream>> {
            match stream.parameters().medium() {
                ffmpeg_next::media::Type::Video => {
                    if video_framer.is_some() {
                        return Ok(None);
                    }

                    let parameters = stream.parameters();
                    let context_decoder =
                        ffmpeg_next::codec::context::Context::from_parameters(parameters.clone())?;

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

                    Ok(Some(DecodingStream::Video {
                        decoder,
                        tx,
                        parameters,
                        seek_to: None,
                        sent_count: 0,
                        time_base: f64::from(stream.time_base()), // NOTE: decoder.time_base() shows 0/1. I have no idea.
                    }))
                }
                ffmpeg_next::media::Type::Audio => {
                    if audio_buffer.is_some() {
                        return Ok(None);
                    }

                    let parameters = stream.parameters();
                    let context_decoder =
                        ffmpeg_next::codec::context::Context::from_parameters(parameters.clone())?;

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

                    Ok(Some(DecodingStream::Audio {
                        decoder,
                        tx,
                        parameters,
                        seek_to: None,
                        sent_count: 0,
                        time_base: f64::from(stream.time_base()),
                    }))
                }
                _ => Ok(None),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    spawn_decoding_thread(input_ctx, decoding_streams, command_rx, media_controller);

    Ok((video_framer, audio_buffer, duration))
}

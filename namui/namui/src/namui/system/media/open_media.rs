use super::{
    audio_buffer::AudioBuffer,
    audio_resampling::start_audio_resampling,
    media_control::MediaController,
    media_decoding_stream::DecodingStream,
    media_decoding_thread::{spawn_decoding_thread, DecodingThreadCommand},
    video_framer::VideoFramer,
    video_scaling::start_video_scaling,
    with_instant::WithInstant,
};
use crate::media::{AudioConfig, AUDIO_CHANNEL_BOUND, VIDEO_CHANNEL_BOUND};
use anyhow::Result;
use namui_type::*;

pub(crate) fn open_media(
    input_ctx: ffmpeg_next::format::context::Input,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    audio_output_config: AudioConfig,
) -> Result<(Option<VideoFramer>, Option<AudioBuffer>, Duration)> {
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
                    }))
                }
                _ => Ok(None),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    spawn_decoding_thread(input_ctx, decoding_streams, command_rx, media_controller);

    Ok((video_framer, audio_buffer, duration))
}

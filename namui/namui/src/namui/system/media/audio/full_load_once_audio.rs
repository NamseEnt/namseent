use crate::media::audio::{self, AudioConfig, AudioConsume, AudioContext};
use anyhow::{bail, Result};
use namui_type::*;
use std::{collections::VecDeque, path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub struct FullLoadOnceAudio {
    audio_context: Arc<AudioContext>,
    audio_config: AudioConfig,
    buffer: VecDeque<f32>,
}

impl FullLoadOnceAudio {
    pub(crate) async fn new(
        audio_context: Arc<AudioContext>,
        path: &impl AsRef<Path>,
    ) -> Result<Self> {
        let output_config = audio_context.output_config;
        let path = path.as_ref().to_path_buf();
        let buffer = tokio::task::spawn_blocking(move || -> Result<VecDeque<f32>> {
            let mut input_ctx = ffmpeg_next::format::input(&path)?;

            let audio_stream = input_ctx
                .streams()
                .find(|stream| stream.parameters().medium() == ffmpeg_next::media::Type::Audio)
                .ok_or_else(|| anyhow::anyhow!("No audio stream found."))?;

            let context_decoder =
                ffmpeg_next::codec::context::Context::from_parameters(audio_stream.parameters())?;

            let mut decoder = context_decoder.decoder().audio()?;

            let input_config = AudioConfig {
                sample_rate: decoder.rate(),
                sample_format: decoder.format(),
                channel_layout: decoder.channel_layout(),
                channel_count: decoder.channel_layout().channels() as usize,
            };

            let mut resampler = audio::get_resampler(input_config, output_config)?;

            let mut output: VecDeque<f32> = VecDeque::new();

            let mut receive_frame =
                |decoder: &mut ffmpeg_next::codec::decoder::Audio, eof: bool| -> Result<()> {
                    let mut decoded = ffmpeg_next::frame::Audio::empty();
                    if let Err(err) = decoder.receive_frame(&mut decoded) {
                        if eof && std::ffi::c_int::from(err) == ffmpeg_next::ffi::AVERROR_EOF {
                            return Ok(());
                        }
                        bail!("[namui-media] error while decoding audio: {:?}", err);
                    }

                    let mut resampled = ffmpeg_next::frame::Audio::empty();
                    if let Some(delay) = resampler.run(&decoded, &mut resampled)? {
                        eprintln!("[namui-media] unexpected delay: {:?}", delay);
                    }

                    assert!(resampled.is_packed());
                    const PACKED_DATA_INDEX: usize = 0;

                    let slice_with_extra = resampled.data(PACKED_DATA_INDEX);
                    let slice = &slice_with_extra
                        [..slice_with_extra.len() - slice_with_extra.len() % resampled.samples()];

                    let f32_slice = unsafe {
                        std::slice::from_raw_parts(
                            slice.as_ptr() as *const f32,
                            slice.len() / std::mem::size_of::<f32>(),
                        )
                    };

                    output.extend(f32_slice);

                    Ok(())
                };

            let stream_index = audio_stream.index();

            for (_, packet) in input_ctx
                .packets()
                .filter(|(stream, _)| stream.index() == stream_index)
            {
                decoder.send_packet(&packet)?;
                receive_frame(&mut decoder, false)?;
            }
            decoder.send_eof()?;
            receive_frame(&mut decoder, true)?;

            Ok(output)
        })
        .await??;

        Ok(Self {
            audio_context,
            buffer,
            audio_config: output_config,
        })
    }

    /// It doesn't support pause and seek.
    pub fn play(self) -> Result<()> {
        self.audio_context.clone().load_audio(self)
    }

    pub fn slice(&self, range: std::ops::Range<Duration>) -> Result<Self> {
        let start = range.start.as_secs_f64() * self.audio_config.sample_rate as f64;
        let end = range.end.as_secs_f64() * self.audio_config.sample_rate as f64;
        let start = start as usize;
        let end = end as usize;
        let buffer = self
            .buffer
            .iter()
            .skip(start)
            .take(end - start)
            .cloned()
            .collect();
        Ok(Self {
            audio_context: self.audio_context.clone(),
            audio_config: self.audio_config,
            buffer,
        })
    }
}

impl AudioConsume for FullLoadOnceAudio {
    fn consume(&mut self, output: &mut [f32]) {
        let right = self.buffer.len().min(output.len());
        self.buffer
            .drain(..right)
            .zip(output)
            .for_each(|(src, dst)| {
                *dst += src;
            });
    }

    fn is_end(&self) -> bool {
        self.buffer.is_empty()
    }
}

use crate::media::audio::{self, AudioConfig, AudioConsume, AudioContext};
use anyhow::{bail, Result};
use namui_type::*;
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Debug, Clone)]
pub struct FullLoadRepeatAudio {
    is_playing: Arc<AtomicBool>,
    is_dropped: Arc<AtomicBool>,
    duration: Duration,
}

impl Drop for FullLoadRepeatAudio {
    fn drop(&mut self) {
        self.is_dropped
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

impl FullLoadRepeatAudio {
    pub(crate) async fn new(
        audio_context: Arc<AudioContext>,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        let output_config = audio_context.output_config;
        let path = path.as_ref().to_path_buf();
        let buffer = tokio::task::spawn_blocking(move || -> Result<Vec<f32>> {
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

            let mut output: Vec<f32> = Vec::new();

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

        let duration = Duration::from_secs_f64(
            buffer.len() as f64
                / output_config.sample_rate as f64
                / output_config.channel_count as f64,
        );

        let is_dropped = Arc::new(AtomicBool::new(false));
        let is_playing = Arc::new(AtomicBool::new(false));

        audio_context.load_audio(Inner {
            buffer,
            index: 0,
            is_dropped: is_dropped.clone(),
            is_playing: is_playing.clone(),
        })?;

        Ok(Self {
            is_playing,
            is_dropped,
            duration,
        })
    }

    /// It does nothing when you call this method on already playing audio.
    pub fn play(&self) -> Result<()> {
        self.is_playing
            .store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn pause(&self) -> Result<()> {
        self.is_playing
            .store(false, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
}

#[derive(Debug)]
struct Inner {
    buffer: Vec<f32>,
    index: usize,
    is_playing: Arc<AtomicBool>,
    is_dropped: Arc<AtomicBool>,
}

impl AudioConsume for Inner {
    fn consume(&mut self, output: &mut [f32]) {
        if !self.is_playing.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        for output_sample in output {
            *output_sample += self.buffer[self.index];
            self.index += 1;
            if self.index >= self.buffer.len() {
                self.index = 0;
            }
        }
    }

    fn is_end(&self) -> bool {
        self.is_dropped.load(std::sync::atomic::Ordering::Relaxed)
    }
}

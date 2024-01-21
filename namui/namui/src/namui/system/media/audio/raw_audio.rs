use super::AudioConsume;
use crate::media::audio::{self, AudioConfig};
use anyhow::{bail, Result};
use namui_type::{CheapSlice, Duration};
use std::path::Path;

#[derive(Clone)]
pub struct RawAudio {
    pub audio_config: AudioConfig,
    pub channels: Vec<CheapSlice<f32>>,
}

impl std::fmt::Debug for RawAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawAudio")
            .field("audio_config", &self.audio_config)
            .field("channels", &self.channels.len())
            .finish()
    }
}

impl RawAudio {
    pub async fn load(path: &impl AsRef<Path>, sample_rate: Option<u32>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let (channels, output_config) = tokio::task::spawn_blocking(move || -> Result<_> {
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

            let output_config = AudioConfig {
                sample_rate: sample_rate.unwrap_or(input_config.sample_rate),
                sample_format: ffmpeg_next::format::Sample::F32(
                    ffmpeg_next::format::sample::Type::Planar,
                ),
                channel_layout: decoder.channel_layout(),
                channel_count: decoder.channel_layout().channels() as usize,
            };

            let mut resampler = audio::get_resampler(input_config, output_config)?;

            let mut channels = vec![vec![]; output_config.channel_count];

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

                    assert!(!resampled.is_packed());

                    for (channel_index, channel) in channels.iter_mut().enumerate() {
                        let plane = resampled.plane(channel_index);
                        channel.extend(plane);
                    }

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

            Ok((
                channels.into_iter().map(CheapSlice::from_vec).collect(),
                output_config,
            ))
        })
        .await??;

        Ok(Self {
            channels,
            audio_config: output_config,
        })
    }

    pub fn slice(&self, range: std::ops::Range<Duration>) -> Self {
        let calculate_index = |duration: Duration| {
            (duration.as_secs_f64() * self.audio_config.sample_rate as f64) as usize
        };

        let start = calculate_index(range.start);
        let end = calculate_index(range.end);

        Self {
            audio_config: self.audio_config,
            channels: self
                .channels
                .iter()
                .map(|channel| channel.slice(start, end - start))
                .collect(),
        }
    }

    pub fn sample_count(&self) -> usize {
        self.channels
            .first()
            .map(|channel| channel.len())
            .unwrap_or(0)
    }
}

impl AudioConsume for RawAudio {
    fn consume(&mut self, output: &mut [f32]) {
        let output_sample_length_per_track = output.len() / 2;

        match self.channels.len() {
            0 => {}
            1 => {
                let channel = self.channels.get_mut(0).unwrap();
                let front = channel.slice_front(output_sample_length_per_track);
                for (i, sample) in front.into_iter().enumerate() {
                    output[i * 2] += sample;
                    output[i * 2 + 1] += sample;
                }
            }
            _ => {
                let channel = self.channels.get_mut(0).unwrap();
                let front = channel.slice_front(output_sample_length_per_track);
                for (i, sample) in front.into_iter().enumerate() {
                    output[i * 2] += sample;
                }

                let channel = self.channels.get_mut(1).unwrap();
                let front = channel.slice_front(output_sample_length_per_track);
                for (i, sample) in front.into_iter().enumerate() {
                    output[i * 2 + 1] += sample;
                }
            }
        }
    }

    fn is_end(&self) -> bool {
        self.channels.iter().any(|channel| channel.is_empty())
    }
}

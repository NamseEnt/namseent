use super::{atomic_floating::AtomicF32, AudioConsume};
use crate::media::audio::AudioConfig;
use anyhow::{anyhow, Result};
use std::{fmt::Debug, mem::size_of, sync::Arc};

/// Currently it implemented WASAPI IAudioClient.
/// TODO: Implement IAudioClient3 for low latency.
#[derive(Debug)]
pub(crate) struct AudioContext {
    pub output_config: AudioConfig,
    audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
    /// 0 to 1
    volume: Arc<AtomicF32>,
}

const CHANNELS: usize = 2;
const SAMPLE_RATE: usize = 44100;

impl AudioContext {
    pub fn new() -> Result<Self> {
        let volume = Arc::new(AtomicF32::new(1.0));
        let output_config = AudioConfig {
            sample_rate: SAMPLE_RATE as u32,
            sample_format: ffmpeg_next::format::Sample::F32(
                ffmpeg_next::format::sample::Type::Packed,
            ),
            channel_layout: ffmpeg_next::channel_layout::ChannelLayout::STEREO,
            channel_count: CHANNELS,
        };

        let mut audios: Vec<Box<dyn AudioConsume>> = Default::default();

        let (audio_command_tx, audio_command_rx) = std::sync::mpsc::channel();

        let handle_audio_commands = move |audios: &mut Vec<Box<dyn AudioConsume>>| {
            while let Ok(command) = audio_command_rx.try_recv() {
                match command {
                    AudioCommand::LoadAudio { audio } => {
                        audios.push(audio);
                    }
                }
            }
        };

        let (init_tx, init_rx) = std::sync::mpsc::channel();

        std::thread::spawn({
            let volume = volume.clone();
            move || {
                let InitWasApiOutput {
                    audio_client,
                    render_client,
                    blockalign,
                    h_event,
                } = match init_wasapi() {
                    Ok(ok) => {
                        init_tx.send(Ok(())).unwrap();
                        ok
                    }
                    Err(err) => {
                        init_tx.send(Err(err)).unwrap();
                        return;
                    }
                };

                loop {
                    handle_audio_commands(&mut audios);
                    let volume = volume.load(std::sync::atomic::Ordering::Relaxed);

                    let buffer_frame_count = audio_client.get_available_space_in_frames().unwrap();
                    let mut output = vec![0.0f32; buffer_frame_count as usize * CHANNELS];

                    for audio_buffer in &mut audios {
                        audio_buffer.consume(&mut output)
                    }

                    for sample in &mut output {
                        *sample *= volume;
                    }

                    render_client
                        .write_to_device(
                            buffer_frame_count as usize,
                            blockalign as usize,
                            unsafe {
                                std::slice::from_raw_parts_mut(
                                    output.as_mut_ptr() as *mut u8,
                                    output.len() * size_of::<f32>(),
                                )
                            },
                            None,
                        )
                        .unwrap();

                    audios.retain(|audio_buffer| !audio_buffer.is_end());

                    if h_event.wait_for_event(1000).is_err() {
                        crate::log!("[namui-media] failed to wait for event");
                        audio_client.stop_stream().unwrap();
                        break;
                    }
                }
            }
        });

        init_rx.recv()??;

        Ok(Self {
            output_config,
            audio_command_tx,
            volume,
        })
    }

    pub(crate) fn load_audio(&self, audio: impl AudioConsume + 'static) -> Result<()> {
        self.audio_command_tx
            .send(AudioCommand::LoadAudio {
                audio: Box::new(audio),
            })
            .map_err(|_| anyhow!("failed to send AudioCommand::LoadAudio"))?;

        Ok(())
    }

    pub(crate) fn set_volume(&self, volume: f32) {
        let volume = volume.max(0.0).min(1.0);
        self.volume
            .store(volume, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn volume(&self) -> f32 {
        self.volume.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub(crate) enum AudioCommand {
    LoadAudio { audio: Box<dyn AudioConsume> },
}

struct InitWasApiOutput {
    audio_client: wasapi::AudioClient,
    render_client: wasapi::AudioRenderClient,
    blockalign: u32,
    h_event: wasapi::Handle,
}

fn init_wasapi() -> Result<InitWasApiOutput> {
    wasapi::initialize_mta()
        .map_err(|error| anyhow!("[namui-media] failed to initialize mta: {error}",))?;

    let device = wasapi::get_default_device(&wasapi::Direction::Render)
        .map_err(|error| anyhow!("[namui-media] failed to get default device: {error}",))?;

    let mut audio_client = device
        .get_iaudioclient()
        .map_err(|error| anyhow!("[namui-media] failed to get iaudioclient: {error}",))?;

    let desired_format = wasapi::WaveFormat::new(
        32,
        32,
        &wasapi::SampleType::Float,
        SAMPLE_RATE,
        CHANNELS,
        None,
    );

    let needs_convert =
        calculate_needs_convert(&audio_client, &desired_format, &wasapi::ShareMode::Shared);

    let (_def_time, min_time) = audio_client
        .get_periods()
        .map_err(|error| anyhow!("[namui-media] failed to get periods: {error}",))?;

    audio_client
        .initialize_client(
            &desired_format,
            min_time, // I Think this min_time doesn't work. Maybe need to impl following https://learn.microsoft.com/en-us/windows-hardware/drivers/audio/low-latency-audio#windows-audio-session-api-wasapi
            &wasapi::Direction::Render,
            &wasapi::ShareMode::Shared,
            needs_convert,
        )
        .map_err(|error| anyhow!("[namui-media] failed to initialize client: {error}",))?;

    let blockalign = desired_format.get_blockalign();
    assert_eq!(blockalign as usize, std::mem::size_of::<f32>() * CHANNELS);

    let h_event = audio_client.set_get_eventhandle().unwrap();

    let render_client = audio_client.get_audiorenderclient().unwrap();

    audio_client
        .start_stream()
        .map_err(|error| anyhow!("[namui-media] failed to start stream: {error}",))?;

    Ok(InitWasApiOutput {
        audio_client,
        render_client,
        blockalign,
        h_event,
    })
}

fn calculate_needs_convert(
    audio_client: &wasapi::AudioClient,
    desired_format: &wasapi::WaveFormat,
    share_mode: &wasapi::ShareMode,
) -> bool {
    match audio_client.is_supported(desired_format, share_mode) {
        Ok(None) => {
            crate::log!("[namui-media] Device supports format {:?}", desired_format);
            false
        }
        Ok(Some(modified)) => {
            crate::log!(
                "[namui-media] Device doesn't support format:\n{:#?}\nClosest match is:\n{:#?}",
                desired_format,
                modified
            );
            true
        }
        Err(err) => {
            crate::log!(
                "[namui-media] Device doesn't support format:\n{:#?}\nError: {}",
                desired_format,
                err
            );
            crate::log!("[namui-media] Repeating query with format as WAVEFORMATEX");
            let desired_formatex = desired_format.to_waveformatex().unwrap();
            match audio_client.is_supported(&desired_formatex, share_mode) {
                Ok(None) => {
                    crate::log!(
                        "[namui-media] Device supports format {:?}",
                        desired_formatex
                    );
                    false
                }
                Ok(Some(modified)) => {
                    crate::log!(
                        "[namui-media] Device doesn't support format:\n{:#?}\nClosest match is:\n{:#?}",
                        desired_formatex, modified
                    );
                    true
                }
                Err(err) => {
                    crate::log!(
                        "[namui-media] Device doesn't support format:\n{:#?}\nError: {}",
                        desired_formatex,
                        err
                    );
                    true
                }
            }
        }
    }
}

use super::audio_handle::AudioHandle;
use crate::media::{
    audio_buffer_core::AudioBufferCore, ref_counting::RefCounting, synced_audio::SyncedAudio,
    AudioConfig,
};
use anyhow::{anyhow, Result};
use namui_type::*;
use std::{
    collections::HashMap,
    fmt::Debug,
    mem::size_of,
    sync::{atomic::AtomicBool, Arc},
};

/// Currently it implemented WASAPI IAudioClient.
/// TODO: Implement IAudioClient3 for low latency.
#[derive(Debug)]
pub(crate) struct AudioContext {
    pub output_config: AudioConfig,
    audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
}

impl AudioContext {
    pub fn new() -> Result<Self> {
        const CHANNELS: usize = 2;
        const SAMPLE_RATE: usize = 44100;

        let output_config = AudioConfig {
            sample_rate: SAMPLE_RATE as u32,
            sample_format: ffmpeg_next::format::Sample::F32(
                ffmpeg_next::format::sample::Type::Packed,
            ),
            channel_layout: ffmpeg_next::channel_layout::ChannelLayout::STEREO,
            sample_byte_size: std::mem::size_of::<f32>(),
            channel_count: CHANNELS,
        };

        type PlayingAudios = HashMap<usize, PlayingAudio>;
        let mut playing_audios: PlayingAudios = Default::default();
        let mut audio_buffer_cores: HashMap<usize, RefCounting<AudioBufferCore>> =
            Default::default();

        let (audio_command_tx, audio_command_rx) = std::sync::mpsc::channel();

        let mut handle_audio_commands = move |playing_audios: &mut PlayingAudios| {
            while let Ok(command) = audio_command_rx.try_recv() {
                match command {
                    AudioCommand::LoadAudio { audio_buffer_core } => {
                        assert!(playing_audios.iter().all(|(_, audio)| {
                            audio.synced_audio.audio_buffer_core_id() != audio_buffer_core.id()
                        }));
                        assert!(audio_buffer_cores
                            .insert(audio_buffer_core.id(), RefCounting::new(audio_buffer_core))
                            .is_none());
                    }
                    AudioCommand::IncreaseAudioRefCount {
                        audio_buffer_core_id,
                    } => {
                        audio_buffer_cores
                            .get_mut(&audio_buffer_core_id)
                            .unwrap()
                            .increase_ref_count();
                    }
                    AudioCommand::DecreaseAudioRefCount {
                        audio_buffer_core_id,
                    } => {
                        let audio_buffer_core =
                            audio_buffer_cores.get_mut(&audio_buffer_core_id).unwrap();

                        audio_buffer_core.decrease_ref_count();
                        if audio_buffer_core.is_ref_count_zero() {
                            audio_buffer_cores.remove(&audio_buffer_core_id);
                        }
                    }
                    AudioCommand::Play {
                        audio_handle_id,
                        audio_buffer_core_id,
                        is_playing,
                        start_at,
                        start_offset,
                    } => {
                        let audio_buffer_core = audio_buffer_cores
                            .get(&audio_buffer_core_id)
                            .expect("failed to get audio_buffer_core")
                            .inner_clone();

                        let synced_audio =
                            SyncedAudio::new(audio_buffer_core, start_at, start_offset);

                        playing_audios.insert(
                            audio_handle_id,
                            PlayingAudio {
                                synced_audio,
                                is_playing,
                            },
                        );
                    }
                    AudioCommand::Stop { audio_handle_id } => {
                        playing_audios.remove(&audio_handle_id);
                    }
                    AudioCommand::SeekTo {
                        audio_handle_id,
                        offset,
                    } => {
                        let audio = playing_audios
                            .get_mut(&audio_handle_id)
                            .expect("failed to get audio");

                        audio.synced_audio.seek_to(offset);
                    }
                }
            }
        };

        let (init_tx, init_rx) = std::sync::mpsc::channel();

        let init_wasapi = move || -> Result<_> {
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

            Ok((audio_client, render_client, blockalign, h_event))
        };

        std::thread::spawn(move || {
            let (audio_client, render_client, blockalign, h_event) = match init_wasapi() {
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
                handle_audio_commands(&mut playing_audios);

                let buffer_frame_count = audio_client.get_available_space_in_frames().unwrap();

                let mut output = vec![0.0f32; buffer_frame_count as usize * CHANNELS];
                let output_bytes = output.len() * size_of::<f32>();

                let now = crate::system::time::now();

                let mut finished_audio_handle_ids = vec![];

                for (audio_handle_id, audio) in playing_audios.iter_mut() {
                    let Ok(wave) = audio.synced_audio.consume(output_bytes, now) else {
                        eprintln!("[namui-media] failed to consume audio");
                        continue;
                    };

                    let wave = unsafe {
                        std::slice::from_raw_parts(
                            wave.as_ptr() as *const f32,
                            wave.len() / size_of::<f32>(),
                        )
                    };

                    for (i, sample) in wave.iter().enumerate() {
                        output[i] += *sample;
                    }

                    if audio.synced_audio.is_finished() {
                        finished_audio_handle_ids.push(*audio_handle_id);
                    }
                }

                for audio_handle_id in finished_audio_handle_ids {
                    let audio = playing_audios.remove(&audio_handle_id).unwrap();
                    audio
                        .is_playing
                        .store(false, std::sync::atomic::Ordering::SeqCst);
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

                if h_event.wait_for_event(1000).is_err() {
                    eprintln!("[namui-media] failed to wait for event");
                    audio_client.stop_stream().unwrap();
                    break;
                }
            }
        });

        init_rx.recv()??;

        Ok(Self {
            output_config,
            audio_command_tx,
        })
    }

    pub(crate) fn load_audio(&self, audio_buffer_core: AudioBufferCore) -> Result<AudioHandle> {
        let audio_buffer_core_id = audio_buffer_core.id();
        self.audio_command_tx
            .send(AudioCommand::LoadAudio { audio_buffer_core })?;

        Ok(AudioHandle::new(
            audio_buffer_core_id,
            self.audio_command_tx.clone(),
        ))
    }
}

pub(crate) enum AudioCommand {
    LoadAudio {
        audio_buffer_core: AudioBufferCore,
    },
    IncreaseAudioRefCount {
        audio_buffer_core_id: usize,
    },
    DecreaseAudioRefCount {
        audio_buffer_core_id: usize,
    },
    Play {
        audio_handle_id: usize,
        audio_buffer_core_id: usize,
        is_playing: Arc<AtomicBool>,
        start_at: Instant,
        start_offset: Duration,
    },
    Stop {
        audio_handle_id: usize,
    },
    SeekTo {
        audio_handle_id: usize,
        offset: Duration,
    },
}

fn calculate_needs_convert(
    audio_client: &wasapi::AudioClient,
    desired_format: &wasapi::WaveFormat,
    share_mode: &wasapi::ShareMode,
) -> bool {
    match audio_client.is_supported(desired_format, share_mode) {
        Ok(None) => {
            println!("[namui-media] Device supports format {:?}", desired_format);
            false
        }
        Ok(Some(modified)) => {
            println!(
                "[namui-media] Device doesn't support format:\n{:#?}\nClosest match is:\n{:#?}",
                desired_format, modified
            );
            true
        }
        Err(err) => {
            println!(
                "[namui-media] Device doesn't support format:\n{:#?}\nError: {}",
                desired_format, err
            );
            println!("[namui-media] Repeating query with format as WAVEFORMATEX");
            let desired_formatex = desired_format.to_waveformatex().unwrap();
            match audio_client.is_supported(&desired_formatex, share_mode) {
                Ok(None) => {
                    println!(
                        "[namui-media] Device supports format {:?}",
                        desired_formatex
                    );
                    false
                }
                Ok(Some(modified)) => {
                    println!(
                        "[namui-media] Device doesn't support format:\n{:#?}\nClosest match is:\n{:#?}",
                        desired_formatex, modified
                    );
                    true
                }
                Err(err) => {
                    println!(
                        "[namui-media] Device doesn't support format:\n{:#?}\nError: {}",
                        desired_formatex, err
                    );
                    true
                }
            }
        }
    }
}

struct PlayingAudio {
    synced_audio: SyncedAudio,
    is_playing: Arc<AtomicBool>,
}

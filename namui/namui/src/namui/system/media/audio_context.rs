use super::audio_handle::AudioHandle;
use crate::media::{audio_buffer_core::AudioBufferCore, synced_audio::SyncedAudio, AudioConfig};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    mem::size_of,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Debug)]
pub(crate) struct AudioContext {
    _audio_output_stream_keeper: AudioOutputStreamKeeper,
    pub output_config: AudioConfig,
    audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
}

impl AudioContext {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device.default_output_config()?;

        let output_config = AudioConfig {
            sample_rate: config.sample_rate().0,
            sample_format: {
                use cpal::SampleFormat;
                use ffmpeg_next::format::{sample::Type, Sample};
                match config.sample_format() {
                    SampleFormat::I8 => unimplemented!(),
                    SampleFormat::I16 => Sample::I16(Type::Packed),
                    SampleFormat::I32 => Sample::I32(Type::Packed),
                    SampleFormat::I64 => Sample::I64(Type::Packed),
                    SampleFormat::U8 => Sample::U8(Type::Packed),
                    SampleFormat::U16 => unimplemented!(),
                    SampleFormat::U32 => unimplemented!(),
                    SampleFormat::U64 => unimplemented!(),
                    SampleFormat::F32 => Sample::F32(Type::Packed),
                    SampleFormat::F64 => Sample::F64(Type::Packed),
                    _ => unimplemented!(),
                }
            },
            channel_layout: match config.channels() {
                1 => ffmpeg_next::channel_layout::ChannelLayout::MONO,
                2 => ffmpeg_next::channel_layout::ChannelLayout::STEREO,
                _ => unimplemented!(),
            },
            sample_byte_size: config.sample_format().sample_size(),
            channel_count: config.channels().into(),
        };

        struct PlayingAudio {
            synced_audio: SyncedAudio,
            is_playing: Arc<AtomicBool>,
        }

        let mut playing_audios: HashMap<usize, PlayingAudio> = Default::default();
        let mut audio_buffer_cores: HashMap<usize, RefCounting<AudioBufferCore>> =
            Default::default();

        let (audio_command_tx, audio_command_rx) = std::sync::mpsc::channel();

        let audio_output_stream = device.build_output_stream(
            &config.into(),
            {
                move |output: &mut [f32], _info| {
                    while let Ok(command) = audio_command_rx.try_recv() {
                        match command {
                            AudioCommand::LoadAudio { audio_buffer_core } => {
                                assert!(playing_audios.iter().all(|(_, audio)| {
                                    audio.synced_audio.audio_buffer_core_id()
                                        != audio_buffer_core.id()
                                }));
                                assert!(audio_buffer_cores
                                    .insert(
                                        audio_buffer_core.id(),
                                        RefCounting::new(audio_buffer_core)
                                    )
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
                                playback_duration,
                            } => {
                                let audio_buffer_core = audio_buffer_cores
                                    .get(&audio_buffer_core_id)
                                    .expect("failed to get audio_buffer_core")
                                    .inner_clone();

                                let buffer_byte_offset = {
                                    let buffer_byte_offset = (playback_duration.as_secs_f64()
                                        * audio_buffer_core.output_config.sample_rate as f64
                                        * audio_buffer_core.output_config.channel_count as f64
                                        * audio_buffer_core.output_config.sample_byte_size as f64)
                                        as usize;

                                    buffer_byte_offset - buffer_byte_offset % size_of::<f32>()
                                };

                                let synced_audio =
                                    SyncedAudio::new(audio_buffer_core, buffer_byte_offset);

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
                        }
                    }

                    output.fill(0.0);

                    let mut finished_audio_handle_ids = vec![];

                    for (audio_handle_id, audio) in playing_audios.iter_mut() {
                        let Ok(wave) = audio.synced_audio.consume(std::mem::size_of_val(output))
                        else {
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
                }
            },
            |err| {
                eprintln!("[namui-media] an error occurred on stream: {}", err);
            },
            None,
        )?;

        audio_output_stream.play()?;

        Ok(Self {
            _audio_output_stream_keeper: AudioOutputStreamKeeper {
                _audio_output_stream: audio_output_stream,
            },
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
        playback_duration: std::time::Duration,
    },
    Stop {
        audio_handle_id: usize,
    },
}

struct AudioOutputStreamKeeper {
    /// Don't use stream, it's not Sync and Send.
    /// I made this struct to avoid Drop, not to use stream.
    _audio_output_stream: cpal::Stream,
}

impl Debug for AudioOutputStreamKeeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioOutputStreamKeeper").finish()
    }
}

unsafe impl Send for AudioOutputStreamKeeper {}
unsafe impl Sync for AudioOutputStreamKeeper {}

struct RefCounting<T> {
    value: T,
    ref_count: usize,
}

impl<T> RefCounting<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            ref_count: 1,
        }
    }
    fn increase_ref_count(&mut self) {
        self.ref_count += 1;
    }
    fn decrease_ref_count(&mut self) {
        self.ref_count -= 1;
    }
    fn is_ref_count_zero(&self) -> bool {
        self.ref_count == 0
    }
}

impl<T: Clone> RefCounting<T> {
    fn inner_clone(&self) -> T {
        self.value.clone()
    }
}

impl<T> std::ops::Deref for RefCounting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

use super::{audio_buffer_core::AudioBufferCore, media_struct::Media, synced_audio::SyncedAudio};
use crate::MediaHandle;
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{
    collections::HashMap,
    mem::size_of,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
};

pub struct MediaContext {
    _audio_output_stream: cpal::Stream,
    pub(crate) audio_output_sample_format: cpal::SampleFormat,
    pub(crate) audio_output_channel_count: usize,
    pub(crate) audio_output_sample_rate: u32,
    audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
}

unsafe impl Send for MediaContext {}
unsafe impl Sync for MediaContext {}

enum AudioCommand {
    LoadAudio {
        audio_buffer_core: AudioBufferCore,
    },
    UnloadAudio {
        audio_buffer_core_id: usize,
    },
    Play {
        audio_handle_id: usize,
        audio_buffer_core_id: usize,
        is_playing: Arc<AtomicBool>,
    },
    Pause {
        audio_handle_id: usize,
    },
}

impl MediaContext {
    pub fn new() -> Result<MediaContext> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device.default_output_config()?;
        let audio_output_channel_count = config.channels().into();
        let audio_output_sample_format = config.sample_format();
        let audio_output_sample_rate = config.sample_rate().0;

        struct PlayingAudio {
            synced_audio: SyncedAudio,
            is_playing: Arc<AtomicBool>,
        }

        let mut playing_audios: HashMap<usize, PlayingAudio> = Default::default();
        let mut audio_buffer_cores: HashMap<usize, AudioBufferCore> = Default::default();

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
                                    .insert(audio_buffer_core.id(), audio_buffer_core)
                                    .is_none());
                            }
                            AudioCommand::UnloadAudio {
                                audio_buffer_core_id,
                            } => {
                                assert!(playing_audios.iter().all(|(_, audio)| {
                                    audio.synced_audio.audio_buffer_core_id()
                                        != audio_buffer_core_id
                                }));
                                assert!(audio_buffer_cores.remove(&audio_buffer_core_id).is_some());
                            }
                            AudioCommand::Play {
                                audio_handle_id,
                                audio_buffer_core_id,
                                is_playing,
                            } => {
                                let audio_buffer_core = audio_buffer_cores
                                    .get(&audio_buffer_core_id)
                                    .expect("failed to get audio_buffer_core")
                                    .clone();

                                let synced_audio =
                                    SyncedAudio::new(audio_handle_id, audio_buffer_core);

                                playing_audios.insert(
                                    audio_handle_id,
                                    PlayingAudio {
                                        synced_audio,
                                        is_playing,
                                    },
                                );
                            }
                            AudioCommand::Pause { audio_handle_id } => {
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

        Ok(MediaContext {
            _audio_output_stream: audio_output_stream,
            audio_output_sample_format,
            audio_output_channel_count,
            audio_output_sample_rate,
            audio_command_tx,
        })
    }

    pub(crate) fn new_media(&self, path: &impl AsRef<Path>) -> Result<MediaHandle> {
        let media = Media::new(self, path)?;

        MediaHandle::new(media)
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

/// Q. What happens if you clone an AudioHandle and play it?
/// A. Let's say it's independent. If you don't want it to be independent, you have to wrap it in Arc.
///
/// Q. What happens if you play an already playing AudioHandle again?
/// A. Nothing happens. (If you play an already playing AudioHandle again, it is ignored.)
#[derive(Debug)]
struct AudioHandle {
    id: usize,
    audio_buffer_core_id: usize,
    audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
    is_playing: bool,
    last_playback_playing: Option<Arc<AtomicBool>>,
}

impl AudioHandle {
    pub(crate) fn new(
        audio_buffer_core_id: usize,
        audio_command_tx: std::sync::mpsc::Sender<AudioCommand>,
    ) -> Self {
        static AUDIO_HANDLE_ID: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: AUDIO_HANDLE_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            audio_buffer_core_id,
            audio_command_tx,
            is_playing: false,
            last_playback_playing: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
            && self
                .last_playback_playing
                .as_ref()
                .unwrap()
                .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn play(&mut self) {
        if self.is_playing {
            return;
        }
        self.is_playing = true;
        let last_playback_playing = Arc::new(AtomicBool::new(true));
        self.last_playback_playing = Some(last_playback_playing.clone());

        self.audio_command_tx
            .send(AudioCommand::Play {
                audio_handle_id: self.id,
                audio_buffer_core_id: self.audio_buffer_core_id,
                is_playing: last_playback_playing,
            })
            .expect("failed to send AudioCommand::Play");
    }

    pub fn pause(&mut self) {
        if !self.is_playing {
            return;
        }
        self.is_playing = false;
        self.last_playback_playing = None;

        self.audio_command_tx
            .send(AudioCommand::Pause {
                audio_handle_id: self.id,
            })
            .expect("failed to send AudioCommand::Pause");
    }
}

impl Drop for AudioHandle {
    fn drop(&mut self) {
        self.audio_command_tx
            .send(AudioCommand::UnloadAudio {
                audio_buffer_core_id: self.audio_buffer_core_id,
            })
            .expect("failed to send audio");
    }
}

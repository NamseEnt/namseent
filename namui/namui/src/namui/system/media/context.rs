use super::{media_struct::Media, synced_audio::SyncedAudio};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{
    mem::size_of,
    path::Path,
    sync::{Arc, Mutex},
};

pub struct MediaContext {
    _audio_output_stream: cpal::Stream,
    pub(crate) audio_output_sample_format: cpal::SampleFormat,
    pub(crate) audio_output_channel_count: usize,
    pub(crate) audio_output_sample_rate: u32,
    audio_tx: std::sync::mpsc::Sender<SyncedAudio>,
}

unsafe impl Send for MediaContext {}
unsafe impl Sync for MediaContext {}

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

        let mut playing_audios: Vec<SyncedAudio> = Vec::new();

        let (audio_tx, audio_rx) = std::sync::mpsc::channel();

        let audio_output_stream = device.build_output_stream(
            &config.into(),
            {
                move |output: &mut [f32], _info| {
                    while let Ok(buffer) = audio_rx.try_recv() {
                        playing_audios.push(buffer);
                    }

                    output.fill(0.0);

                    for audio in playing_audios.iter_mut() {
                        let Ok(wave) = audio.consume(std::mem::size_of_val(output)) else {
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
                    }

                    playing_audios.retain(|audio| !audio.is_finished());
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
            audio_tx,
        })
    }

    pub(crate) fn new_media(&self, path: &impl AsRef<Path>) -> Result<MediaHandle> {
        let media = Media::new(self, path)?;

        Ok(Arc::new(Mutex::new(media)))
    }

    pub(crate) fn play(&self, media: &mut Media) {
        let audio = media.play();

        if let Some(audio) = audio {
            self.audio_tx.send(audio).expect("failed to send audio");
        }
    }

    // pub fn play(&self, audio_source: &AudioSource) -> AudioPlayHandle {
    //     let wave_stream = WaveStream {
    //         read_buffer_index: 0,
    //         read_frame_index: 0,
    //         audio_source: audio_source.clone(),
    //     };

    //     self.playing_wave_stream_tx
    //         .send(wave_stream)
    //         .expect("failed to send wave stream");

    //     AudioPlayHandle {}
    // }

    // pub fn stop(&mut self, _audio_play_handle: AudioPlayHandle) {
    //     todo!()
    // }
}

pub type MediaHandle = Arc<Mutex<Media>>;

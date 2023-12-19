use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use std::{
    rc::Rc,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use symphonia::core::audio::Signal;

pub struct AudioContext {
    _output_stream: cpal::Stream,
    playing_wave_stream_tx: std::sync::mpsc::Sender<WaveStream>,
}

unsafe impl Send for AudioContext {}
unsafe impl Sync for AudioContext {}

impl AudioContext {
    pub fn new() -> Result<AudioContext> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device.default_output_config()?;
        let channels = config.channels();

        let mut playing_wave_streams: Vec<WaveStream> = Vec::new();

        let (tx, rx) = std::sync::mpsc::channel();

        let output_stream = device.build_output_stream(
            &config.into(),
            {
                move |output: &mut [f32], _info| {
                    while let Ok(buffer) = rx.try_recv() {
                        playing_wave_streams.push(buffer);
                    }

                    for wave_stream in playing_wave_streams.iter_mut() {
                        wave_stream.consume(output, channels as usize)
                    }

                    playing_wave_streams.retain(|wave_stream| !wave_stream.is_finished());
                }
            },
            |err| {
                eprintln!("[namui-audio] an error occurred on stream: {}", err);
            },
            None,
        )?;

        Ok(AudioContext {
            _output_stream: output_stream,
            playing_wave_stream_tx: tx,
        })
    }

    pub fn play(&self, audio_source: &AudioSource) -> AudioPlayHandle {
        let wave_stream = WaveStream {
            read_vec_index: 0,
            read_vec_item_index: 0,
            audio_source: audio_source.clone(),
        };

        self.playing_wave_stream_tx
            .send(wave_stream)
            .expect("failed to send wave stream");

        AudioPlayHandle {}
    }

    pub fn stop(&mut self, _audio_play_handle: AudioPlayHandle) {
        todo!()
    }
}

pub struct WaveStream {
    read_vec_index: usize,
    read_vec_item_index: usize,
    audio_source: AudioSource,
}

unsafe impl Send for WaveStream {}

impl WaveStream {
    pub fn is_finished(&self) -> bool {
        if !self
            .audio_source
            .decode_finished
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            return false;
        }

        let mut audio_buffers = self.audio_source.audio_buffers.lock().unwrap();
        if self.read_vec_index < audio_buffers.len() {
            return false;
        }

        match self.audio_source.rx.try_recv() {
            Ok(buffer) => {
                audio_buffers.push(buffer);
                false
            }
            Err(err) => match err {
                std::sync::mpsc::TryRecvError::Empty => true,
                std::sync::mpsc::TryRecvError::Disconnected => true,
            },
        }
    }
    pub fn consume(&mut self, output: &mut [f32], channels: usize) {
        let mut audio_buffers = self.audio_source.audio_buffers.lock().unwrap();

        if let Ok(buffer) = self.audio_source.rx.try_recv() {
            audio_buffers.push(buffer);
        }

        let Some(mut audio_buffer) = audio_buffers.get(self.read_vec_index) else {
            return;
        };

        let mut channel_buffers = (0..channels)
            .map(|channel| audio_buffer.chan(channel))
            .collect::<Vec<_>>();

        for frame in output.chunks_mut(channels) {
            for (channel, sample) in frame.iter_mut().enumerate() {
                *sample = channel_buffers[channel][self.read_vec_item_index];
            }

            self.read_vec_item_index += 1;

            if self.read_vec_item_index >= channel_buffers[0].len() {
                self.read_vec_item_index = 0;
                self.read_vec_index += 1;

                if let Ok(buffer) = self.audio_source.rx.try_recv() {
                    audio_buffers.push(buffer);
                }

                let Some(next) = audio_buffers.get(self.read_vec_index) else {
                    return;
                };
                audio_buffer = next;
                channel_buffers = (0..channels)
                    .map(|channel| audio_buffer.chan(channel))
                    .collect::<Vec<_>>();

                assert!(!channel_buffers[0].is_empty());
            }
        }
    }
}

#[derive(Clone)]
pub struct AudioSource {
    audio_buffers: Arc<Mutex<Vec<symphonia::core::audio::AudioBuffer<f32>>>>,
    decode_finished: Arc<AtomicBool>,
    rx: Rc<std::sync::mpsc::Receiver<symphonia::core::audio::AudioBuffer<f32>>>,
}

impl AudioSource {
    pub fn new(
        hint_extension: Option<&str>,
        hint_mime_type: Option<&str>,
        stream: std::fs::File,
    ) -> Result<AudioSource> {
        let mut hint = symphonia::core::probe::Hint::new();
        if let Some(ext) = hint_extension {
            hint.with_extension(ext);
        }
        if let Some(mime) = hint_mime_type {
            hint.mime_type(mime);
        }

        let (tx, rx) = std::sync::mpsc::channel();
        let decode_finished = Arc::new(AtomicBool::new(false));

        let audio_source = AudioSource {
            audio_buffers: Arc::new(Mutex::new(Vec::new())),
            decode_finished: decode_finished.clone(),
            rx: Rc::new(rx),
        };

        std::thread::spawn(move || decode_thread_inner(hint, stream, tx, decode_finished));

        Ok(audio_source)
    }
}

fn decode_thread_inner(
    hint: symphonia::core::probe::Hint,
    stream: std::fs::File,
    tx: std::sync::mpsc::Sender<symphonia::core::audio::AudioBuffer<f32>>,
    decode_finished: Arc<AtomicBool>,
) -> Result<()> {
    let media_source_stream =
        symphonia::core::io::MediaSourceStream::new(Box::new(stream), Default::default());

    let mut probe_result = symphonia::default::get_probe().format(
        &hint,
        media_source_stream,
        &Default::default(),
        &Default::default(),
    )?;

    struct TrackDecoder {
        codec: Box<dyn symphonia::core::codecs::Decoder>,
        track_id: u32,
    }

    let mut track_decoders = probe_result
        .format
        .tracks()
        .iter()
        .map(|track| {
            symphonia::default::get_codecs()
                .make(&track.codec_params, &Default::default())
                .map(|codec| TrackDecoder {
                    codec,
                    track_id: track.id,
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    loop {
        match probe_result.format.next_packet() {
            Ok(packet) => {
                let track_id = packet.track_id();
                let audio_buffer_ref = track_decoders
                    .iter_mut()
                    .find(|track_decoder| track_decoder.track_id == track_id)
                    .unwrap()
                    .codec
                    .decode(&packet)
                    .unwrap();

                let audio_buffer = audio_buffer_ref.make_equivalent::<f32>();
                tx.send(audio_buffer).unwrap();
            }
            Err(err) => {
                if let symphonia::core::errors::Error::IoError(err) = err {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        decode_finished.store(true, std::sync::atomic::Ordering::SeqCst);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

pub struct AudioPlayHandle {}

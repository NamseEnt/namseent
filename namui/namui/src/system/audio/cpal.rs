use super::InterleavedAllSamples;
use crate::system::InitResult;
use anyhow::{Result, anyhow, bail};
use cpal::{traits::*, *};
use dashmap::DashMap;
use rubato::Resampler;
use std::{
    fmt::Debug,
    sync::{
        Arc, OnceLock, Weak,
        atomic::{AtomicU32, AtomicUsize},
    },
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const DEFAULT_SAMPLE_RATE: usize = 48_000;

static PLAY_REQUEST_TX: OnceLock<UnboundedSender<PlayRequest>> = OnceLock::new();
fn play_request_tx() -> &'static UnboundedSender<PlayRequest> {
    PLAY_REQUEST_TX.get().unwrap()
}
static AUDIO_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PLAYBACK_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PLAYBACKS: OnceLock<DashMap<usize, Playback>> = OnceLock::new();
fn playbacks() -> &'static DashMap<usize, Playback> {
    PLAYBACKS.get_or_init(Default::default)
}
static LAST_DEVICE_SAMPLE_RATE: AtomicUsize = AtomicUsize::new(DEFAULT_SAMPLE_RATE);
fn last_device_sample_rate() -> usize {
    LAST_DEVICE_SAMPLE_RATE.load(std::sync::atomic::Ordering::Relaxed)
}
static ALL_SAMPLE_STORES: OnceLock<DashMap<usize, Weak<SampleStore>>> = OnceLock::new();
fn all_sample_stores() -> &'static DashMap<usize, Weak<SampleStore>> {
    ALL_SAMPLE_STORES.get_or_init(Default::default)
}

pub(super) async fn init() -> InitResult {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    PLAY_REQUEST_TX.set(tx).unwrap();

    tokio::spawn(async move {
        let host = cpal::default_host();

        loop {
            if let Err(error) = tick_in_checking_device_change(&host, &mut rx).await {
                eprintln!("NAMUI: audio tick error: {error:?}");
            };
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    Ok(())
}

#[derive(Clone)]
pub struct Audio {
    audio_id: usize,
    sample_store: Arc<SampleStore>,
}

impl Audio {
    pub(crate) fn new(all_samples: InterleavedAllSamples) -> Self {
        let interleaved_samples = all_samples.into_iter().flatten().collect::<Vec<f32>>();
        let planar_samples = {
            let mut left_samples = Vec::with_capacity(interleaved_samples.len() / 2);
            let mut right_samples = Vec::with_capacity(interleaved_samples.len() / 2);
            for i in 0..interleaved_samples.len() / 2 {
                left_samples.push(interleaved_samples[i * 2]);
                right_samples.push(interleaved_samples[i * 2 + 1]);
            }
            [left_samples, right_samples]
        };

        let sample_len_in_default_sample_rate = planar_samples[0].len();
        let audio_id = AUDIO_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let streams = DashMap::new();
        streams.insert(
            DEFAULT_SAMPLE_RATE,
            Some(Arc::new(LinkedStream {
                data: planar_samples,
                next: Default::default(),
            })),
        );

        let sample_store = Arc::new(SampleStore {
            id: {
                static SAMPLE_STORE_ID: AtomicUsize = AtomicUsize::new(0);
                SAMPLE_STORE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            },
            sample_len_in_default_sample_rate,
            streams,
        });

        all_sample_stores().insert(sample_store.id, Arc::downgrade(&sample_store));
        start_resample(last_device_sample_rate(), &sample_store);

        Audio {
            audio_id,
            sample_store,
        }
    }
    pub fn play(&self) -> PlayHandle {
        self.play_impl(PlayRequestKind::Play)
    }
    pub fn play_repeat(&self) -> PlayHandle {
        self.play_impl(PlayRequestKind::PlayRepeat)
    }
    pub fn play_and_forget(&self) {
        self.play_impl(PlayRequestKind::PlayAndForget);
    }

    fn play_impl(&self, kind: PlayRequestKind) -> PlayHandle {
        let playback_id = PLAYBACK_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        play_request_tx()
            .send(PlayRequest::Play {
                playback_id,
                sample_store: self.sample_store.clone(),
                repeat: matches!(kind, PlayRequestKind::PlayRepeat),
            })
            .unwrap();

        PlayHandle {
            playback_id,
            is_play_and_forget: matches!(kind, PlayRequestKind::PlayAndForget),
        }
    }
}

enum PlayRequestKind {
    Play,
    PlayRepeat,
    PlayAndForget,
}

#[derive(Debug)]
pub struct PlayHandle {
    playback_id: usize,
    is_play_and_forget: bool,
}

impl Drop for PlayHandle {
    fn drop(&mut self) {
        if !self.is_play_and_forget {
            play_request_tx()
                .send(PlayRequest::Stop {
                    playback_id: self.playback_id,
                })
                .unwrap();
        }
    }
}

static VOLUME: OnceLock<AtomicU32> = OnceLock::new();
pub(crate) fn set_volume(zero_to_one: f32) {
    let volume = VOLUME.get_or_init(|| AtomicU32::new(u32::MAX));
    volume.store(
        (zero_to_one * (u32::MAX as f32)) as u32,
        std::sync::atomic::Ordering::Relaxed,
    );
}

pub(crate) fn volume() -> f32 {
    let volume = VOLUME.get_or_init(|| AtomicU32::new(u32::MAX));
    volume.load(std::sync::atomic::Ordering::Relaxed) as f32 / u32::MAX as f32
}

async fn tick_in_checking_device_change(
    host: &Host,
    play_request_rx: &mut UnboundedReceiver<PlayRequest>,
) -> Result<()> {
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("no default output device available"))?;

    println!("NAMUI: audio device selected: {:?}", device.name());

    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate().0;

    LAST_DEVICE_SAMPLE_RATE.store(sample_rate as usize, std::sync::atomic::Ordering::Relaxed);

    start_resample_all_sample_stores(sample_rate as usize);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::I16 => run::<i16>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::I32 => run::<i32>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::I64 => run::<i64>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::U8 => run::<u8>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::U16 => run::<u16>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::U32 => run::<u32>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::U64 => run::<u64>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::F32 => run::<f32>(device, config.into(), play_request_rx).await?,
        cpal::SampleFormat::F64 => run::<f64>(device, config.into(), play_request_rx).await?,
        sample_format => bail!("Unsupported sample format: {:?}", sample_format),
    };

    return Ok(());

    async fn run<T>(
        device: cpal::Device,
        config: cpal::StreamConfig,
        play_request_rx: &mut UnboundedReceiver<PlayRequest>,
    ) -> Result<()>
    where
        T: SizedSample + Debug + FromSample<f32>,
        <T as Sample>::Signed: FromSample<f32>,
    {
        println!("NAMUI: audio config: {config:?}");
        let (err_tx, mut err_rx) = tokio::sync::oneshot::channel();
        let mut err_tx = Some(err_tx);

        let err_fn = move |err| {
            eprintln!("NAMUI: an error occurred on audio output stream: {err}");

            if let StreamError::DeviceNotAvailable = err
                && let Some(err_tx) = err_tx.take()
            {
                let _ = err_tx.send(());
            }
        };

        let sample_rate = config.sample_rate.0 as usize;

        let _stream = {
            let stream = device.build_output_stream(
                &config,
                move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                    let mut finished_playback_ids = vec![];

                    let mut output_f32 = vec![0.0; output.len()];

                    for mut playback in playbacks().iter_mut() {
                        let is_done = playback.play(sample_rate, &mut output_f32);
                        if is_done {
                            finished_playback_ids.push(playback.playback_id);
                        }
                    }
                    let volume = volume();

                    output_f32.iter_mut().for_each(|sample| *sample *= volume);

                    for (output, input) in output.iter_mut().zip(output_f32.iter()) {
                        *output = input.to_sample();
                    }

                    for playback_id in finished_playback_ids {
                        playbacks().remove(&playback_id);
                    }
                },
                err_fn,
                None,
            )?;
            stream.play()?;
            // Because the stream is not Send, but we need to send it to the tokio task.
            SendableStream { _stream: stream }
        };

        loop {
            tokio::select! {
                err = &mut err_rx => {
                    err?;
                },
                play_request = play_request_rx.recv() => {
                    let play_request = play_request.unwrap();
                    match play_request {
                        PlayRequest::Play {
                            playback_id,
                            repeat,
                            sample_store,
                        } => {
                            start_resample(sample_rate, &sample_store);
                            playbacks().insert(
                                playback_id,
                                Playback {
                                    playback_id,
                                    repeat,
                                    stream: sample_store.stream(sample_rate),
                                    sample_store,
                                    offset: 0,
                                    sample_rate,
                                    offset_in_stream: 0,
                                    skipped_samples: 0,
                                },
                            );
                        }
                        PlayRequest::Stop { playback_id } => {
                            playbacks().remove(&playback_id);
                        }
                    }
                },
            }
        }
    }
}

fn start_resample_all_sample_stores(sample_rate: usize) {
    if sample_rate == DEFAULT_SAMPLE_RATE {
        return;
    }

    for sample_store in all_sample_stores() {
        let Some(sample_store) = sample_store.upgrade() else {
            continue;
        };
        start_resample(sample_rate, &sample_store);
    }
}

fn start_resample(sample_rate: usize, sample_store: &SampleStore) {
    if sample_rate == DEFAULT_SAMPLE_RATE {
        return;
    }

    let mut stream = {
        let mut entry = sample_store.streams.entry(sample_rate).or_default();

        let is_already_in_progress = entry.is_some();
        if is_already_in_progress {
            return;
        }

        let stream = Arc::new(LinkedStream {
            data: Default::default(),
            next: Default::default(),
        });

        *entry = Some(stream.clone());

        stream
    };
    let default_sample_stream = sample_store.stream(DEFAULT_SAMPLE_RATE);

    rayon::spawn(move || {
        let original_samples = &default_sample_stream.data;

        let chunk_size = 4096;
        let mut resampler =
            rubato::FftFixedIn::new(DEFAULT_SAMPLE_RATE, sample_rate, chunk_size, 2, 2).unwrap();

        let input_sample_len = original_samples[0].len();
        for start_index in (0..input_sample_len).step_by(chunk_size) {
            let end_index = (start_index + chunk_size).min(input_sample_len);
            let wave_in = [
                &original_samples[0][start_index..end_index],
                &original_samples[1][start_index..end_index],
            ];
            let mut output = if end_index - start_index < chunk_size {
                resampler.process_partial(Some(&wave_in), None).unwrap()
            } else {
                resampler.process(&wave_in, None).unwrap()
            };

            let next_stream = Arc::new(LinkedStream {
                data: [output.swap_remove(0), output.swap_remove(0)],
                next: Default::default(),
            });

            stream
                .next
                .set(next_stream.clone())
                .map_err(|_| unreachable!())
                .unwrap();

            stream = next_stream;
        }
    });
}

struct Playback {
    playback_id: usize,
    repeat: bool,
    sample_store: Arc<SampleStore>,
    offset: usize,
    sample_rate: usize,
    stream: Arc<LinkedStream>,
    offset_in_stream: usize,
    skipped_samples: usize,
}

impl Playback {
    fn play(&mut self, sample_rate: usize, mut output: &mut [f32]) -> bool {
        let sample_len = self.sample_store.sample_len(sample_rate);
        // TODO: Handle mono output.

        if self.sample_rate != sample_rate {
            self.offset = self.offset * sample_rate / self.sample_rate;
            self.stream = self.sample_store.stream(sample_rate);
            self.offset_in_stream = 0;
            self.skipped_samples = self.offset;
            self.sample_rate = sample_rate;
        }

        while self.skipped_samples > 0 {
            if self.offset_in_stream < self.stream.len() {
                let advance = self
                    .skipped_samples
                    .min(self.stream.len() - self.offset_in_stream);
                self.skipped_samples -= advance;
                self.offset_in_stream += advance;

                continue;
            }

            match self.stream.next() {
                Some(next_stream) => {
                    self.stream = next_stream;
                    self.offset_in_stream = 0;
                    continue;
                }
                None => {
                    self.skipped_samples += output.len() / 2;
                    self.offset += output.len() / 2;

                    if self.repeat && sample_len <= self.offset {
                        self.skipped_samples = self.offset - sample_len;
                        self.offset -= sample_len;
                        self.offset_in_stream = 0;
                        self.stream = self.sample_store.stream(sample_rate);
                        continue;
                    }

                    return !self.repeat && sample_len <= self.offset;
                }
            }
        }

        loop {
            let start_index = self.offset_in_stream;
            let end_index = (start_index + output.len() / 2).min(self.stream.len());

            for i in start_index..end_index {
                let left_sample = self.stream.data[0][i];
                let right_sample = self.stream.data[1][i];
                output[0] += left_sample;
                output[1] += right_sample;
                output = &mut output[2..];
            }
            self.offset += end_index - start_index;
            self.offset_in_stream = end_index;

            if end_index < self.stream.len() {
                self.skipped_samples = output.len() / 2;
                return false;
            }

            match self.stream.next() {
                Some(next_stream) => {
                    self.stream = next_stream;
                    self.offset_in_stream = 0;
                }
                None => {
                    if self.offset < sample_len {
                        self.skipped_samples = output.len() / 2;
                        return false;
                    }

                    if self.repeat {
                        self.offset = 0;
                        self.offset_in_stream = 0;
                        self.stream = self.sample_store.stream(sample_rate);
                    } else {
                        return true;
                    }
                }
            }
        }
    }
}

impl std::fmt::Debug for Audio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Audio")
            .field("audio_id", &self.audio_id)
            .finish()
    }
}

struct SampleStore {
    id: usize,
    sample_len_in_default_sample_rate: usize,
    streams: DashMap<usize, Option<Arc<LinkedStream>>>,
}

impl Drop for SampleStore {
    fn drop(&mut self) {
        all_sample_stores().remove(&self.id);
    }
}

impl SampleStore {
    fn sample_len(&self, sample_rate: usize) -> usize {
        self.sample_len_in_default_sample_rate * sample_rate / DEFAULT_SAMPLE_RATE
    }
    fn stream(&self, sample_rate: usize) -> Arc<LinkedStream> {
        self.streams
            .get(&sample_rate)
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
}

struct LinkedStream {
    data: [Vec<f32>; 2],
    next: OnceLock<Arc<LinkedStream>>,
}

impl LinkedStream {
    fn len(&self) -> usize {
        self.data[0].len()
    }
    fn next(&self) -> Option<Arc<LinkedStream>> {
        self.next.get().cloned()
    }
}

enum PlayRequest {
    Play {
        playback_id: usize,
        repeat: bool,
        sample_store: Arc<SampleStore>,
    },
    Stop {
        playback_id: usize,
    },
}

/// https://github.com/RustAudio/cpal/issues/818
struct SendableStream {
    _stream: cpal::Stream,
}
unsafe impl Send for SendableStream {}

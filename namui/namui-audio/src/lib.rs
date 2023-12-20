mod media;
mod synced_audio;

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use media::*;
use std::{
    fmt::{Debug, Formatter},
    rc::Rc,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
pub use synced_audio::*;

// pub struct AudioContext {
//     _output_stream: cpal::Stream,
//     playing_wave_stream_tx: std::sync::mpsc::Sender<WaveStream>,
// }

// unsafe impl Send for AudioContext {}
// unsafe impl Sync for AudioContext {}

// impl AudioContext {
//     pub fn new() -> Result<AudioContext> {
//         // ffmpeg_next::decoder::new()
//         //     .open_as(ffmpeg_next::decoder::find(ffmpeg_next::codec::Id::MP3))
//         //     .unwrap()
//         //     .audio();

//         let host = cpal::default_host();
//         let device = host
//             .default_output_device()
//             .expect("no output device available");

//         let config = device.default_output_config()?;
//         let channels = config.channels();

//         let mut playing_wave_streams: Vec<WaveStream> = Vec::new();

//         let (tx, rx) = std::sync::mpsc::channel();

//         let output_stream = device.build_output_stream(
//             &config.into(),
//             {
//                 move |output: &mut [f32], _info| {
//                     while let Ok(buffer) = rx.try_recv() {
//                         playing_wave_streams.push(buffer);
//                     }

//                     output.fill(0.0);

//                     for wave_stream in playing_wave_streams.iter_mut() {
//                         wave_stream.consume(output, channels as usize)
//                     }

//                     playing_wave_streams.retain(|wave_stream| !wave_stream.is_finished());
//                 }
//             },
//             |err| {
//                 eprintln!("[namui-audio] an error occurred on stream: {}", err);
//             },
//             None,
//         )?;

//         output_stream.play()?;

//         Ok(AudioContext {
//             _output_stream: output_stream,
//             playing_wave_stream_tx: tx,
//         })
//     }

//     pub fn play(&self, audio_source: &AudioSource) -> AudioPlayHandle {
//         let wave_stream = WaveStream {
//             read_buffer_index: 0,
//             read_frame_index: 0,
//             audio_source: audio_source.clone(),
//         };

//         self.playing_wave_stream_tx
//             .send(wave_stream)
//             .expect("failed to send wave stream");

//         AudioPlayHandle {}
//     }

//     pub fn stop(&mut self, _audio_play_handle: AudioPlayHandle) {
//         todo!()
//     }
// }

// pub struct WaveStream {
//     read_buffer_index: usize,
//     read_frame_index: usize,
//     audio_source: AudioSource,
// }

// unsafe impl Send for WaveStream {}

// impl WaveStream {
//     pub fn is_finished(&self) -> bool {
//         if !self
//             .audio_source
//             .decode_finished
//             .load(std::sync::atomic::Ordering::SeqCst)
//         {
//             return false;
//         }

//         let mut audio_buffers = self.audio_source.audio_buffers.lock().unwrap();
//         if self.read_buffer_index < audio_buffers.len() {
//             return false;
//         }

//         match self.audio_source.rx.try_recv() {
//             Ok(buffer) => {
//                 audio_buffers.push(buffer);
//                 false
//             }
//             Err(err) => match err {
//                 std::sync::mpsc::TryRecvError::Empty => true,
//                 std::sync::mpsc::TryRecvError::Disconnected => true,
//             },
//         }
//     }
//     pub fn consume(&mut self, output: &mut [f32], channels: usize) {
//         let mut audio_buffers = self.audio_source.audio_buffers.lock().unwrap();

//         if let Ok(buffer) = self.audio_source.rx.try_recv() {
//             audio_buffers.push(buffer);
//         }

//         let Some(mut audio_buffer) = audio_buffers.get(self.read_buffer_index) else {
//             return;
//         };

//         let mut channel_buffers = (0..channels)
//             .map(|channel| audio_buffer.chan(channel))
//             .collect::<Vec<_>>();

//         for frame in output.chunks_mut(channels) {
//             while self.read_frame_index >= audio_buffer.frames() {
//                 self.read_frame_index = 0;
//                 self.read_buffer_index += 1;

//                 if let Ok(buffer) = self.audio_source.rx.try_recv() {
//                     audio_buffers.push(buffer);
//                 }

//                 let Some(next) = audio_buffers.get(self.read_buffer_index) else {
//                     return;
//                 };
//                 audio_buffer = next;
//                 channel_buffers = (0..channels)
//                     .map(|channel| audio_buffer.chan(channel))
//                     .collect::<Vec<_>>();
//             }

//             for (channel, sample) in frame.iter_mut().enumerate() {
//                 let channel_buffer = channel_buffers[channel];
//                 *sample += channel_buffer[self.read_frame_index];
//             }

//             self.read_frame_index += 1;
//         }
//     }
// }

// #[derive(Clone)]
// pub struct AudioSource {
//     audio_buffers: Arc<Mutex<Vec<symphonia::core::audio::AudioBuffer<f32>>>>,
//     decode_finished: Arc<AtomicBool>,
//     rx: Rc<std::sync::mpsc::Receiver<symphonia::core::audio::AudioBuffer<f32>>>,
// }

// impl Debug for AudioSource {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("AudioSource")
//             .field("audio_buffers", &self.audio_buffers.lock().unwrap().len())
//             .field("decode_finished", &self.decode_finished)
//             .finish()
//     }
// }

// unsafe impl Send for AudioSource {}
// unsafe impl Sync for AudioSource {}

// impl AudioSource {
//     pub fn new(
//         hint_extension: Option<&str>,
//         hint_mime_type: Option<&str>,
//         stream: Vec<u8>,
//     ) -> Result<AudioSource> {
//         let mut hint = symphonia::core::probe::Hint::new();
//         if let Some(ext) = hint_extension {
//             hint.with_extension(ext);
//         }
//         if let Some(mime) = hint_mime_type {
//             hint.mime_type(mime);
//         }

//         let (tx, rx) = std::sync::mpsc::channel();
//         let decode_finished = Arc::new(AtomicBool::new(false));

//         let audio_source = AudioSource {
//             audio_buffers: Arc::new(Mutex::new(Vec::new())),
//             decode_finished: decode_finished.clone(),
//             rx: Rc::new(rx),
//         };

//         std::thread::spawn(move || {
//             decode_thread_inner(hint, stream, tx, decode_finished).unwrap();
//         });

//         Ok(audio_source)
//     }
// }

// fn decode_thread_inner(
//     hint: symphonia::core::probe::Hint,
//     stream: Vec<u8>,
//     tx: std::sync::mpsc::Sender<symphonia::core::audio::AudioBuffer<f32>>,
//     decode_finished: Arc<AtomicBool>,
// ) -> Result<()> {
//     let media_source_stream = symphonia::core::io::MediaSourceStream::new(
//         Box::new(std::io::Cursor::new(stream)),
//         Default::default(),
//     );

//     let mut probe_result = symphonia::default::get_probe().format(
//         &hint,
//         media_source_stream,
//         &Default::default(),
//         &Default::default(),
//     )?;

//     struct TrackDecoder {
//         codec: Box<dyn symphonia::core::codecs::Decoder>,
//         track_id: u32,
//     }

//     let mut track_decoders = probe_result
//         .format
//         .tracks()
//         .iter()
//         .map(|track| {
//             symphonia::default::get_codecs()
//                 .make(&track.codec_params, &Default::default())
//                 .map(|codec| TrackDecoder {
//                     codec,
//                     track_id: track.id,
//                 })
//         })
//         .collect::<Result<Vec<_>, _>>()
//         .unwrap();
//     loop {
//         match probe_result.format.next_packet() {
//             Ok(packet) => {
//                 let track_id = packet.track_id();
//                 let audio_buffer_ref = track_decoders
//                     .iter_mut()
//                     .find(|track_decoder| track_decoder.track_id == track_id)
//                     .unwrap()
//                     .codec
//                     .decode(&packet)
//                     .unwrap();

//                 let mut audio_buffer = audio_buffer_ref.make_equivalent::<f32>();
//                 audio_buffer_ref.convert(&mut audio_buffer);
//                 tx.send(audio_buffer).unwrap();
//             }
//             Err(err) => {
//                 if let symphonia::core::errors::Error::IoError(err) = &err {
//                     if err.kind() == std::io::ErrorKind::UnexpectedEof {
//                         decode_finished.store(true, std::sync::atomic::Ordering::SeqCst);
//                         break;
//                     }
//                 }
//                 panic!("[namui-audio] an error occurred on decode: {}", err);
//             }
//         }
//     }

//     Ok(())
// }

// pub struct AudioPlayHandle {}

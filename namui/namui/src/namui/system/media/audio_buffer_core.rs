// use super::AudioConfig;
// use anyhow::Result;
// use dashmap::DashMap;
// use std::sync::{atomic::AtomicBool, Arc};

// const BUFFER_MAX_SIZE: usize = 4096;

// #[derive(Debug, Clone)]
// pub struct AudioBufferCore {
//     id: usize,
//     /// Buffer size is limited to BUFFER_MAX_SIZE. Only last value can be smaller than BUFFER_MAX_SIZE.
//     buffers: Arc<DashMap<usize, Arc<Vec<u8>>>>,
//     done: Arc<AtomicBool>,
//     pub(crate) output_config: AudioConfig,
// }

// impl AudioBufferCore {
//     pub(crate) fn new(
//         frame_rx: crossbeam_channel::Receiver<ffmpeg_next::frame::Audio>,
//         input_config: AudioConfig,
//         output_config: AudioConfig,
//     ) -> Result<Self> {
//         let id = generate_audio_buffer_core_id();

//         let buffers = Arc::new(DashMap::new());
//         let done = Arc::new(AtomicBool::new(false));

//         std::thread::spawn({
//             let buffers = buffers.clone();
//             let done = done.clone();
//             move || {
//                 let result = (move || -> Result<()> {
//                     let mut resampler = ffmpeg_next::software::resampling::Context::get(
//                         input_config.sample_format,
//                         input_config.channel_layout,
//                         input_config.sample_rate,
//                         output_config.sample_format,
//                         if output_config.channel_count == 1 {
//                             ffmpeg_next::ChannelLayout::MONO
//                         } else {
//                             ffmpeg_next::ChannelLayout::STEREO
//                         },
//                         output_config.sample_rate,
//                     )?;

//                     let mut next_frame_index = 0;
//                     let mut queue = vec![];

//                     while let Ok(frame) = frame_rx.recv() {
//                         let mut resampled = ffmpeg_next::frame::Audio::empty();
//                         if let Some(delay) = resampler.run(&frame, &mut resampled)? {
//                             eprintln!("delay: {:?}", delay);
//                         }

//                         assert!(resampled.is_packed());

//                         const PACKED_DATA_INDEX: usize = 0;
//                         queue.extend(resampled.data(PACKED_DATA_INDEX));

//                         while queue.len() >= BUFFER_MAX_SIZE {
//                             let data = queue.drain(..BUFFER_MAX_SIZE).collect::<Vec<_>>();
//                             assert!(buffers.insert(next_frame_index, Arc::new(data)).is_none());
//                             next_frame_index += 1;
//                         }
//                     }

//                     println!("Audio Buffer Core Thread {} finished", id);

//                     assert!(buffers.insert(next_frame_index, Arc::new(queue)).is_none());

//                     Ok(())
//                 })();

//                 done.store(true, std::sync::atomic::Ordering::SeqCst);

//                 if let Err(err) = result {
//                     eprintln!("[namui-media] failed to fetch audio frames: {}", err);
//                 }
//             }
//         });

//         Ok(Self {
//             id,
//             buffers,
//             done,
//             output_config,
//         })
//     }
//     pub(crate) fn id(&self) -> usize {
//         self.id
//     }
//     pub fn is_loading_finished(&self) -> bool {
//         self.done.load(std::sync::atomic::Ordering::SeqCst)
//     }
//     pub fn get_best_effort_data(&self, byte_offset: usize, desired_length: usize) -> Vec<u8> {
//         let mut data = vec![];

//         let mut rest_length = desired_length;
//         let mut map_index = byte_offset / BUFFER_MAX_SIZE;
//         let mut buffer_offset = byte_offset % BUFFER_MAX_SIZE;

//         while rest_length > 0 {
//             let Some(entry) = self.buffers.get(&map_index) else {
//                 break;
//             };

//             let buffer = entry.value();

//             if buffer_offset >= buffer.len() {
//                 break;
//             }

//             let right = (buffer_offset + rest_length).min(buffer.len());
//             let slice = &buffer[buffer_offset..right];

//             data.extend_from_slice(slice);

//             rest_length -= slice.len();
//             map_index += 1;
//             buffer_offset = 0;
//         }

//         data
//     }
//     pub fn is_byte_offset_out_of_buffer(&self, byte_offset: usize) -> bool {
//         let map_index = byte_offset / BUFFER_MAX_SIZE;
//         self.buffers.len() <= map_index
//     }
// }

// fn generate_audio_buffer_core_id() -> usize {
//     static AUDIO_BUFFER_CORE_ID: std::sync::atomic::AtomicUsize =
//         std::sync::atomic::AtomicUsize::new(0);
//     AUDIO_BUFFER_CORE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
// }

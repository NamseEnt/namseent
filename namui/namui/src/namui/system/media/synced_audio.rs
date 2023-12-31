// use super::{audio_buffer_core::AudioBufferCore, AudioConfig};
// use anyhow::Result;
// use namui_type::*;
// use std::mem::size_of;

// #[derive(Debug)]
// /// SyncedAudio assumes device format not changed after SyncedAudio creation.
// /// SyncedAudio assumes that it start play right after creation.
// pub struct SyncedAudio {
//     audio_buffer_core: AudioBufferCore,
//     /// buffer_offset could be greater than buffer.len() when it skips some frames.
//     buffer_byte_offset: usize,
//     start_at: Instant,
//     #[allow(dead_code)]
//     start_buffer_byte_offset_for_debug: usize,
// }

// impl SyncedAudio {
//     pub(crate) fn new(
//         audio_buffer_core: AudioBufferCore,
//         start_at: Instant,
//         start_offset: Duration,
//     ) -> Self {
//         let buffer_byte_offset =
//             calculate_byte_offset(start_offset, audio_buffer_core.output_config);

//         Self {
//             audio_buffer_core,
//             buffer_byte_offset,
//             start_at,
//             start_buffer_byte_offset_for_debug: buffer_byte_offset,
//         }
//     }
//     pub(crate) fn audio_buffer_core_id(&self) -> usize {
//         self.audio_buffer_core.id()
//     }
//     pub fn is_finished(&self) -> bool {
//         self.audio_buffer_core.is_loading_finished()
//             && self
//                 .audio_buffer_core
//                 .is_byte_offset_out_of_buffer(self.buffer_byte_offset)
//     }
//     pub(crate) fn consume(
//         &mut self,
//         expected_output_sample_byte_len: usize,
//         playback_at: Instant,
//     ) -> Result<Vec<u8>> {
//         if self.is_finished() || playback_at < self.start_at {
//             return Ok(vec![]);
//         }

//         let data = self
//             .audio_buffer_core
//             .get_best_effort_data(self.buffer_byte_offset, expected_output_sample_byte_len);

//         // Keep increasing buffer_byte_offset to skip delayed frames for sync.
//         self.buffer_byte_offset += expected_output_sample_byte_len;

//         self.debug_latency(playback_at);

//         // For Debug Latency

//         Ok(data)
//     }

//     pub(crate) fn seek_to(&mut self, offset: Duration) {
//         self.buffer_byte_offset =
//             calculate_byte_offset(offset, self.audio_buffer_core.output_config);

//         self.start_at = crate::time::now();
//         self.start_buffer_byte_offset_for_debug = self.buffer_byte_offset;
//     }

//     fn debug_latency(&self, playback_at: Instant) {
//         if std::env::var("DEBUG_LATENCY").is_err() {
//             return;
//         }

//         let expected_buffer_byte_offset = calculate_byte_offset(
//             playback_at - self.start_at,
//             self.audio_buffer_core.output_config,
//         ) + self.start_buffer_byte_offset_for_debug;

//         if expected_buffer_byte_offset != self.buffer_byte_offset {
//             eprintln!(
//                 "expected: {} but {} -> {}, now: {:?}",
//                 expected_buffer_byte_offset,
//                 self.buffer_byte_offset,
//                 expected_buffer_byte_offset.abs_diff(self.buffer_byte_offset),
//                 crate::time::now(),
//             );
//         }
//     }
// }

// fn calculate_byte_offset(duration: Duration, output_config: AudioConfig) -> usize {
//     let buffer_byte_offset = (duration.as_secs_f64()
//         * output_config.sample_rate as f64
//         * output_config.channel_count as f64
//         * output_config.sample_byte_size as f64) as usize;

//     buffer_byte_offset - buffer_byte_offset % size_of::<f32>()
// }

use super::{audio_buffer_core::AudioBufferCore, AudioConfig};
use anyhow::Result;

#[derive(Debug)]
/// Assumed device format not changed after create SyncedAudio.
pub struct SyncedAudio {
    buffer_core: AudioBufferCore,
    /// buffer_offset could be greater than buffer.len() when it skips some frames.
    buffer_byte_offset: usize,
    start_instant: Option<std::time::Instant>,
    last_sync_instant: Option<std::time::Instant>,
    output_config: AudioConfig,
}

impl SyncedAudio {
    pub(crate) fn new(
        frame_rx: crossbeam_channel::Receiver<ffmpeg_next::frame::Audio>,
        input_config: AudioConfig,
        output_config: AudioConfig,
    ) -> Result<Self> {
        Ok(Self {
            buffer_core: AudioBufferCore::new(frame_rx, input_config, output_config)?,
            buffer_byte_offset: 0,
            start_instant: None,
            last_sync_instant: None,
            output_config,
        })
    }
    pub fn is_finished(&self) -> bool {
        self.buffer_core.is_loading_finished()
            && self
                .buffer_core
                .is_byte_offset_out_of_buffer(self.buffer_byte_offset)
    }
    pub fn start(&mut self) {
        self.start_instant = Some(std::time::Instant::now());
    }
    pub(crate) fn consume(&mut self, expected_output_sample_byte_len: usize) -> Result<Vec<u8>> {
        if self.start_instant.is_none() {
            return Ok(vec![]);
        }

        let data = self
            .buffer_core
            .get_best_effort_data(self.buffer_byte_offset, expected_output_sample_byte_len);

        // Keep increasing buffer_byte_offset to skip delayed frames for sync.
        self.buffer_byte_offset += expected_output_sample_byte_len;

        // self.try_sync()?;

        Ok(data)
    }

    // TODO
    #[allow(dead_code)]
    fn try_sync(&mut self) -> Result<()> {
        // NOTE: HMM...? something is wrong?

        let now = std::time::Instant::now();
        if let Some(last_sync_instant) = self.last_sync_instant {
            if now - last_sync_instant < std::time::Duration::from_secs(1) {
                return Ok(());
            }
        }

        self.last_sync_instant = Some(now);

        let expected_byte_offset = (now - self.start_instant.unwrap()).as_secs()
            * self.output_config.sample_rate as u64
            * self.output_config.sample_byte_size as u64;

        let byte_offset_diff = expected_byte_offset.abs_diff(self.buffer_byte_offset as u64);

        // 0.1 seconds
        let max_byte_offset_diff =
            self.output_config.sample_rate as u64 * self.output_config.sample_byte_size as u64 / 10;

        if byte_offset_diff > max_byte_offset_diff {
            eprintln!(
                "audio sync activated! {} -> {}",
                self.buffer_byte_offset, expected_byte_offset
            );
            self.buffer_byte_offset = expected_byte_offset as usize;
        }

        Ok(())
    }
}

impl Clone for SyncedAudio {
    fn clone(&self) -> Self {
        Self {
            buffer_core: self.buffer_core.clone(),
            buffer_byte_offset: 0,
            start_instant: None,
            last_sync_instant: None,
            output_config: self.output_config,
        }
    }
}

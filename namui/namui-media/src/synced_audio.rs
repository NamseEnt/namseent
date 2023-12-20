use anyhow::Result;

#[derive(Debug)]
/// Assumed device format not changed after create SyncedAudio.
pub struct SyncedAudio {
    resampled_frame_rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
    buffer: Vec<u8>,
    /// buffer_offset could be greater than buffer.len() when it skips some frames.
    buffer_byte_offset: usize,
    audio_frame_rx_eof: bool,
    start_instant: Option<std::time::Instant>,
    last_sync_instant: Option<std::time::Instant>,
    output_config: AudioConfig,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AudioConfig {
    pub(crate) sample_rate: u32,
    pub(crate) sample_format: ffmpeg_next::format::Sample,
    pub(crate) channel_layout: ffmpeg_next::channel_layout::ChannelLayout,
    pub(crate) sample_byte_size: usize,
    pub(crate) channel_count: usize,
}

impl SyncedAudio {
    pub(crate) fn new(
        frame_rx: tokio::sync::mpsc::Receiver<ffmpeg_next::frame::Audio>,
        input_config: AudioConfig,
        output_config: AudioConfig,
    ) -> Result<Self> {
        let (resampled_frame_tx, resampled_frame_rx) = tokio::sync::mpsc::channel(512);
        tokio::spawn({
            async move {
                run_fetch_audio_frames(frame_rx, input_config, output_config, resampled_frame_tx)
                    .await
            }
        });

        Ok(Self {
            resampled_frame_rx,
            buffer: Vec::new(),
            buffer_byte_offset: 0,
            start_instant: None,
            last_sync_instant: None,
            output_config,
            audio_frame_rx_eof: false,
        })
    }
    pub fn is_finished(&self) -> bool {
        self.audio_frame_rx_eof && self.buffer_byte_offset >= self.buffer.len()
    }
    pub(crate) fn consume<T: cpal::Sample>(
        &mut self,
        expected_output_sample_len: usize,
    ) -> Result<&[T]> {
        if self.start_instant.is_none() {
            return Ok(&[]);
        }

        self.fetch_audio_frames()?;

        let left = self.buffer_byte_offset.min(self.buffer.len());
        let right = (self.buffer_byte_offset
            + expected_output_sample_len * self.output_config.sample_byte_size)
            .min(self.buffer.len());

        let output = unsafe {
            std::slice::from_raw_parts(
                self.buffer.as_ptr().add(left) as *const T,
                (right - left) / self.output_config.sample_byte_size,
            )
        };

        // Keep increasing buffer_byte_offset to skip delayed frames for sync.
        self.buffer_byte_offset += expected_output_sample_len * self.output_config.sample_byte_size;

        self.try_sync()?;

        Ok(output)
    }

    fn fetch_audio_frames(&mut self) -> Result<()> {
        loop {
            match self.resampled_frame_rx.try_recv() {
                Ok(buffer) => {
                    self.buffer.extend(buffer);
                }
                Err(err) => match err {
                    tokio::sync::mpsc::error::TryRecvError::Empty => {
                        break;
                    }
                    tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                        self.audio_frame_rx_eof = true;
                    }
                },
            }
        }

        Ok(())
    }

    /// Make sure last_sync_instant and start_instant are set.
    fn try_sync(&mut self) -> Result<()> {
        let now = std::time::Instant::now();
        if now - self.last_sync_instant.unwrap() < std::time::Duration::from_secs(1) {
            return Ok(());
        }

        self.last_sync_instant = Some(now);

        let expected_byte_offset = (now - self.start_instant.unwrap()).as_secs()
            * self.output_config.sample_rate as u64
            * self.output_config.sample_byte_size as u64;

        let offset_diff = expected_byte_offset.abs_diff(self.buffer_byte_offset as u64);

        // 0.1 seconds
        let max_offset_diff =
            self.output_config.sample_rate as u64 * self.output_config.sample_byte_size as u64 / 10;

        if offset_diff > max_offset_diff {
            self.buffer_byte_offset = expected_byte_offset as usize;
        }

        Ok(())
    }
}

async fn run_fetch_audio_frames(
    mut frame_rx: tokio::sync::mpsc::Receiver<ffmpeg_next::frame::Audio>,
    input: AudioConfig,
    output_config: AudioConfig,
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
) -> Result<()> {
    let mut resampler = ffmpeg_next::software::resampling::Context::get(
        input.sample_format,
        input.channel_layout,
        input.sample_rate,
        output_config.sample_format,
        if output_config.channel_count == 1 {
            ffmpeg_next::ChannelLayout::MONO
        } else {
            ffmpeg_next::ChannelLayout::STEREO
        },
        output_config.sample_rate,
    )?;

    while let Some(frame) = frame_rx.recv().await {
        assert!(frame.is_packed());

        let mut resampled = ffmpeg_next::frame::Audio::empty();
        if let Some(delay) = resampler.run(&frame, &mut resampled)? {
            eprintln!("delay: {:?}", delay);
        }

        let data_index = 0; // because it's packed.
        tx.send(resampled.data(data_index).to_vec()).await?;
    }

    Ok(())
}

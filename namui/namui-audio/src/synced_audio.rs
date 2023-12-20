use crate::AudioMedia;
use anyhow::Result;

/// Assumed device format not changed after create SyncedAudio.
pub struct SyncedAudio {
    audio_media: AudioMedia,
    resampler: ffmpeg_next::software::resampling::Context,
    buffer: Vec<u8>,
    /// buffer_offset could be greater than buffer.len() when it skips some frames.
    buffer_byte_offset: usize,
    audio_frame_rx_eof: bool,
    sample_byte_size: usize,
    start_instant: std::time::Instant,
    last_sync_instant: std::time::Instant,
}

impl SyncedAudio {
    pub(crate) fn new(
        audio_media: AudioMedia,
        output_sample_format: cpal::SampleFormat,
        output_channel_count: usize,
        output_sample_rate: u32,
        start_instant: std::time::Instant,
    ) -> Result<Self> {
        let packed = ffmpeg_next::format::sample::Type::Packed;

        let resampler = ffmpeg_next::software::resampling::Context::get(
            audio_media.sample_format,
            audio_media.channel_layout,
            audio_media.sample_rate,
            match output_sample_format {
                cpal::SampleFormat::I16 => ffmpeg_next::format::Sample::I16(packed),
                cpal::SampleFormat::I32 => ffmpeg_next::format::Sample::I32(packed),
                cpal::SampleFormat::I64 => ffmpeg_next::format::Sample::I64(packed),
                cpal::SampleFormat::U8 => ffmpeg_next::format::Sample::U8(packed),
                cpal::SampleFormat::F32 => ffmpeg_next::format::Sample::F32(packed),
                cpal::SampleFormat::F64 => ffmpeg_next::format::Sample::F64(packed),
                cpal::SampleFormat::I8 => unimplemented!(), // It's not supported by ffmpeg ffmpeg_next::format::Sample::I8(packed),
                cpal::SampleFormat::U16 => unimplemented!(), // It's not supported by ffmpeg ffmpeg_next::format::Sample::U16(packed),
                cpal::SampleFormat::U32 => unimplemented!(), // It's not supported by ffmpeg ffmpeg_next::format::Sample::U32(packed),
                cpal::SampleFormat::U64 => unimplemented!(), // It's not supported by ffmpeg ffmpeg_next::format::Sample::U64(packed),
                _ => unreachable!(),
            },
            if output_channel_count == 1 {
                ffmpeg_next::ChannelLayout::MONO
            } else {
                ffmpeg_next::ChannelLayout::STEREO
            },
            output_sample_rate,
        )?;

        Ok(Self {
            audio_media,
            resampler,
            buffer: Vec::new(),
            buffer_byte_offset: 0,
            audio_frame_rx_eof: false,
            sample_byte_size: output_sample_format.sample_size(),
            start_instant,
            last_sync_instant: start_instant,
        })
    }
    pub(crate) fn consume<T: cpal::Sample>(
        &mut self,
        expected_output_sample_len: usize,
    ) -> Result<&[T]> {
        self.fetch_audio_frames()?;

        let left = self.buffer_byte_offset.min(self.buffer.len());
        let right = (self.buffer_byte_offset + expected_output_sample_len * self.sample_byte_size)
            .min(self.buffer.len());

        let output = unsafe {
            std::slice::from_raw_parts(
                self.buffer.as_ptr().add(left) as *const T,
                (right - left) / self.sample_byte_size,
            )
        };

        // Keep increasing buffer_byte_offset to skip delayed frames for sync.
        self.buffer_byte_offset += expected_output_sample_len * self.sample_byte_size;

        self.try_sync()?;

        Ok(output)

        /*
           2. 시작 instant와 buffer_offset을 계산한 시간의 갭이 크다면 조정을 한다.
        */
    }

    fn fetch_audio_frames(&mut self) -> Result<()> {
        while !self.audio_frame_rx_eof {
            match self.audio_media.frame_rx.try_recv() {
                Ok(frame) => {
                    let mut resampled = ffmpeg_next::frame::Audio::empty();
                    if let Some(delay) = self.resampler.run(&frame, &mut resampled)? {
                        eprintln!("delay: {:?}", delay);
                    }

                    let data_index = 0; // because it's packed.
                    self.buffer.extend_from_slice(resampled.data(data_index))
                }
                Err(err) => match err {
                    std::sync::mpsc::TryRecvError::Empty => break,
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        self.audio_frame_rx_eof = true;
                    }
                },
            }
        }

        Ok(())
    }

    fn try_sync(&mut self) -> Result<()> {
        let now = std::time::Instant::now();
        if now - self.last_sync_instant < std::time::Duration::from_secs(1) {
            return Ok(());
        }

        self.last_sync_instant = now;

        let expected_byte_offset = (now - self.start_instant).as_secs()
            * self.audio_media.sample_rate as u64
            * self.sample_byte_size as u64;

        let offset_diff = expected_byte_offset.abs_diff(self.buffer_byte_offset as u64);

        // 0.1 seconds
        let max_offset_diff =
            self.audio_media.sample_rate as u64 * self.sample_byte_size as u64 / 10;

        if offset_diff > max_offset_diff {
            self.buffer_byte_offset = expected_byte_offset as usize;
        }

        Ok(())
    }
}

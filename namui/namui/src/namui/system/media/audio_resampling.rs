use super::{
    audio_buffer::AudioBuffer, flush_button::FlushButtonReceiver, AudioConfig, AUDIO_CHANNEL_BOUND,
};
use anyhow::Result;

pub(crate) fn start_audio_resampling(
    mut ffmpeg_audio_frame_rx: rtrb::Consumer<ffmpeg_next::frame::Audio>,
    input_config: AudioConfig,
    output_config: AudioConfig,
    flush_requested: FlushButtonReceiver,
) -> AudioBuffer {
    let (mut producer, consumer) = rtrb::RingBuffer::new(AUDIO_CHANNEL_BOUND);

    std::thread::spawn({
        move || {
            let result = (move || -> Result<()> {
                let mut resampler = ffmpeg_next::software::resampling::Context::get(
                    input_config.sample_format,
                    input_config.channel_layout,
                    input_config.sample_rate,
                    output_config.sample_format,
                    if output_config.channel_count == 1 {
                        ffmpeg_next::ChannelLayout::MONO
                    } else {
                        ffmpeg_next::ChannelLayout::STEREO
                    },
                    output_config.sample_rate,
                )?;

                loop {
                    let Ok(frame) = ffmpeg_audio_frame_rx.pop() else {
                        if ffmpeg_audio_frame_rx.is_abandoned() {
                            break;
                        }
                        std::thread::yield_now();
                        continue;
                    };

                    let mut resampled = ffmpeg_next::frame::Audio::empty();
                    if let Some(delay) = resampler.run(&frame, &mut resampled)? {
                        eprintln!("delay: {:?}", delay);
                    }

                    assert!(resampled.is_packed());

                    const PACKED_DATA_INDEX: usize = 0;
                    let slice = resampled.data(PACKED_DATA_INDEX);
                    let f32_slice = unsafe {
                        std::slice::from_raw_parts(
                            slice.as_ptr() as *const f32,
                            slice.len() / std::mem::size_of::<f32>(),
                        )
                    };
                    producer.push(f32_slice.to_vec())?;
                }

                Ok(())
            })();

            if let Err(err) = result {
                eprintln!("[namui-media] failed to fetch audio frames: {}", err);
            }
        }
    });

    AudioBuffer::new(consumer, flush_requested)
}

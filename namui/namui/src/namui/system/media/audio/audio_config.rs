#[derive(Debug, Clone, Copy)]
pub(crate) struct AudioConfig {
    pub sample_rate: u32,
    pub sample_format: ffmpeg_next::format::Sample,
    pub channel_layout: ffmpeg_next::channel_layout::ChannelLayout,
    pub channel_count: usize,
}

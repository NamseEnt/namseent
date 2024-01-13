#[derive(Debug, Clone, Copy)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub sample_format: ffmpeg_next::format::Sample,
    pub channel_layout: ffmpeg_next::channel_layout::ChannelLayout,
    pub channel_count: usize,
}

pub type Sample = ffmpeg_next::format::Sample;
pub type ChannelLayout = ffmpeg_next::channel_layout::ChannelLayout;

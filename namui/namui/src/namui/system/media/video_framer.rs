#[derive(Debug)]
pub(crate) struct VideoFramer {}

impl VideoFramer {
    pub(crate) fn new(rx: rtrb::Consumer<ffmpeg_next::frame::Video>) -> VideoFramer {
        todo!()
    }

    pub(crate) fn get_image(&self) -> Result<Option<namui_type::ImageHandle>, anyhow::Error> {
        todo!()
    }
}

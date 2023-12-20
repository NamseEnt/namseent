use anyhow::Result;
use namui_type::*;

#[derive(Debug)]
/// Video only contains images. If you want to play audio, use `SyncedVideo`.
pub struct ImageOnlyVideo {
    frame_rx: tokio::sync::mpsc::Receiver<ffmpeg_next::frame::Video>,
    start_instant: Option<std::time::Instant>,
    fps: f64,
    last_frame: Option<namui_type::Image>,
    frame_index: usize,
    eof: bool,
}

const FFMPEG_DEST_FORMAT: ffmpeg_next::util::format::Pixel =
    ffmpeg_next::util::format::Pixel::RGB32;
const COLOR_TYPE: namui_type::ColorType = namui_type::ColorType::RGB888x;

impl ImageOnlyVideo {
    pub(crate) fn new(
        frame_rx: tokio::sync::mpsc::Receiver<ffmpeg_next::frame::Video>,
        fps: f64,
        wh: Wh<u32>,
        pixel_type: ffmpeg_next::util::format::Pixel,
    ) -> Result<Self> {
        let scaler = ffmpeg_next::software::scaling::Context::get(
            pixel_type,
            wh.width,
            wh.height,
            FFMPEG_DEST_FORMAT,
            wh.width,
            wh.height,
            ffmpeg_next::software::scaling::Flags::BILINEAR,
        )?;

        Ok(Self {
            frame_rx,
            start_instant: None,
            fps,
            last_frame: None,
            frame_index: 0,
            eof: false,
            scaler,
        })
    }
    pub fn start(&mut self) {
        self.start_instant = Some(std::time::Instant::now());
    }
    pub fn get_image(&mut self) -> Result<Option<namui_type::Image>> {
        let Some(start_instant) = self.start_instant else {
            return Ok(self.last_frame.clone());
        };
        let now = std::time::Instant::now();
        let expected_frame_index = ((now - start_instant).as_secs_f64() * self.fps) as usize;
        assert!(expected_frame_index >= self.frame_index);

        if self.frame_index == expected_frame_index {
            if let Some(last_frame) = &self.last_frame {
                return Ok(Some(last_frame.clone()));
            }
        }

        let frame_count_to_gather =
            expected_frame_index - self.frame_index + if self.last_frame.is_none() { 1 } else { 0 };

        let mut new_frame = None;
        for _ in 0..frame_count_to_gather {
            match self.frame_rx.try_recv() {
                Ok(frame) => {
                    new_frame = Some(frame);
                    if !(self.frame_index == 0 && self.last_frame.is_none()) {
                        self.frame_index += 1;
                    }
                }
                Err(err) => match err {
                    tokio::sync::mpsc::error::TryRecvError::Empty => {
                        break;
                    }
                    tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                        self.eof = true;
                        break;
                    }
                },
            }
        }

        if let Some(new_frame) = new_frame {
            let mut scaled = ffmpeg_next::frame::Video::empty();
            self.scaler.run(&new_frame, &mut scaled)?;

            self.last_frame = Some(namui_type::Image {
                wh: Wh {
                    width: (scaled.width() as f32).px(),
                    height: (scaled.height() as f32).px(),
                },
                src: namui_type::ImageSource::Bytes {
                    bytes: scaled.data(0).to_vec().into(),
                    color_type: COLOR_TYPE,
                },
            });
        }

        Ok(self.last_frame.clone())
    }
}

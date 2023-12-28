use super::VIDEO_CHANNEL_BOUND;
use anyhow::{anyhow, Result};
use namui_type::*;

#[derive(Debug)]
/// Video only contains images. If you want to play audio, use `SyncedVideo`.
pub struct ImageOnlyVideo {
    frame_rx: crossbeam_channel::Receiver<ImageHandle>,
    start_instant: Option<std::time::Instant>,
    fps: f64,
    last_frame: Option<ImageHandle>,
    frame_index: usize,
    eof: bool,
}

const FFMPEG_DEST_FORMAT: ffmpeg_next::util::format::Pixel = ffmpeg_next::util::format::Pixel::RGBA;
const COLOR_TYPE: namui_type::ColorType = namui_type::ColorType::Rgba8888;

impl ImageOnlyVideo {
    pub(crate) fn new(
        frame_rx: crossbeam_channel::Receiver<ffmpeg_next::frame::Video>,
        fps: f64,
        wh: Wh<u32>,
        pixel_type: ffmpeg_next::util::format::Pixel,
    ) -> Result<Self> {
        let (image_tx, image_rx) = crossbeam_channel::bounded(VIDEO_CHANNEL_BOUND);
        std::thread::spawn(move || {
            let result = move || -> Result<()> {
                let mut scaler = ffmpeg_next::software::scaling::Context::get(
                    pixel_type,
                    wh.width,
                    wh.height,
                    FFMPEG_DEST_FORMAT,
                    wh.width,
                    wh.height,
                    ffmpeg_next::software::scaling::Flags::BILINEAR,
                )
                .map_err(|err| anyhow!("ffmpeg scaling context get error: {:?}", err))?;

                while let Ok(frame) = frame_rx.recv() {
                    let mut output = ffmpeg_next::frame::Video::empty();
                    scaler
                        .run(&frame, &mut output)
                        .map_err(|err| anyhow!("ffmpeg scaling run error: {:?}", err))?;

                    let image_handle = crate::system::skia::load_image2(
                        ImageInfo {
                            alpha_type: AlphaType::Opaque,
                            color_type: COLOR_TYPE,
                            height: (wh.height as f32).px(),
                            width: (wh.width as f32).px(),
                        },
                        output.data_mut(0),
                    );

                    image_tx.send(image_handle)?;
                }

                Ok(())
            }();

            if let Err(err) = result {
                eprintln!("Error: {:?}", err);
            }
        });

        Ok(Self {
            frame_rx: image_rx,
            start_instant: None,
            fps,
            last_frame: None,
            frame_index: 0,
            eof: false,
        })
    }
    pub fn start(&mut self) {
        self.start_instant = Some(std::time::Instant::now());
    }
    pub(crate) fn stop(&mut self) {
        self.start_instant = None;
    }
    pub fn get_image(&mut self) -> Result<Option<ImageHandle>> {
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
                    crossbeam_channel::TryRecvError::Empty => {
                        break;
                    }
                    crossbeam_channel::TryRecvError::Disconnected => {
                        self.eof = true;
                        break;
                    }
                },
            }
        }

        if let Some(new_frame) = new_frame {
            self.last_frame = Some(new_frame);
        }

        Ok(self.last_frame.clone())
    }

    pub(crate) fn is_playing(&self) -> bool {
        self.start_instant.is_some() && !self.eof
    }
}

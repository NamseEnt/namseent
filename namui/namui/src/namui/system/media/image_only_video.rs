use super::{open_media::open_video, VIDEO_CHANNEL_BOUND};
use anyhow::{anyhow, bail, Result};
use crossbeam_channel::Receiver;
use namui_type::*;
use std::path::PathBuf;

#[derive(Debug)]
/// Video only contains images. If you want to play audio, use `SyncedVideo`.
pub struct ImageOnlyVideo {
    video_source: PathBuf,
    image_rx: crossbeam_channel::Receiver<ImageHandle>,
    start_instant: Option<Instant>,
    start_offset: Option<Duration>,
    fps: f64,
    last_frame: Option<(ImageHandle, usize)>,
    skip_frame_count: usize,
    eof: bool,
    wh: Wh<u32>,
    pixel_type: ffmpeg_next::util::format::Pixel,
}

const FFMPEG_DEST_FORMAT: ffmpeg_next::util::format::Pixel = ffmpeg_next::util::format::Pixel::RGBA;
const COLOR_TYPE: namui_type::ColorType = namui_type::ColorType::Rgba8888;

pub(crate) struct VideoMaterials {
    pub(crate) frame_rx: Receiver<ffmpeg_next::frame::Video>,
    pub(crate) fps: f64,
    pub(crate) wh: Wh<u32>,
    pub(crate) pixel_type: ffmpeg_next::util::format::Pixel,
}

impl ImageOnlyVideo {
    pub(crate) fn new(
        video_source: &impl AsRef<std::path::Path>,
        VideoMaterials {
            frame_rx,
            fps,
            wh,
            pixel_type,
        }: VideoMaterials,
    ) -> Self {
        let image_rx = spawn_video_decoding_thread(frame_rx, wh, pixel_type);

        Self {
            video_source: video_source.as_ref().to_path_buf(),
            image_rx,
            start_instant: None,
            start_offset: None,
            fps,
            last_frame: None,
            skip_frame_count: 0,
            eof: false,
            wh,
            pixel_type,
        }
    }
    pub(crate) fn is_playing(&self) -> bool {
        self.start_instant.is_some() && !self.eof
    }
    pub(crate) fn start(&mut self, start_at: Instant, start_offset: Duration) {
        self.start_instant = Some(start_at);
        self.start_offset = Some(start_offset);
        self.skip_frame_count = (start_offset.as_secs_f64() * self.fps) as usize;
    }
    pub(crate) fn stop(&mut self) {
        todo!()
    }
    pub(crate) fn pause(&mut self) {
        self.start_instant = None;
    }
    pub fn get_image(&mut self) -> Result<Option<ImageHandle>> {
        let Some(start_instant) = self.start_instant else {
            return Ok(self.last_frame.clone().map(|x| x.0));
        };

        let now = crate::time::now();
        let playback_duration = (now - start_instant) + self.start_offset.unwrap_or_default();

        let expected_frame_index = ((playback_duration).as_secs_f64() * self.fps) as usize;

        if let Some((ref last_frame_image, last_frame_index)) = self.last_frame {
            if last_frame_index >= expected_frame_index {
                return Ok(Some(last_frame_image.clone()));
            }
        }

        loop {
            match self.image_rx.try_recv() {
                Ok(frame) => {
                    if self.skip_frame_count > 0 {
                        self.skip_frame_count -= 1;
                        continue;
                    }

                    let frame_index = self
                        .last_frame
                        .as_ref()
                        .map(|x| x.1 + 1)
                        .unwrap_or_default();

                    self.last_frame = Some((frame, frame_index));

                    if frame_index >= expected_frame_index {
                        break;
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

        println!(
            "expected_frame_index: {}, frame_index: {}",
            expected_frame_index,
            self.last_frame.as_ref().map(|x| x.1).unwrap_or_default()
        );

        Ok(self.last_frame.clone().map(|x| x.0))
    }

    pub(crate) fn seek_to(&mut self, playback_duration: Duration) -> Result<()> {
        let expected_frame_index = (playback_duration.as_secs_f64() * self.fps) as usize;
        let frame_index = self.last_frame.as_ref().map(|x| x.1).unwrap_or_default();

        if expected_frame_index < frame_index {
            self.last_frame = None;

            let Some(video_material) = open_video(&self.video_source)? else {
                bail!("failed to open video");
            };
            self.image_rx =
                spawn_video_decoding_thread(video_material.frame_rx, self.wh, self.pixel_type);
        } else {
            // self.skip_frame_count = expected_frame_index - frame_index;
        }

        self.start_offset = Some({
            if let Some(start_instant) = self.start_instant {
                playback_duration - (crate::time::now() - start_instant)
            } else {
                playback_duration
            }
        });

        Ok(())
    }
}

fn spawn_video_decoding_thread(
    frame_rx: crossbeam_channel::Receiver<ffmpeg_next::frame::Video>,
    wh: Wh<u32>,
    pixel_type: ffmpeg_next::util::format::Pixel,
) -> Receiver<ImageHandle> {
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

    image_rx
}

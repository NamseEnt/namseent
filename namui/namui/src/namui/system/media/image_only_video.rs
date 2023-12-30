use super::{open_media::open_media, VIDEO_CHANNEL_BOUND};
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
    last_playback_duration: Option<Duration>,
    fps: f64,
    last_frame: Option<ImageHandle>,
    frame_index: usize,
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
            last_playback_duration: None,
            fps,
            last_frame: None,
            frame_index: 0,
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
        self.last_playback_duration = Some(start_offset);
    }
    pub(crate) fn stop(&mut self) {
        todo!()
    }
    pub(crate) fn pause(&mut self) {
        let Some(start_instant) = self.start_instant.take() else {
            return;
        };
        self.last_playback_duration = Some(
            (crate::time::now() - start_instant) + self.last_playback_duration.unwrap_or_default(),
        );
    }
    pub fn get_image(&mut self) -> Result<Option<ImageHandle>> {
        let Some(start_instant) = self.start_instant else {
            return Ok(self.last_frame.clone());
        };

        let now = crate::time::now();
        let playback_duration =
            (now - start_instant) + self.last_playback_duration.unwrap_or_default();

        let expected_frame_index = ((playback_duration).as_secs_f64() * self.fps) as usize;
        if expected_frame_index < self.frame_index {
            return Ok(self.last_frame.clone());
        }

        if self.frame_index == expected_frame_index {
            if let Some(last_frame) = &self.last_frame {
                return Ok(Some(last_frame.clone()));
            }
        }

        let frame_count_to_gather =
            expected_frame_index - self.frame_index + if self.last_frame.is_none() { 1 } else { 0 };

        let mut new_frame = None;
        for _ in 0..frame_count_to_gather {
            match self.image_rx.try_recv() {
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

    pub(crate) fn seek_to(&mut self, playback_duration: Duration) -> Result<()> {
        let expected_frame_index = (playback_duration.as_secs_f64() * self.fps) as usize;
        if expected_frame_index < self.frame_index {
            let (video_material, _) = open_media(
                &self.video_source,
                super::open_media::OpenMediaFilter::YesVideoNoAudio,
            )?;

            let Some(video_material) = video_material else {
                bail!("failed to open video");
            };

            self.image_rx =
                spawn_video_decoding_thread(video_material.frame_rx, self.wh, self.pixel_type);
        }

        self.last_playback_duration = Some(playback_duration);

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

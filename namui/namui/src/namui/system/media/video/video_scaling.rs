use super::video_framer::VideoFramer;
use crate::system::media::{
    core::MediaControlReceiver, with_instant::WithInstant, VIDEO_CHANNEL_BOUND,
};
use crate::*;
use anyhow::{anyhow, Result};

const FFMPEG_DEST_FORMAT: ffmpeg_next::util::format::Pixel = ffmpeg_next::util::format::Pixel::RGBA;
const COLOR_TYPE: ColorType = ColorType::Rgba8888;

pub(crate) fn start_video_scaling(
    ffmpeg_video_frames_rx: std::sync::mpsc::Receiver<WithInstant<ffmpeg_next::frame::Video>>,
    control_receiver: MediaControlReceiver,
    wh: Wh<u32>,
    pixel_type: ffmpeg_next::util::format::Pixel,
    fps: f64,
) -> VideoFramer {
    let (image_handle_tx, image_handle_rx) = std::sync::mpsc::sync_channel(VIDEO_CHANNEL_BOUND);

    std::thread::spawn({
        let control_receiver = control_receiver.clone();
        move || {
            let result = move || -> Result<()> {
                let runtime = tokio::runtime::Builder::new_current_thread().build()?;

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

                while let Ok(frame) = ffmpeg_video_frames_rx.recv() {
                    if control_receiver.should_skip_this(frame.instant) {
                        continue;
                    }

                    let mut output = ffmpeg_next::frame::Video::empty();
                    scaler
                        .run(&frame, &mut output)
                        .map_err(|err| anyhow!("ffmpeg scaling run error: {:?}", err))?;

                    let image_handle_future = crate::system::skia::load_image_from_raw(
                        ImageInfo {
                            alpha_type: AlphaType::Opaque,
                            color_type: COLOR_TYPE,
                            height: (wh.height as f32).px(),
                            width: (wh.width as f32).px(),
                        },
                        output.data_mut(0),
                    );

                    let image_handle = runtime.block_on(image_handle_future)?;

                    image_handle_tx.send(WithInstant::new(image_handle, frame.instant))?;
                }

                Ok(())
            }();

            if let Err(err) = result {
                eprintln!(
                    "[namui-media] Err on spawn_video_decoding_thread (would not real error): {:?}",
                    err
                );
            }
        }
    });

    VideoFramer::new(image_handle_rx, control_receiver, fps)
}

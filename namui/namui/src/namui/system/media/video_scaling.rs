use anyhow::{anyhow, Result};
use namui_type::*;
use std::sync::{Arc, Mutex};

const FFMPEG_DEST_FORMAT: ffmpeg_next::util::format::Pixel = ffmpeg_next::util::format::Pixel::RGBA;
const COLOR_TYPE: namui_type::ColorType = namui_type::ColorType::Rgba8888;

pub(crate) fn start_video_scaling(
    mut ffmpeg_video_rx: rtrb::Consumer<ffmpeg_next::frame::Video>,
    wh: Wh<u32>,
    pixel_type: ffmpeg_next::util::format::Pixel,
) -> Arc<Mutex<Option<ImageHandle>>> {
    let output_image_handle = Arc::new(Mutex::new(None));

    std::thread::spawn({
        let output_image_handle = output_image_handle.clone();
        move || {
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

                loop {
                    let Ok(ffmpeg_video) = ffmpeg_video_rx.pop() else {
                        if ffmpeg_video_rx.is_abandoned() {
                            break;
                        } else {
                            std::thread::yield_now();
                            continue;
                        }
                    };

                    let mut output = ffmpeg_next::frame::Video::empty();
                    scaler
                        .run(&ffmpeg_video, &mut output)
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

                    output_image_handle.lock().unwrap().replace(image_handle);
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

    output_image_handle
}

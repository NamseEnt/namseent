use super::{
    media_control::MediaControlReceiver, video_framer::VideoFramer, with_instant::WithInstant,
    VIDEO_CHANNEL_BOUND,
};
use anyhow::{anyhow, Result};
use namui_type::*;

const FFMPEG_DEST_FORMAT: ffmpeg_next::util::format::Pixel = ffmpeg_next::util::format::Pixel::RGBA;
const COLOR_TYPE: namui_type::ColorType = namui_type::ColorType::Rgba8888;

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

                    let image_handle = crate::system::skia::load_image2(
                        ImageInfo {
                            alpha_type: AlphaType::Opaque,
                            color_type: COLOR_TYPE,
                            height: (wh.height as f32).px(),
                            width: (wh.width as f32).px(),
                        },
                        output.data_mut(0),
                    );

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

/*
다음 요청들이 들어오면 어떻게 해야하는지 생각해보자

- 재생
    - 들어오는대로 보여주면 됨. fps가 지남에 따라 프레임을 보여주면 됨.
- 일시정지
    - 가만히 있으면 됨. 기존에 보여주고 있던게 있으면 계속 보여주면 됨.
- seek
    - 아직 사용 안한 프레임들은 다 날리고, 새로 온 것들만 사용하면 됨.
- 정지
    - 모든걸 다 날리고 대기하면 됨.

# fps 처리하는 방법은?
내가 몇번째 프레임을 재생하고 있는지 일단 알아야한다.
시작한지 얼마나 되었는지를 보고, 지금 몇번째 프레임이어야하는지 계산할 수 있다.
거기에 격차가 생기면
  - 마이너스라면 뭔가 이상한거고
  - 플러스라면 그만큼 새 프레임을 받아오면 된다.

*/

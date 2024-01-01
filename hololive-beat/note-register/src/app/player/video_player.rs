use super::STATE;
use namui::{prelude::*, time::now};

#[component]
pub struct VideoPlayer<'a> {
    pub wh: Wh<Px>,
    pub video: &'a MediaHandle,
}

impl Component for VideoPlayer<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, video } = self;

        let (state, _) = ctx.atom(&STATE);

        ctx.effect("handle video", || match *state {
            super::State::Stop => {
                video.stop();
            }
            super::State::Play { started_time } => {
                video.play(started_time);
            }
            super::State::Pause { played_time } => {
                video.play(now() - played_time);
            }
        });

        ctx.compose(|ctx| {
            let Some(image_handle) = video.get_image().unwrap() else {
                return;
            };

            let rect = if wh.width / wh.height > 16.0 / 9.0 {
                let width = wh.height * (16.0 / 9.0);
                Rect::Xywh {
                    x: (wh.width - width) / 2,
                    y: 0.px(),
                    width,
                    height: wh.height,
                }
            } else {
                let height = wh.width * (9.0 / 16.0);
                Rect::Xywh {
                    x: 0.px(),
                    y: (wh.height - height) / 2,
                    width: wh.width,
                    height,
                }
            };

            ctx.add(namui::image(ImageParam {
                rect,
                source: ImageSource::ImageHandle { image_handle },
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint: None,
                },
            }));
        });

        ctx.done()
    }
}

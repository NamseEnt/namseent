use namui::prelude::*;
use namui_prebuilt::button::TextButton;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, || MediaExample).await
}

#[namui::component]
struct MediaExample;

impl Component for MediaExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (audio_mp3, set_audio_mp3) = ctx.state(|| None);
        let (video_mp4, set_video_mp4) = ctx.state(|| None);

        ctx.effect("load media", || {
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path("bundle:resources/audio.mp3")
                    .unwrap();

                let mp3 = namui::system::media::new_media(&path).unwrap();
                println!("mp3 loaded");

                set_audio_mp3.set(Some(mp3));

                let path =
                    namui::system::file::bundle::to_real_path("bundle:resources/you-re-mine.mp4")
                        .unwrap();

                let mp4 = namui::system::media::new_media(&path).unwrap();
                println!("mp4 loaded");

                set_video_mp4.set(Some(mp4));
            });
        });

        ctx.component(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 20.px(),
                width: 100.px(),
                height: 20.px(),
            },
            text: "play audio",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(media) = audio_mp3.as_ref() {
                    namui::system::media::play(media)
                }
            },
        });

        ctx.component(TextButton {
            rect: Rect::Xywh {
                x: 200.px(),
                y: 20.px(),
                width: 100.px(),
                height: 20.px(),
            },
            text: "play video",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(media) = video_mp4.as_ref() {
                    namui::system::media::play(media)
                }
            },
        });

        ctx.compose(|ctx| {
            let Some(mp4) = video_mp4.as_ref() else {
                return;
            };
            let mut mp4 = mp4.lock().unwrap();
            let Some(image_handle) = mp4.get_image().unwrap() else {
                return;
            };

            ctx.add(namui::image(ImageParam {
                rect: Rect::Xywh {
                    x: 200.px(),
                    y: 100.px(),
                    width: 100.px(),
                    height: 100.px(),
                },
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

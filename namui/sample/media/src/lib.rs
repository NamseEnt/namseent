use namui::prelude::*;
use namui_prebuilt::button::TextButton;

pub fn main() {
    namui::start(|| MediaExample)
}

#[namui::component]
struct MediaExample;

impl Component for MediaExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (audio_mp3, set_audio_mp3) = ctx.state(|| None);
        let (video_mp4, set_video_mp4) = ctx.state(|| None);
        let (media_handle_for_toggle, set_media_handle_for_toggle) = ctx.state(|| None);

        ctx.effect("load media", || {
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path("bundle:resources/audio.mp3")
                    .unwrap();

                let mp3 = namui::system::media::new_media(&path).unwrap();
                println!("mp3 loaded");

                set_audio_mp3.set(Some(mp3.clone_independent().unwrap()));
                set_media_handle_for_toggle.set(Some(mp3));

                let path = namui::system::file::bundle::to_real_path("bundle:resources/video.mp4")
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
                width: 200.px(),
                height: 20.px(),
            },
            text: "play audio (Fire & Forget)",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(media) = audio_mp3.as_ref() {
                    media.clone_independent().unwrap().play(namui::time::now());
                }
            },
        });

        ctx.component(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 60.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: &format!(
                "[Toggle] {}",
                if let Some(media_handle_for_toggle) = media_handle_for_toggle.as_ref() {
                    if media_handle_for_toggle.is_playing() {
                        "stop audio"
                    } else {
                        "play audio"
                    }
                } else {
                    "play audio"
                }
            ),
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                let Some(media_handle_for_toggle) = media_handle_for_toggle.as_ref() else {
                    return;
                };

                if media_handle_for_toggle.is_playing() {
                    media_handle_for_toggle.stop();
                } else {
                    media_handle_for_toggle.play(namui::time::now());
                }
            },
        });

        ctx.component(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 120.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: "Pause",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                let Some(media_handle_for_toggle) = media_handle_for_toggle.as_ref() else {
                    return;
                };
                media_handle_for_toggle.pause();
            },
        });

        ctx.component(TextButton {
            rect: Rect::Xywh {
                x: 300.px(),
                y: 20.px(),
                width: 100.px(),
                height: 20.px(),
            },
            text: if let Some(video_mp4) = video_mp4.as_ref() {
                if video_mp4.is_playing() {
                    "Pause Video"
                } else {
                    "Play Video"
                }
            } else {
                "Loading..."
            },
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(video_mp4) = video_mp4.as_ref() {
                    if video_mp4.is_playing() {
                        video_mp4.pause();
                    } else {
                        video_mp4.play(namui::time::now());
                    }
                }
            },
        });

        ctx.compose(|ctx| {
            let Some(mp4) = video_mp4.as_ref() else {
                return;
            };
            let Some(image_handle) = mp4.get_image().unwrap() else {
                return;
            };

            ctx.add(namui::image(ImageParam {
                rect: Rect::Xywh {
                    x: 300.px(),
                    y: 100.px(),
                    width: 400.px(),
                    height: 400.px(),
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

use namui::*;
use namui_prebuilt::button::TextButton;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(MediaExample {});
    })
}

#[namui::component]
struct MediaExample;

impl Component for MediaExample {
    fn render(self, ctx: &RenderCtx) {
        let (audio_mp3, set_audio_mp3) = ctx.state::<Option<MediaHandle>>(|| None);
        let (audio_opus, set_audio_opus) = ctx.state::<Option<MediaHandle>>(|| None);
        let (video_mp4, set_video_mp4) = ctx.state::<Option<MediaHandle>>(|| None);
        let (media_handle_for_toggle, set_media_handle_for_toggle) =
            ctx.state::<Option<MediaHandle>>(|| None);
        let (sliced_audio, set_sliced_audio) = ctx.state::<Option<FullLoadOnceAudio>>(|| None);

        ctx.effect("load media", || {
            let mp3 = namui::system::media::new_media(
                &namui::system::file::bundle::to_real_path("bundle:resources/audio.mp3").unwrap(),
            )
            .unwrap();
            println!("mp3 loaded");
            set_audio_mp3.set(Some(mp3.clone_independent().unwrap()));
            set_media_handle_for_toggle.set(Some(mp3));

            let opus = namui::system::media::new_media(
                &namui::system::file::bundle::to_real_path("bundle:resources/audio.opus").unwrap(),
            )
            .unwrap();
            println!("opus loaded");
            set_audio_opus.set(Some(opus));

            namui::spawn({
                let set_sliced_audio = set_sliced_audio.cloned();
                async move {
                    let opus = namui::system::media::new_full_load_once_audio(
                        &namui::system::file::bundle::to_real_path("bundle:resources/audio.opus")
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                    println!("full load once audio loaded");
                    set_sliced_audio.set(Some(
                        opus.slice(Duration::from_secs(1)..Duration::from_secs(2))
                            .unwrap(),
                    ));
                }
            });

            let mp4 = namui::system::media::new_media(
                &namui::system::file::bundle::to_real_path("bundle:resources/video.mp4").unwrap(),
            )
            .unwrap();
            println!("mp4 loaded");

            set_video_mp4.set(Some(mp4));
        });

        let seek_to = |media_handle: MediaHandle, to: Duration| {
            namui::spawn(async move {
                let was_playing = media_handle.is_playing();

                if was_playing {
                    media_handle.pause().unwrap();
                }

                media_handle.seek_to(to).unwrap();

                media_handle.wait_for_preload().await.unwrap();

                if was_playing {
                    media_handle.play().unwrap();
                }
            })
        };

        ctx.add(TextButton {
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
                    media.clone_independent().unwrap().play().unwrap();
                }
            },
        });

        ctx.add(TextButton {
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
                    media_handle_for_toggle.stop().unwrap();
                } else {
                    media_handle_for_toggle.play().unwrap();
                }
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 100.px(),
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
                media_handle_for_toggle.pause().unwrap();
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 140.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: "opus (Fire & Forget)",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(media) = audio_opus.as_ref() {
                    media.clone_independent().unwrap().play().unwrap();
                }
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 180.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: if sliced_audio.is_some() {
                "slice (Fire & Forget)"
            } else {
                "Loading..."
            },
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(audio) = sliced_audio.as_ref() {
                    audio.clone().play().unwrap();
                }
            },
        });

        ctx.add(TextButton {
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
                        video_mp4.pause().unwrap();
                    } else {
                        video_mp4.play().unwrap();
                    }
                }
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 300.px(),
                y: 60.px(),
                width: 30.px(),
                height: 20.px(),
            },
            text: "-5sec",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(video_mp4) = video_mp4.as_ref() {
                    println!(
                        "video_mp4.playback_duration(): {:?}",
                        video_mp4.playback_duration()
                    );
                    println!(
                        "video_mp4.playback_duration() - Duration::from_secs(5): {:?}",
                        video_mp4.playback_duration() - Duration::from_secs(5)
                    );
                    seek_to(
                        video_mp4.clone(),
                        video_mp4.playback_duration() - Duration::from_secs(5),
                    );
                }
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 300.px(),
                y: 40.px(),
                width: 100.px(),
                height: 20.px(),
            },
            text: &if let Some(video_mp4) = video_mp4.as_ref() {
                format!("{:.1?}", video_mp4.playback_duration().as_secs_f32())
            } else {
                "Loading...".to_string()
            },
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![],
            on_mouse_up_in: &|_| {},
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 370.px(),
                y: 60.px(),
                width: 30.px(),
                height: 20.px(),
            },
            text: "+5sec",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(video_mp4) = video_mp4.as_ref() {
                    println!(
                        "video_mp4.playback_duration(): {:?}",
                        video_mp4.playback_duration()
                    );
                    println!(
                        "video_mp4.playback_duration() + Duration::from_secs(5): {:?}",
                        video_mp4.playback_duration() + Duration::from_secs(5)
                    );
                    seek_to(
                        video_mp4.clone(),
                        video_mp4.playback_duration() + Duration::from_secs(5),
                    );
                }
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 20.px(),
                y: 220.px(),
                width: 40.px(),
                height: 20.px(),
            },
            text: "vol -",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                system::media::set_volume(system::media::volume() - 0.05);
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 60.px(),
                y: 220.px(),
                width: 40.px(),
                height: 20.px(),
            },
            text: &(system::media::volume() * 100.0).floor().to_string(),
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![],
            on_mouse_up_in: &|_| {},
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 100.px(),
                y: 220.px(),
                width: 40.px(),
                height: 20.px(),
            },
            text: "vol +",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                system::media::set_volume(system::media::volume() + 0.05);
            },
        });

        ctx.compose(|ctx| {
            let Some(mp4) = video_mp4.as_ref() else {
                return;
            };
            let Some(image_handle) = mp4.get_image() else {
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
    }
}

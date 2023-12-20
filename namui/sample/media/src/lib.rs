use namui::prelude::*;
use namui_prebuilt::{button::TextButton, typography};

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, || MediaExample).await
}

#[namui::component]
struct MediaExample;

// impl MediaExample {
//     fn new() -> Self {
//         Self {
//             media: Media::new(
//                 Url::parse("bundle:resources/media.mp3").unwrap(),
//                 || namui::event::send(Event::MediaLoaded),
//                 |error| {
//                     namui::log!("media error: {:?}", error);
//                 },
//             ),
//         }
//     }
// }

// enum Event {
//     MediaLoaded,
//     PlayMedia,
//     PlayAndForget,
//     ToggleLoop,
//     StopMedia,
// }

impl Component for MediaExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (media, set_media) = ctx.state(|| None);

        ctx.effect("load media", || {
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path("bundle:resources/audio.mp3")
                    .unwrap();

                let media = namui::system::media::new_media(&path).unwrap();

                set_media.set(Some(media));
            });
        });

        ctx.component(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 20.px(),
                width: 100.px(),
                height: 20.px(),
            },
            text: "play",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::WHITE,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: &|_| {
                if let Some(media_source) = media.as_ref() {
                    namui::system::media::play_media_source(media_source)
                }
            },
        });

        // ctx.component([
        //     typography::body::left(
        //         20.px(),
        //         format!(
        //             "is loaded: {}, is loop: {}",
        //             self.media.is_loaded(),
        //             self.media.is_loop()
        //         ),
        //         Color::BLACK,
        //     ),
        //     text_button(
        //         Rect::Xywh {
        //             x: 10.px(),
        //             y: 20.px(),
        //             width: 100.px(),
        //             height: 20.px(),
        //         },
        //         "play",
        //         Color::BLACK,
        //         Color::BLACK,
        //         1.px(),
        //         Color::WHITE,
        //         [MouseButton::Left],
        //         {
        //             move |_| {
        //                 namui::event::send(Event::PlayMedia);
        //             }
        //         },
        //     ),
        //     text_button(
        //         Rect::Xywh {
        //             x: 120.px(),
        //             y: 20.px(),
        //             width: 100.px(),
        //             height: 20.px(),
        //         },
        //         "stop",
        //         Color::BLACK,
        //         Color::BLACK,
        //         1.px(),
        //         Color::WHITE,
        //         [MouseButton::Left],
        //         {
        //             move |_| {
        //                 namui::event::send(Event::StopMedia);
        //             }
        //         },
        //     ),
        //     text_button(
        //         Rect::Xywh {
        //             x: 10.px(),
        //             y: 60.px(),
        //             width: 100.px(),
        //             height: 20.px(),
        //         },
        //         "fire and forget",
        //         Color::BLACK,
        //         Color::BLACK,
        //         1.px(),
        //         Color::WHITE,
        //         [MouseButton::Left],
        //         {
        //             move |_| {
        //                 namui::event::send(Event::PlayAndForget);
        //             }
        //         },
        //     ),
        //     text_button(
        //         Rect::Xywh {
        //             x: 10.px(),
        //             y: 100.px(),
        //             width: 100.px(),
        //             height: 20.px(),
        //         },
        //         "toggle loop",
        //         Color::BLACK,
        //         Color::BLACK,
        //         1.px(),
        //         Color::WHITE,
        //         [MouseButton::Left],
        //         {
        //             move |_| {
        //                 namui::event::send(Event::ToggleLoop);
        //             }
        //         },
        //     ),
        // ]);

        ctx.done()
    }

    // fn update(&mut self, event: &namui::Event) {
    //     event.is::<Event>(|event| match event {
    //         Event::MediaLoaded => {
    //             namui::log!("media loaded");
    //         }
    //         Event::PlayMedia => {
    //             self.media.play();
    //         }
    //         Event::PlayAndForget => {
    //             // let mut media = self.media.clone();
    //             // media.set_loop(false);
    //             // media.play();

    //             // or
    //             self.media.play_and_forget();
    //         }
    //         Event::ToggleLoop => {
    //             self.media.set_loop(!self.media.is_loop());
    //         }
    //         Event::StopMedia => {
    //             self.media.stop();
    //         }
    //     });
    // }
}

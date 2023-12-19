use namui::prelude::*;
use namui_prebuilt::{button::TextButton, typography};

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, || AudioExample).await
}

#[namui::component]
struct AudioExample;

// impl AudioExample {
//     fn new() -> Self {
//         Self {
//             audio: Audio::new(
//                 Url::parse("bundle:resources/audio.mp3").unwrap(),
//                 || namui::event::send(Event::AudioLoaded),
//                 |error| {
//                     namui::log!("audio error: {:?}", error);
//                 },
//             ),
//         }
//     }
// }

// enum Event {
//     AudioLoaded,
//     PlayAudio,
//     PlayAndForget,
//     ToggleLoop,
//     StopAudio,
// }

impl Component for AudioExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (audio_source, set_audio_source) = ctx.state(|| None);

        ctx.effect("load audio", || {
            namui::spawn(async move {
                let audio_vec = namui::system::file::bundle::read("bundle:resources/audio.mp3")
                    .await
                    .unwrap();

                let audio_source = namui::system::audio::new_audio_source(
                    Some("mp3"),
                    Some("audio/mpeg"),
                    audio_vec,
                )
                .unwrap();

                set_audio_source.set(Some(audio_source));
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
                if let Some(audio_source) = audio_source.as_ref() {
                    namui::system::audio::play_audio_source(audio_source)
                }
            },
        });

        // ctx.component([
        //     typography::body::left(
        //         20.px(),
        //         format!(
        //             "is loaded: {}, is loop: {}",
        //             self.audio.is_loaded(),
        //             self.audio.is_loop()
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
        //                 namui::event::send(Event::PlayAudio);
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
        //                 namui::event::send(Event::StopAudio);
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
    //         Event::AudioLoaded => {
    //             namui::log!("audio loaded");
    //         }
    //         Event::PlayAudio => {
    //             self.audio.play();
    //         }
    //         Event::PlayAndForget => {
    //             // let mut audio = self.audio.clone();
    //             // audio.set_loop(false);
    //             // audio.play();

    //             // or
    //             self.audio.play_and_forget();
    //         }
    //         Event::ToggleLoop => {
    //             self.audio.set_loop(!self.audio.is_loop());
    //         }
    //         Event::StopAudio => {
    //             self.audio.stop();
    //         }
    //     });
    // }
}

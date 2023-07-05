use namui::prelude::*;
use namui_prebuilt::{button::text_button, typography};

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut AudioExample::new(), &()).await
}

struct AudioExample {
    audio: Audio,
}

impl AudioExample {
    fn new() -> Self {
        Self {
            audio: Audio::new(
                Url::parse("bundle:resources/audio.mp3").unwrap(),
                || namui::event::send(Event::AudioLoaded),
                |error| {
                    namui::log!("audio error: {:?}", error);
                },
            ),
        }
    }
}

enum Event {
    AudioLoaded,
    PlayAudio,
    PlayAndForget,
    ToggleLoop,
    StopAudio,
}

impl Entity for AudioExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        render([
            typography::body::left(
                20.px(),
                format!(
                    "is loaded: {}, is loop: {}",
                    self.audio.is_loaded(),
                    self.audio.is_loop()
                ),
                Color::BLACK,
            ),
            text_button(
                Rect::Xywh {
                    x: 10.px(),
                    y: 20.px(),
                    width: 100.px(),
                    height: 20.px(),
                },
                "play",
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::WHITE,
                [MouseButton::Left],
                {
                    move |_| {
                        namui::event::send(Event::PlayAudio);
                    }
                },
            ),
            text_button(
                Rect::Xywh {
                    x: 120.px(),
                    y: 20.px(),
                    width: 100.px(),
                    height: 20.px(),
                },
                "stop",
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::WHITE,
                [MouseButton::Left],
                {
                    move |_| {
                        namui::event::send(Event::StopAudio);
                    }
                },
            ),
            text_button(
                Rect::Xywh {
                    x: 10.px(),
                    y: 60.px(),
                    width: 100.px(),
                    height: 20.px(),
                },
                "fire and forget",
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::WHITE,
                [MouseButton::Left],
                {
                    move |_| {
                        namui::event::send(Event::PlayAndForget);
                    }
                },
            ),
            text_button(
                Rect::Xywh {
                    x: 10.px(),
                    y: 100.px(),
                    width: 100.px(),
                    height: 20.px(),
                },
                "toggle loop",
                Color::BLACK,
                Color::BLACK,
                1.px(),
                Color::WHITE,
                [MouseButton::Left],
                {
                    move |_| {
                        namui::event::send(Event::ToggleLoop);
                    }
                },
            ),
        ])
    }

    fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::AudioLoaded => {
                namui::log!("audio loaded");
            }
            Event::PlayAudio => {
                self.audio.play();
            }
            Event::PlayAndForget => {
                // let mut audio = self.audio.clone();
                // audio.set_loop(false);
                // audio.play();

                // or
                self.audio.play_and_forget();
            }
            Event::ToggleLoop => {
                self.audio.set_loop(!self.audio.is_loop());
            }
            Event::StopAudio => {
                self.audio.stop();
            }
        });
    }
}

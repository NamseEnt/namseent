use namui::prelude::*;
use namui_prebuilt::{simple_rect, table};
use quick::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut TuiExample::new(), &()).await
}

struct TuiExample {}

impl TuiExample {
    fn new() -> Self {
        Self {}
    }
}

impl Entity for TuiExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let wh = namui::screen::size();

        let content = quick::vertical([
            quick::block(
                "Tabs",
                [
                    quick::line([
                        link("Tab 0"),
                        link("Tab 1"),
                    ]),
                ],
            ),
            quick::block(
                "Tabs",
                [
                    quick::line([
                        link("Tab 0"),
                        link("Tab 1"),
                    ]),
                ],
            ),
        ])
        .render(wh);

        render([
            simple_rect(wh, Color::TRANSPARENT, 0.px(), quick::color::BACKGROUND),
            content,
        ])
    }

    fn update(&mut self, _event: &namui::Event) {}
}

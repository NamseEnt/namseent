use namui::*;
use namui_prebuilt::simple_rect;
use quick::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(TuiExample {});
    })
}

struct TuiExample {}

impl Entity for TuiExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let wh = namui::screen::size();

        let content = quick::vertical([
            quick::block(
                "v block 0",
                [
                    quick::line([
                        link("Tab 0"),
                        link("Tab 1"),
                    ]),
                    quick::line([
                        link("Tab 2"),
                        link("Tab 3"),
                    ]),
                ],
            ),
            quick::block(
                "v block 1",
                [
                    quick::line([
                        link("Tab 0"),
                        link("Tab 1"),
                    ]),
                ],
            ),
            quick::horizontal([
                quick::block(
                    "h block 0",
                    [
                        quick::line([
                            link("Tab 0"),
                            link("Tab 1"),
                        ]),
                    ],
                ),
                quick::block(
                    "h block 1",
                    [
                        quick::line([
                            link("Tab 0"),
                            link("Tab 1"),
                        ]),
                    ],
                ),
            ]),
        ])
        .render(wh);

        render([
            simple_rect(wh, Color::TRANSPARENT, 0.px(), quick::color::BACKGROUND),
            content,
        ])
    }

    fn update(&mut self, _event: &namui::Event) {}
}

use super::*;
use crate::components::sequence_player::get_inner_content_rect;

impl ScreenEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let screen_images = self.wysiwyg_editor.screen_images.clone();
        let done = self.done.clone();

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            table::vertical([
                table::fixed(
                    20.px(),
                    table::horizontal([
                        table::ratio(1, |_wh| RenderingTree::Empty),
                        table::fit(
                            table::FitAlign::RightBottom,
                            button::text_button_fit(
                                20.px(),
                                "Done",
                                Color::WHITE,
                                Color::WHITE,
                                1.px(),
                                Color::BLACK,
                                12.px(),
                                [MouseButton::Left],
                                move |_| done(screen_images.clone()),
                            ),
                        ),
                    ]),
                ),
                table::ratio(1, |wh| self.render_images_with_wysiwyg_editor(wh, &props)),
            ])(props.wh),
        ])
    }

    fn render_images_with_wysiwyg_editor(&self, wh: Wh<Px>, props: &Props) -> RenderingTree {
        let rect = get_inner_content_rect(wh);
        translate(
            rect.x(),
            rect.y(),
            render([
                self.wysiwyg_editor
                    .render(wysiwyg_editor::Props { wh: rect.wh() }),
                sequence_player::render_text_box(rect.wh()),
                sequence_player::render_text(
                    props.project_shared_data,
                    rect.wh(),
                    props.cut,
                    1.one_zero(),
                ),
            ]),
        )
    }
}

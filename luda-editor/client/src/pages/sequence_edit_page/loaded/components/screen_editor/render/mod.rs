use super::*;
use crate::{components::sequence_player::get_inner_content_rect, storage::get_project_image_url};

impl ScreenEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            table::vertical([
                table::fixed(
                    20.px(),
                    table::horizontal([
                        table::fit(
                            table::FitAlign::LeftTop,
                            button::text_button_fit(
                                20.px(),
                                "Preview",
                                Color::WHITE,
                                Color::WHITE,
                                1.px(),
                                Color::BLACK,
                                12.px(),
                                || todo!(),
                            ),
                        ),
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
                                || todo!(),
                            ),
                        ),
                    ]),
                ),
                table::ratio(1, |wh| self.render_images_with_wysiwyg_editor(wh)),
            ])(props.wh),
        ])
    }

    fn render_images_with_wysiwyg_editor(&self, wh: Wh<Px>) -> RenderingTree {
        let rect = get_inner_content_rect(wh);
        translate(
            rect.x(),
            rect.y(),
            self.wysiwyg_editor
                .render(wysiwyg_editor::Props { wh: rect.wh() }),
        )
    }
}

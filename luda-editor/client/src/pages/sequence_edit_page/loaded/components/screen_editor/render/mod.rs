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
                table::ratio(1, |wh| self.render_images(wh)),
            ])(props.wh),
        ])
    }

    fn render_images(&self, wh: Wh<Px>) -> RenderingTree {
        let rect = get_inner_content_rect(wh);
        let wh = rect.wh();
        translate(
            rect.x(),
            rect.y(),
            render(
                [simple_rect(rect.wh(), Color::WHITE, 1.px(), Color::BLACK)]
                    .into_iter()
                    .chain(self.screen_images.iter().map(|screen_image| {
                        let url = get_project_image_url(self.project_id, screen_image.id).unwrap();
                        let image = namui::image::try_load_url(&url);

                        if image.is_none() {
                            return RenderingTree::Empty;
                        }
                        let image = image.unwrap();

                        let center_xy = wh.as_xy() * screen_image.center_percent_xy;
                        let screen_radius = wh.length() / 2;
                        let radius_px = screen_radius * screen_image.radius_percent;

                        let image_radius_px = image.size().length() / 2;
                        let image_to_screen_ratio = radius_px / image_radius_px;

                        let wh_px = image.size() * image_to_screen_ratio;
                        let left_top_px = center_xy - (wh_px.as_xy() / 2);
                        let rect = Rect::from_xy_wh(left_top_px, wh_px);

                        namui::image(ImageParam {
                            rect,
                            source: ImageSource::Image(image),
                            style: ImageStyle {
                                fit: ImageFit::Fill,
                                paint_builder: None,
                            },
                        })
                    })),
            ),
        )
    }
}

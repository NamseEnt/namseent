use super::*;
use std::str::FromStr;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageSelectModal {
    pub fn render_image_preview(&self, props: Props) -> namui::RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Preview", Color::WHITE),
        );

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            title,
            table::vertical([
                table::ratio(1, |wh| {
                    namui::try_render(|| {
                        Some(namui::image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                            source: ImageSource::Url(
                                namui::Url::from_str(&self.selected_image.as_ref()?.url).unwrap(),
                            ),
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint_builder: None,
                            },
                        }))
                    })
                }),
                table::ratio(1, |wh| {
                    namui::try_render(|| {
                        let label_title = typography::body::left_top_bold("Labels", Color::WHITE);

                        let label_text = self
                            .selected_image
                            .as_ref()?
                            .labels
                            .iter()
                            .map(|label| format!("{}:{}", label.key, label.value))
                            .collect::<Vec<_>>()
                            .join(" / ");

                        Some(render([
                            translate(12.px(), 12.px(), label_title),
                            text(TextParam {
                                text: label_text,
                                x: 12.px(),
                                y: 24.px() + typography::body::FONT_SIZE,
                                align: TextAlign::Left,
                                baseline: TextBaseline::Top,
                                font_type: FontType {
                                    serif: false,
                                    size: typography::body::FONT_SIZE,
                                    language: Language::Ko,
                                    font_weight: FontWeight::REGULAR,
                                },
                                style: TextStyle {
                                    border: None,
                                    drop_shadow: None,
                                    color: Color::WHITE,
                                    background: None,
                                },
                                max_width: Some(wh.width - 24.px()),
                            }),
                        ]))
                    })
                }),
                table::fixed(64.px(), |wh| {
                    let padding = 12.px();
                    let button_height = wh.height - padding * 2;
                    let cancel_button = button::text_button_fit(
                        button_height,
                        "Cancel",
                        Color::WHITE,
                        Color::WHITE,
                        1.px(),
                        Color::BLACK,
                        padding,
                        || namui::event::send(Event::Close),
                    );
                    let image_id = self.selected_image.as_ref().map(|image| image.id);
                    let confirm_button = button::text_button_fit(
                        button_height,
                        "Confirm",
                        Color::WHITE,
                        Color::WHITE,
                        1.px(),
                        Color::BLACK,
                        padding,
                        move || namui::event::send(InternalEvent::Done { image_id }),
                    );
                    let cancel_button_width = match cancel_button.get_bounding_box() {
                        Some(bounding_box) => bounding_box.width(),
                        None => return RenderingTree::Empty,
                    };
                    let confirm_button_width = match confirm_button.get_bounding_box() {
                        Some(bounding_box) => bounding_box.width(),
                        None => return RenderingTree::Empty,
                    };

                    render([
                        translate(
                            wh.width
                                - padding
                                - confirm_button_width
                                - padding
                                - cancel_button_width,
                            padding,
                            cancel_button,
                        ),
                        translate(
                            wh.width - padding - confirm_button_width,
                            padding,
                            confirm_button,
                        ),
                    ])
                }),
            ])(props.wh),
        ])
    }
}

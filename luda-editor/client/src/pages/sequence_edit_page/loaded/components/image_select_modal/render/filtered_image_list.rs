use super::*;
use std::str::FromStr;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageSelectModal {
    pub fn render_filtered_image_list(&self, props: Props) -> namui::RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Select Image", Color::WHITE),
        );

        let add_image_button = button::text_button(
            Rect::Xywh {
                x: props.wh.width - 12.px() - 128.px(),
                y: 8.px(),
                width: 128.px(),
                height: 28.px(),
            },
            "+ Add Image",
            Color::WHITE,
            Color::WHITE,
            1.px(),
            Color::BLACK,
            || {
                namui::event::send(InternalEvent::AddImageButtonClicked);
            },
        );

        let filtered_images = self
            .images
            .iter()
            .filter(|image| {
                image
                    .labels
                    .iter()
                    .any(|label| self.selected_labels.contains(label))
            })
            .collect::<Vec<_>>();

        let row_cell_count = 4;

        let mut rows = vec![];

        let padding = 12.px();

        for index in (0..filtered_images.len()).step_by(row_cell_count) {
            let row_index = index / row_cell_count;
            let row_images = filtered_images.iter().skip(index).take(row_cell_count);

            let image_width = (props.wh.width - padding * 5) / row_cell_count;
            let row = row_images.enumerate().map(|(column_index, image)| {
                translate(
                    column_index * (image_width + padding) + padding,
                    row_index * (image_width + padding),
                    render([
                        simple_rect(Wh::single(image_width), Color::WHITE, 1.px(), Color::BLACK),
                        namui::image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::zero(), Wh::single(image_width)),
                            source: ImageSource::Url(namui::Url::from_str(&image.url).unwrap()),
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint_builder: None,
                            },
                        }),
                    ])
                    .attach_event(move |builder| {
                        let image = (*image).clone();
                        builder.on_mouse_down_in(move |_| {
                            namui::event::send(InternalEvent::ImageSelected(image.clone()));
                        });
                    }),
                )
            });

            rows.push(render(row));
        }

        let title_height_with_padding = 24.px() + typography::title::FONT_SIZE;

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            title,
            add_image_button,
            self.image_list_scroll_view.render(&scroll_view::Props {
                xy: Xy::new(0.px(), title_height_with_padding),
                height: props.wh.height - title_height_with_padding,
                scroll_bar_width: 4.px(),
                content: render(rows),
            }),
        ])
    }
}

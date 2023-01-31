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
            [MouseButton::Left],
            |_| {
                namui::event::send(InternalEvent::AddImageButtonClicked);
            },
        );

        let filtered_images = self.get_filtered_images();

        let padding = 12.px();
        let image_width = (props.wh.width - padding * 5) / Self::ROW_CELL_COUNT;
        let row_images_list = (0..filtered_images.len())
            .step_by(Self::ROW_CELL_COUNT)
            .map(|index| {
                let row_images = filtered_images
                    .iter()
                    .skip(index)
                    .take(Self::ROW_CELL_COUNT);
                row_images
            });

        let title_height_with_padding = 24.px() + typography::title::FONT_SIZE;

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            title,
            add_image_button,
            self.image_list_view.render(list_view::Props {
                xy: Xy::new(0.px(), title_height_with_padding),
                height: props.wh.height - title_height_with_padding,
                scroll_bar_width: 4.px(),
                item_wh: Wh::new(props.wh.width, image_width + padding),
                items: row_images_list,
                item_render: |_wh, item| {
                    render(item.enumerate().map(|(column_index, image)| {
                        let is_selected =
                            Some(image.id) == self.selected_image.as_ref().map(|image| image.id);

                        translate(
                            (image_width + padding) * column_index + padding,
                            0.px(),
                            render([
                                namui::image(ImageParam {
                                    rect: Rect::from_xy_wh(Xy::zero(), Wh::single(image_width)),
                                    source: ImageSource::Url(
                                        namui::Url::from_str(&image.url).unwrap(),
                                    ),
                                    style: ImageStyle {
                                        fit: ImageFit::Contain,
                                        paint_builder: None,
                                    },
                                }),
                                simple_rect(
                                    Wh::single(image_width),
                                    if is_selected {
                                        Color::RED
                                    } else {
                                        Color::WHITE
                                    },
                                    1.px(),
                                    Color::TRANSPARENT,
                                ),
                            ])
                            .attach_event(move |builder| {
                                let image = (*image).clone();
                                builder.on_mouse_down_in(move |_| {
                                    namui::event::send(InternalEvent::ImageSelected {
                                        image: image.clone(),
                                        update_labels: false,
                                    });
                                });
                            }),
                        )
                    }))
                },
            }),
        ])
        .attach_event(move |builder| {
            builder.on_key_down(move |event| {
                namui::event::send(InternalEvent::ImageListKeyDown(event.code));
            });
        })
    }
}

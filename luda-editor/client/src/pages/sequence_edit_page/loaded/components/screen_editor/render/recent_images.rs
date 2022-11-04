use super::*;

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub recent_selected_image_ids: &'a VecDeque<Uuid>,
}

impl ImageSelectModal {
    pub fn render_recent_images(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            table::vertical([
                table::fit(
                    table::FitAlign::LeftTop,
                    typography::title::left_top("Recent", Color::WHITE).padding(12.px()),
                ),
                table::ratio(1, |wh| {
                    let column_count = ((wh.width / wh.height).floor() as usize)
                        .min(props.recent_selected_image_ids.len());
                    if column_count == 0 {
                        return RenderingTree::Empty;
                    }

                    table::horizontal((0..column_count).into_iter().map(|column_index| {
                        let image_id = props.recent_selected_image_ids[column_index];
                        let image = self.images.iter().find(|image| image.id == image_id);
                        let is_selected =
                            self.selected_image.as_ref().map(|image| image.id) == Some(image_id);

                        table::fixed(
                            wh.height,
                            table::padding(8.px(), move |wh| match image {
                                Some(image) => render([
                                    namui::image(ImageParam {
                                        rect: Rect::zero_wh(wh),
                                        source: ImageSource::Url(
                                            namui::Url::parse(&image.url).unwrap(),
                                        ),
                                        style: ImageStyle {
                                            fit: ImageFit::Contain,
                                            paint_builder: None,
                                        },
                                    }),
                                    simple_rect(
                                        wh,
                                        if is_selected {
                                            Color::RED
                                        } else {
                                            Color::WHITE
                                        },
                                        1.px(),
                                        Color::TRANSPARENT,
                                    ),
                                ])
                                .attach_event(|builder| {
                                    let image = image.clone();
                                    builder.on_mouse_down_in(move |_| {
                                        namui::event::send(InternalEvent::ImageSelected {
                                            image: image.clone(),
                                            update_labels: true,
                                        });
                                    });
                                }),
                                None => RenderingTree::Empty,
                            }),
                        )
                    }))(wh)
                }),
            ])(props.wh),
        ])
    }
}

use super::*;

pub struct Row {
    pub image_id: Uuid,
    pub labels: Vec<Label>,
}

impl Row {
    pub fn render(&self, image_table: &ImageTable) -> RenderingTree {
        let project_id = image_table.project_id;
        let cell_wh = Wh::new(COLUMN_WIDTH, ROW_HEIGHT);
        let image_id = self.image_id;

        let image = render([
            border(cell_wh),
            namui::try_render(|| {
                let url = get_project_image_url(project_id, self.image_id).unwrap();
                let image = namui::image::try_load_url(&url)?;

                Some(namui::image(ImageParam {
                    rect: Rect::from_xy_wh(Xy::zero(), cell_wh),
                    source: ImageSource::Image(image),
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint_builder: None,
                    },
                }))
            }),
        ]);

        render(
            [image]
                .into_iter()
                .chain(self.labels.iter().map(|label| {
                    let text = {
                        match image_table.editing_target.as_ref() {
                            Some(editing_target)
                                if editing_target.image_id == self.image_id
                                    && editing_target.label_key == label.key =>
                            {
                                image_table.text_input.render(text_input::Props {
                                    rect: Rect::from_xy_wh(Xy::zero(), cell_wh),
                                    text: label.value.clone(),
                                    text_align: TextAlign::Center,
                                    text_baseline: TextBaseline::Middle,
                                    font_type: FontType {
                                        serif: false,
                                        size: FONT_SIZE,
                                        language: Language::Ko,
                                        font_weight: FontWeight::REGULAR,
                                    },
                                    style: text_input::Style {
                                        text: TextStyle {
                                            color: Color::WHITE,
                                            ..Default::default()
                                        },
                                        rect: RectStyle {
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    event_handler: None,
                                })
                            }
                            _ => center_text(cell_wh, &label.value, Color::WHITE, FONT_SIZE),
                        }
                    };

                    render([border(cell_wh), text]).attach_event(move |builder| {
                        let label_key = label.key.clone();
                        builder.on_mouse_down_in(move |event| {
                            if event.button == Some(MouseButton::Left) {
                                namui::event::send(InternalEvent::LeftClickOnLabelCell {
                                    image_id,
                                    label_key: label_key.clone(),
                                });
                            }
                        });
                    })
                }))
                .enumerate()
                .map(|(index, rendering_tree)| {
                    translate(COLUMN_WIDTH * index, 0.px(), rendering_tree)
                }),
        )
    }
}

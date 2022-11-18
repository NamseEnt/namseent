use super::*;

pub struct Row {
    pub image_id: Uuid,
    pub labels: Vec<Label>,
}

impl Row {
    pub fn render(&self, image_table: &ImageTable, row_index: usize) -> RenderingTree {
        let project_id = image_table.project_id;
        let cell_wh = Wh::new(COLUMN_WIDTH, ROW_HEIGHT);
        let image_id = self.image_id;

        let image = render([
            border(cell_wh),
            path(
                PathBuilder::new()
                    .move_to(cell_wh.width - 2.5.px(), 0.px())
                    .line_to(cell_wh.width - 2.5.px(), cell_wh.height),
                PaintBuilder::new()
                    .set_color(Color::WHITE)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(1.px()),
            ),
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
                .chain(self.labels.iter().enumerate().map(|(column_index, label)| {
                    let is_dragged_cell = is_dragged_cell(image_table, row_index, column_index);
                    let fill_rect = simple_rect(
                        cell_wh,
                        Color::TRANSPARENT,
                        0.px(),
                        if is_dragged_cell {
                            Color::from_u8(37, 49, 109, 255)
                        } else {
                            Color::TRANSPARENT
                        },
                    );
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
                            _ => namui::text(TextParam {
                                text: label.value.clone(),
                                x: cell_wh.width / 2.0,
                                y: cell_wh.height / 2.0,
                                align: TextAlign::Center,
                                baseline: TextBaseline::Middle,
                                font_type: FontType {
                                    font_weight: FontWeight::REGULAR,
                                    language: Language::Ko,
                                    serif: false,
                                    size: FONT_SIZE,
                                },
                                style: TextStyle {
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                                max_width: Some(cell_wh.width),
                            }),
                        }
                    };

                    render([fill_rect, border(cell_wh), text]).attach_event(move |builder| {
                        let label_key = label.key.clone();
                        builder.on_mouse_down_in(move |event| {
                            if event.button == Some(MouseButton::Left) {
                                namui::event::send(InternalEvent::LabelCellMouseLeftDown {
                                    image_id,
                                    label_key: label_key.clone(),
                                    row_index,
                                    column_index,
                                });
                            }
                        });
                        let label_key = label.key.clone();
                        builder.on_mouse_up_in(move |event| {
                            if event.button == Some(MouseButton::Left) {
                                namui::event::send(InternalEvent::LabelCellMouseLeftUp {
                                    image_id,
                                    label_key: label_key.clone(),
                                    row_index,
                                    column_index,
                                });
                            }
                        });
                        let last_row_column_index =
                            image_table
                                .cell_drag_context
                                .as_ref()
                                .map(|cell_drag_context| {
                                    (
                                        cell_drag_context.last_row_index,
                                        cell_drag_context.last_column_index,
                                    )
                                });
                        builder.on_mouse_move_in(move |_event| {
                            if let Some((last_row_index, last_column_index)) = last_row_column_index
                            {
                                if last_row_index != row_index || last_column_index != column_index
                                {
                                    namui::event::send(InternalEvent::LabelCellMouseMove {
                                        row_index,
                                        column_index,
                                    });
                                }
                            }
                        });
                    })
                }))
                .enumerate()
                .map(|(index, rendering_tree)| {
                    translate(COLUMN_WIDTH * index, 0.px(), rendering_tree)
                }),
        )
        .attach_event(move |builder| {
            builder.on_mouse_down_in(move |event| {
                if event.button == Some(MouseButton::Right) {
                    namui::event::send(InternalEvent::RightClickOnImageRow {
                        image_id,
                        global_xy: event.global_xy,
                    });
                }
            });
        })
    }
}

fn is_dragged_cell(image_table: &ImageTable, row_index: usize, column_index: usize) -> bool {
    let ltrb = if let Some(cell_drag_context) = image_table.cell_drag_context.as_ref() {
        Some(Ltrb {
            left: cell_drag_context
                .start_column_index
                .min(cell_drag_context.last_column_index),
            top: cell_drag_context
                .start_row_index
                .min(cell_drag_context.last_row_index),
            right: cell_drag_context
                .start_column_index
                .max(cell_drag_context.last_column_index),
            bottom: cell_drag_context
                .start_row_index
                .max(cell_drag_context.last_row_index),
        })
    } else if let Some(selection) = image_table.selection.as_ref() {
        Some(*selection)
    } else {
        None
    };

    if let Some(ltrb) = ltrb {
        ltrb.left <= column_index
            && column_index <= ltrb.right
            && ltrb.top <= row_index
            && row_index <= ltrb.bottom
    } else {
        false
    }
}

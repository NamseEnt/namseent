use super::*;

impl ImageTable {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<InternalEvent>(|event| match event {
                InternalEvent::LoadImages(images) => {
                    self.images = images.clone();
                }
                InternalEvent::LeftClickOnLabelHeader { key } => {
                    self.sort_order_by = match self.sort_order_by.as_ref() {
                        None => Some(SortOrderBy::Ascending { key: key.clone() }),
                        Some(sort_order_by) => match sort_order_by {
                            SortOrderBy::Ascending { key: current_key }
                            | SortOrderBy::Descending { key: current_key }
                                if current_key.ne(key) =>
                            {
                                Some(SortOrderBy::Ascending { key: key.clone() })
                            }
                            SortOrderBy::Ascending { key: current_key } => {
                                Some(SortOrderBy::Descending {
                                    key: current_key.clone(),
                                })
                            }
                            SortOrderBy::Descending { key: _ } => None,
                        },
                    };
                }
                InternalEvent::PutImageMetaDataSuccess => {
                    self.saving_count -= 1;
                }
                &InternalEvent::RightClickOnImageRow {
                    image_id,
                    global_xy,
                } => {
                    let project_id = self.project_id;
                    self.context_menu = Some(context_menu::ContextMenu::new(
                        global_xy,
                        [context_menu::Item::new_button("Delete", move || {
                            crate::RPC
                                .delete_image(rpc::delete_image::Request {
                                    image_id,
                                    project_id,
                                })
                                .callback(move |result| match result {
                                    Ok(_) => {
                                        request_reload_images(project_id);
                                    }
                                    Err(error) => {
                                        namui::event::send(Event::Error(error.to_string()))
                                    }
                                })
                        })],
                    ))
                }
                &InternalEvent::LabelCellMouseLeftDown {
                    image_id,
                    ref label_key,
                    row_index,
                    column_index,
                } => {
                    if let Some(editing_target) = self.editing_target.as_ref() {
                        if editing_target.image_id != image_id
                            || editing_target.label_key.ne(label_key)
                        {
                            self.editing_target = None;
                        }
                    }
                    self.cell_drag_context = Some(CellDragContext {
                        start_row_index: row_index,
                        start_column_index: column_index,
                        last_row_index: row_index,
                        last_column_index: column_index,
                    })
                }
                &InternalEvent::LabelCellMouseMove {
                    row_index,
                    column_index,
                } => {
                    if let Some(cell_drag_context) = self.cell_drag_context.as_mut() {
                        if self.editing_target.is_some()
                            && (row_index != cell_drag_context.start_row_index
                                || column_index != cell_drag_context.start_column_index)
                        {
                            self.editing_target = None;
                        }

                        cell_drag_context.last_row_index = row_index;
                        cell_drag_context.last_column_index = column_index;
                    }
                }
                &InternalEvent::LabelCellMouseLeftUp {
                    image_id,
                    ref label_key,
                    row_index,
                    column_index,
                } => {
                    let is_selected_only_this_cell =
                        if let Some(selection) = self.selection.as_ref() {
                            selection.top == row_index
                                && selection.bottom == row_index
                                && selection.left == column_index
                                && selection.right == column_index
                        } else {
                            false
                        };
                    let is_double_click_only_this_cell = is_selected_only_this_cell
                        && if let Some(cell_drag_context) = self.cell_drag_context.as_ref() {
                            cell_drag_context.start_row_index == row_index
                                && cell_drag_context.start_column_index == column_index
                        } else {
                            false
                        };

                    if is_double_click_only_this_cell {
                        self.editing_target = Some(EditingTarget {
                            image_id,
                            label_key: label_key.clone(),
                        });
                        self.text_input.focus();
                    }

                    if let Some(cell_drag_context) = self.cell_drag_context.take() {
                        self.selection = Some(Ltrb {
                            left: cell_drag_context.start_column_index.min(column_index),
                            top: cell_drag_context.start_row_index.min(row_index),
                            right: cell_drag_context.start_column_index.max(column_index),
                            bottom: cell_drag_context.start_row_index.max(row_index),
                        });
                    }
                }
                InternalEvent::EscKeyDown => {
                    if self.editing_target.is_some() {
                        self.editing_target = None;
                    } else {
                        self.selection = None;
                    }
                    self.context_menu = None;
                }
                &InternalEvent::EditLabel {
                    image_id,
                    ref key,
                    ref value,
                } => {
                    let mut updated_image = None;
                    if let Some(image) = self.images.iter_mut().find(|image| image.id.eq(&image_id))
                    {
                        if let Some(label) = image.labels.iter_mut().find(|label| label.key.eq(key))
                        {
                            label.value = value.clone();
                            updated_image = Some(image.clone());
                        }
                    }

                    if let Some(updated_image) = updated_image {
                        self.update_label(updated_image);
                    }
                }
            })
            .is::<context_menu::Event>(|event| match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            });

        self.list_view.update(event);
        self.context_menu.as_mut().map(|context_menu| {
            context_menu.update(event);
        });
        self.sheet.update(event);
    }
    fn update_label(&mut self, image: ImageWithLabels) {
        let project_id = self.project_id;
        self.saving_count += 1;
        spawn_local(async move {
            let result = crate::RPC
                .put_image_meta_data(rpc::put_image_meta_data::Request {
                    project_id,
                    image_id: image.id,
                    labels: image.labels,
                })
                .await;
            if let Err(error) = result {
                namui::event::send(Event::Error(error.to_string()));
            } else {
                namui::event::send(InternalEvent::PutImageMetaDataSuccess);
            }
        })
    }
}

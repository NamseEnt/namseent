use super::*;
use crate::pages::sequence_edit_page::loaded::components::image_upload::upload_images;

impl ImageSelectModal {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::AddImageButtonClicked => {
                    self.image_edit_modal = Some(image_edit_modal::ImageEditModal::new(
                        image_edit_modal::ModalPurpose::Add,
                        self.project_id,
                    ));
                }
                InternalEvent::LoadImages(images) => {
                    self.images = images.clone();
                }
                InternalEvent::ToggleLabel(label) => {
                    if self.selected_labels.contains(label) {
                        self.selected_labels.remove(label);
                    } else {
                        self.selected_labels
                            .retain(|selected_label| selected_label.key.ne(&label.key));
                        self.selected_labels.insert(label.clone());
                    }
                }
                InternalEvent::ImageSelected {
                    image,
                    update_labels,
                } => {
                    self.select_image(image.clone(), *update_labels, false);
                }
                InternalEvent::EditScreenPressed { screen_images } => {
                    self.screen_editor = Some(screen_editor::ScreenEditor::new(
                        self.project_id,
                        screen_images.clone(),
                        |screen_images| {
                            namui::event::send(InternalEvent::ScreenEditDone { screen_images });
                        },
                    ));
                }
                &InternalEvent::SelectScreenImageIndex { index } => {
                    self.selected_screen_image_index = Some(index);
                }
                InternalEvent::ScreenEditDone { screen_images } => {
                    self.screen_editor = None;
                    (self.on_update_image)(Update {
                        cut_id: self.cut_id,
                        screen_images: screen_images.clone(),
                    })
                }
                InternalEvent::RequestUploadBulkImages => {
                    let project_id = self.project_id;
                    spawn_local(async move {
                        let result = upload_images(project_id).await;
                        match result {
                            Ok(_) => request_reload_images(project_id),
                            Err(error) => namui::event::send(Event::Error(error.to_string())),
                        }
                    });
                }
                &InternalEvent::ImageListKeyDown(code) => {
                    let selected_image_row_column = self.get_selected_image_row_column();
                    if let Some((row, column)) = selected_image_row_column {
                        let next_row_column = self.get_row_column_on_keyboard_event(
                            code,
                            row,
                            column,
                            self.get_row_count(),
                            Self::ROW_CELL_COUNT,
                            self.get_filtered_images().len(),
                        );
                        if let Some((row, column)) = next_row_column {
                            let images = self.get_filtered_images();
                            let index = row * Self::ROW_CELL_COUNT + column;
                            let image = images.get(index);

                            if let Some(image) = image {
                                self.select_image((*image).clone(), false, true);
                            }
                        }
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<context_menu::Event>() {
            match event {
                context_menu::Event::Close => {
                    self.context_menu = None;
                }
            }
        } else if let Some(event) = event.downcast_ref::<image_edit_modal::Event>() {
            match event {
                image_edit_modal::Event::Close => {
                    self.image_edit_modal = None;
                    request_reload_images(self.project_id);
                }
                image_edit_modal::Event::Error(error) => {
                    namui::log!("image_edit_modal error: {}", error);
                }
            }
        }

        self.context_menu
            .as_mut()
            .map(|context_menu| context_menu.update(event));
        self.label_scroll_view.update(event);
        self.image_list_view.update(event);
        self.image_edit_modal
            .as_mut()
            .map(|image_edit_modal| image_edit_modal.update(event));
        self.screen_editor
            .as_mut()
            .map(|screen_editor| screen_editor.update(event));
    }
    fn select_image(&mut self, image: ImageWithLabels, update_labels: bool, scroll_to: bool) {
        self.selected_image = Some(image.clone());
        if update_labels {
            self.selected_labels = image.labels.clone().into_iter().collect();
        }
        if scroll_to {
            let row_of_selected_image = self.get_selected_image_row_column().map(|(row, _)| row);
            if let Some(row) = row_of_selected_image {
                self.image_list_view.scroll_to(row);
            }
        }
    }
}

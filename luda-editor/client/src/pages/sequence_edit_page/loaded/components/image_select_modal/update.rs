use super::*;

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
                    self.selected_image = Some(image.clone());
                    if *update_labels {
                        self.selected_labels = image.labels.clone().into_iter().collect();
                    }
                }
                InternalEvent::EditScreenPressed { screen_images } => {
                    self.screen_editor = Some(screen_editor::ScreenEditor::new(
                        self.project_id,
                        screen_images.clone(),
                    ));
                }
                &InternalEvent::SelectScreenImageIndex { index } => {
                    self.selected_screen_image_index = Some(index);
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
                    self.request_reload_images();
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
        self.image_list_scroll_view.update(event);
        self.image_edit_modal
            .as_mut()
            .map(|image_edit_modal| image_edit_modal.update(event));
        self.screen_editor
            .as_mut()
            .map(|screen_editor| screen_editor.update(event));
    }
}

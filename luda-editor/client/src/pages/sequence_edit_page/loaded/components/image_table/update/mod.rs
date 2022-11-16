use super::*;

impl ImageTable {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
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
                &InternalEvent::LeftClickOnLabelCell {
                    image_id,
                    ref label_key,
                } => {
                    self.editing_target = Some(EditingTarget {
                        image_id,
                        label_key: label_key.clone(),
                    });
                    self.text_input.focus();
                }
                InternalEvent::PutImageMetaDataSuccess => {
                    self.saving_count -= 1;
                }
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::TextUpdated { id, text } => {
                    if id == self.text_input.get_id() {
                        let editing_target = self.editing_target.as_ref().unwrap();
                        let mut updated_image = None;
                        if let Some(image) = self
                            .images
                            .iter_mut()
                            .find(|image| image.id.eq(&editing_target.image_id))
                        {
                            if let Some(label) = image
                                .labels
                                .iter_mut()
                                .find(|label| label.key.eq(&editing_target.label_key))
                            {
                                label.value = text.clone();
                                updated_image = Some(image.clone());
                            }
                        }

                        if let Some(updated_image) = updated_image {
                            self.update_label(updated_image);
                        }
                    }
                }
                _ => {}
            }
        }
        self.list_view.update(event);
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

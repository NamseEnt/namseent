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
                InternalEvent::EscKeyDown => {
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

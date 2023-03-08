use super::super::image_upload::*;
use super::*;

impl ImageEditModal {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<InternalEvent>(|event| match event {
                InternalEvent::ImageChanged { image } => {
                    self.image = Some(image.clone());
                }
                InternalEvent::DonePressed => {
                    let purpose = self.purpose;
                    let project_id = self.project_id;
                    let label_list = self.label_list.clone();
                    let image = self.image.clone();
                    spawn_local(async move {
                        let result = match purpose {
                            ModalPurpose::Add => {
                                create_image(project_id, label_list.clone(), {
                                    match &image {
                                        Some(image) => Some(image.content().await),
                                        None => None,
                                    }
                                })
                                .await
                            }
                            ModalPurpose::Edit => {
                                update_image(
                                    todo!(),
                                    #[allow(unreachable_code)]
                                    label_list.clone(),
                                    image.clone(),
                                )
                                .await
                            }
                        };
                        match result {
                            Ok(_) => {
                                namui::event::send(Event::Close);
                            }
                            Err(error) => namui::event::send(Event::Error(error.to_string())),
                        }
                    });
                }
                InternalEvent::LabelInputEnterPressed => {
                    let tuple = self.label_text.split(":").collect::<Vec<_>>();
                    if tuple.len() != 2 {
                        return;
                    }
                    let key = tuple[0].to_string();
                    let value = tuple[1].to_string();

                    self.label_list.push(Label {
                        key: key.to_string(),
                        value: value.to_string(),
                    });

                    self.label_text = "".to_string();
                }
            })
            .is::<text_input::Event>(|event| {
                if let text_input::Event::TextUpdated {
                    id,
                    text: updated_text,
                } = event
                {
                    if id == &self.label_text_input.get_id() {
                        self.label_text = updated_text.clone();
                    }
                }
            });
    }
}

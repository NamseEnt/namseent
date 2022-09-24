use super::*;

impl ImageEditModal {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::ImageChanged { image } => {
                    self.image = Some(image.clone());
                }
                InternalEvent::DonePressed => {
                    // todo: upload
                    spawn_local({
                        let future: std::pin::Pin<
                            Box<
                                dyn std::future::Future<
                                    Output = Result<(), Box<dyn std::error::Error>>,
                                >,
                            >,
                        > = match self.purpose {
                            ModalPurpose::Add => Box::pin(create_image(
                                self.project_id.clone(),
                                self.label_list.clone(),
                                self.image.clone(),
                            )),
                            ModalPurpose::Edit => Box::pin(update_image(
                                todo!(),
                                self.label_list.clone(),
                                self.image.clone(),
                            )),
                        };
                        async move {
                            let result = future.await;
                            match result {
                                Ok(_) => {
                                    namui::event::send(Event::Close);
                                }
                                Err(error) => namui::event::send(Event::Error(error.to_string())),
                            }
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
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            if let text_input::Event::TextUpdated {
                id,
                text: updated_text,
            } = event
            {
                if id.eq(self.label_text_input.get_id()) {
                    self.label_text = updated_text.clone();
                }
            }
        }
    }
}

async fn create_image(
    project_id: namui::Uuid,
    label_list: Vec<Label>,
    image: Option<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = crate::RPC
        .prepare_upload_image(rpc::prepare_upload_image::Request {
            project_id,
            label_list,
        })
        .await?;

    let body = match image {
        Some(file) => file.content().await,
        None => [].into(),
    };

    namui::network::http::fetch(
        response.upload_url,
        namui::network::http::Method::PUT,
        |builder| builder.body(body.to_vec()),
    )
    .await?;

    Ok(())
}

async fn delete_image(label_list: Vec<Label>) {
    todo!()
}

async fn update_image(
    prev_label_list: Vec<Label>,
    new_label_list: Vec<Label>,
    image: Option<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    /*
    1) 이미지를 수정할 경우
    -> 그냥 put 하면 돼
    2) 레이블을 수정할 경우
    -> move 하면 돼
        -> 근데 move는 Copy & Delete야.
    3) 이미지와 레이블 둘 다 수정할 경우
    -> put & Delete

    실패할 수 있다. 실패했다고 알려주면 된다. Delete를 먼저 하진 않는다.
     */
    todo!()
}

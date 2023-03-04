use super::*;
use crate::pages::sequence_edit_page::loaded::components::image_upload::upload_images_using_picker;

impl ImageManagerModal {
    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<image_table::Event>(|event| match event {
                image_table::Event::Error(error) => {
                    namui::event::send(Event::Error(error.clone()));
                }
            })
            .is::<InternalEvent>(|event| match event {
                InternalEvent::RequestUploadImages => {
                    let project_id = self.project_id;
                    spawn_local(async move {
                        let result = upload_images_using_picker(project_id).await;
                        match result {
                            Ok(_) => namui::event::send(InternalEvent::UploadImageFinished),
                            Err(error) => namui::event::send(Event::Error(error.to_string())),
                        }
                    });
                }
                InternalEvent::UploadImageFinished => {
                    self.image_table.request_reload_images();
                }
            });
        self.image_table.update(event);
    }
}

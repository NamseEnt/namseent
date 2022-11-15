use super::*;
use crate::pages::sequence_edit_page::loaded::components::image_upload::upload_images;

impl ImageManagerModal {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<image_table::Event>() {
            match event {
                image_table::Event::Error(error) => {
                    namui::event::send(Event::Error(error.clone()));
                }
            }
        } else if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::RequestUploadImages => {
                    let project_id = self.project_id;
                    spawn_local(async move {
                        let result = upload_images(project_id).await;
                        match result {
                            Ok(_) => namui::event::send(InternalEvent::UploadImageFinished),
                            Err(error) => namui::event::send(Event::Error(error.to_string())),
                        }
                    });
                }
                InternalEvent::UploadImageFinished => {
                    self.image_table.request_reload_images();
                }
            }
        }
        self.image_table.update(event);
    }
}

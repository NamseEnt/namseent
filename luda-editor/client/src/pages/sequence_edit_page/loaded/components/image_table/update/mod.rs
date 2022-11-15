use super::*;

impl ImageTable {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::LoadImages(images) => {
                    self.images = images.clone();
                }
            }
        }
        self.list_view.update(event);
    }
}

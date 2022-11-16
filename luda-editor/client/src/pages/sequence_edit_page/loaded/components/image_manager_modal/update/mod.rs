use super::*;

impl ImageManagerModal {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<image_table::Event>() {
            match event {
                image_table::Event::Error(error) => {
                    namui::event::send(Event::Error(error.clone()));
                }
            }
        }
        self.image_table.update(event);
    }
}

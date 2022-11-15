use super::*;

impl ImageManagerModal {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.image_table.update(event);
    }
}

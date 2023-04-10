use super::*;

impl CharacterPicker {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::ImagesLoaded(images) => {
                self.pose_files = images.clone();
            }
        });
    }
}

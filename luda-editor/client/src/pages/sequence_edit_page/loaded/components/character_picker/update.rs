use super::*;

impl CharacterPicker {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::ImagesLoaded(images) => {
                self.pose_files = images.clone();
            }
            InternalEvent::OpenPoseNameTooltip {
                global_xy,
                pose_name,
            } => {
                self.pose_name_tooltip = Some(PoseNameTooltip {
                    global_xy: *global_xy,
                    pose_name: pose_name.clone(),
                });
            }
            InternalEvent::ClosePoseNameTooltip => {
                self.pose_name_tooltip = None;
            }
        });
    }
}

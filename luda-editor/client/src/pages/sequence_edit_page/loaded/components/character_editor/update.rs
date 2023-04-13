use super::*;

impl CharacterEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| {
            match event {
                InternalEvent::OpenVariantNameTooltip {
                    global_xy,
                    pose_name,
                } => {
                    self.variant_name_tooltip = Some(VariantNameTooltip {
                        global_xy: *global_xy,
                        pose_name: pose_name.clone(),
                    });
                }
                InternalEvent::CloseVariantNameTooltip => {
                    self.variant_name_tooltip = None;
                }
            };
        });
    }
}

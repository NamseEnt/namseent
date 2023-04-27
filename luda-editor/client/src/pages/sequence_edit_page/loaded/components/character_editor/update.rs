use super::*;

impl CharacterEditor {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| {
            match event {
                InternalEvent::OpenTooltip { global_xy, text } => {
                    self.tooltip = Some(Tooltip {
                        global_xy: *global_xy,
                        text: text.clone(),
                    });
                }
                InternalEvent::CloseTooltip => {
                    self.tooltip = None;
                }
                InternalEvent::CgChangeButtonClicked => {
                    self.edit_target = EditTarget::ExistingCharacter
                }
                InternalEvent::CgFileLoadStateChanged(cg_file_load_state) => {
                    self.cg_file_load_state = cg_file_load_state.clone();
                }
            };
        });
        self.scroll_view.update(event);
    }
}

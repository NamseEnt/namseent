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
            };
        });
        self.scroll_view.update(event);
    }
}

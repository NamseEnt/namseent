use super::*;

impl ScreenEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.wysiwyg_editor.update(event);
    }
}

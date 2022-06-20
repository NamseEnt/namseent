use super::*;
use namui::animation::{Animate, Layer};

impl WysiwygWindow {
    pub(crate) fn render_layer(&self, layer: &Layer) -> namui::RenderingTree {
        layer.image.render(self.playback_time)
    }
}

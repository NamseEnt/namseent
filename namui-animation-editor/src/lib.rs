use namui::prelude::*;
use std::sync::Arc;
mod events;
pub use events::Event;
mod layer_list_window;

pub struct AnimationEditor {
    layer_list_window: layer_list_window::LayerListWindow,
}

pub struct Props<'a> {
    pub wh: Wh<types::PixelSize>,
    pub layers: &'a [Arc<namui::animation::Layer>],
}

impl AnimationEditor {
    pub fn new() -> Self {
        Self {
            layer_list_window: layer_list_window::LayerListWindow::new(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.layer_list_window.update(event);
    }
    pub fn render(&self, props: &Props) -> namui::RenderingTree {
        self.layer_list_window.render(&layer_list_window::Props {
            layers: props.layers,
            wh: Wh {
                width: props.wh.width,
                height: props.wh.height,
            },
        })
    }
}

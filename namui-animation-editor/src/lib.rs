use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::Arc;
mod events;
pub use events::Event;
mod layer_list_window;
mod property_window;

pub struct AnimationEditor {
    layer_list_window: layer_list_window::LayerListWindow,
    property_window: property_window::PropertyWindow,
}

pub struct Props<'a> {
    pub wh: Wh<types::PixelSize>,
    pub layers: &'a [Arc<namui::animation::Layer>],
}

impl AnimationEditor {
    pub fn new() -> Self {
        Self {
            layer_list_window: layer_list_window::LayerListWindow::new(),
            property_window: property_window::PropertyWindow::new(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.layer_list_window.update(event);
    }
    pub fn render(&self, props: &Props) -> namui::RenderingTree {
        horizontal![
            ratio!(
                1.0,
                vertical![
                    ratio!(
                        1.0,
                        &self.layer_list_window,
                        layer_list_window::Props {
                            layers: props.layers,
                        }
                    ),
                    ratio!(2.0, |wh| { RenderingTree::Empty }),
                    ratio!(
                        2.0,
                        &self.property_window,
                        property_window::Props {
                            layer: Some(&props.layers[0]), // TODO
                        }
                    ),
                ]
            ),
            ratio!(2.0, |wh| { RenderingTree::Empty })
        ](Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
}

pub(crate) fn adjust_font_size(height: f32) -> i16 {
    // 0, 4, 8, 16, 20, ...
    let mut font_size = (height * 0.7) as i16;
    font_size -= font_size % 4;
    font_size
}

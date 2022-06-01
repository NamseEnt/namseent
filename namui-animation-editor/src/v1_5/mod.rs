use namui::prelude::*;
use namui_prebuilt::*;
use std::sync::{Arc, RwLock};
mod events;
pub use events::Event;
mod graph_window;
mod layer_list_window;

pub struct AnimationEditor {
    animation: Arc<RwLock<animation::Animation>>,
    layer_list_window: layer_list_window::LayerListWindow,
    graph_window: graph_window::GraphWindow,
}

pub struct Props {
    pub wh: Wh<types::PixelSize>,
}

impl AnimationEditor {
    pub fn new(animation: Arc<RwLock<animation::Animation>>) -> Self {
        Self {
            animation,
            layer_list_window: layer_list_window::LayerListWindow::new(),
            graph_window: graph_window::GraphWindow::new(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<layer_list_window::Event>() {
            match event {
                layer_list_window::Event::LayerSelected(layer_id) => {
                    // TODO
                }
            }
        }

        self.layer_list_window.update(event);
        self.graph_window.update(event);
    }
    pub fn render(&self, props: &Props) -> namui::RenderingTree {
        let animation = self.animation.read().unwrap();
        horizontal![
            ratio!(
                1.0,
                vertical![
                    ratio!(
                        1.0,
                        &self.layer_list_window,
                        layer_list_window::Props {
                            layers: animation.layers.as_slice(),
                        }
                    ),
                    ratio!(2.0, |wh| { RenderingTree::Empty }),
                ]
            ),
            ratio!(
                2.0,
                vertical![
                    calculative!(
                        |wh| {
                            let height = wh.width / 1920.0 * 1080.0;
                            height
                        },
                        |wh| { RenderingTree::Empty }
                    ),
                    ratio!(1.0, &self.graph_window, graph_window::Props {})
                ]
            )
        ](Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
}

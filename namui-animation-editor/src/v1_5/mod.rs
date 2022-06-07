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
    selected_layer_id: Option<String>,
}

pub struct Props {
    pub wh: Wh<types::PixelSize>,
}

impl AnimationEditor {
    pub fn new(animation: Arc<RwLock<animation::Animation>>) -> Self {
        Self {
            layer_list_window: layer_list_window::LayerListWindow::new(),
            graph_window: graph_window::GraphWindow::new(animation.clone()),
            selected_layer_id: None,
            animation,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<layer_list_window::Event>() {
            match event {
                layer_list_window::Event::LayerSelected(layer_id) => {
                    self.selected_layer_id = Some(layer_id.clone());
                }
            }
        }

        self.layer_list_window.update(event);
        self.graph_window.update(event);
    }
    pub fn render(&self, props: &Props) -> namui::RenderingTree {
        let animation = self.animation.read().unwrap();
        let selected_layer = self
            .selected_layer_id
            .as_ref()
            .and_then(|layer_id| animation.layers.iter().find(|layer| layer.id.eq(layer_id)));
        horizontal![
            ratio!(
                1.0,
                vertical![
                    calculative!(|parent_wh| { parent_wh.width / 16.0 * 9.0 }, |wh| {
                        simple_rect(wh, Color::BLACK, 1.0, Color::TRANSPARENT)
                    }),
                    ratio!(
                        1.0,
                        &self.layer_list_window,
                        layer_list_window::Props {
                            layers: animation.layers.as_slice(),
                        }
                    ),
                    ratio!(1.0, |wh| {
                        simple_rect(wh, Color::BLACK, 1.0, Color::TRANSPARENT)
                    }),
                ]
            ),
            ratio!(
                4.0,
                vertical![ratio!(
                    1.0,
                    &self.graph_window,
                    graph_window::Props {
                        layer: selected_layer
                    }
                )]
            )
        ](Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
}

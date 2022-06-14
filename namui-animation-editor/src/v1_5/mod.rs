use namui::{prelude::*, types::Time};
use namui_prebuilt::*;
use std::sync::{Arc, RwLock};
mod events;
pub use events::Event;
mod graph_window;
mod image_select_window;
mod layer_list_window;
mod preview_window;
mod read_only_lock;
use read_only_lock::*;

pub struct AnimationEditor {
    animation: ReadOnlyLock<animation::Animation>,
    layer_list_window: layer_list_window::LayerListWindow,
    graph_window: graph_window::GraphWindow,
    preview_window: preview_window::PreviewWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    selected_layer_id: Option<String>,
}

pub struct Props {
    pub wh: Wh<types::PixelSize>,
}

impl AnimationEditor {
    pub fn new(animation: Arc<RwLock<animation::Animation>>) -> Self {
        let animation = ReadOnlyLock::new(animation);
        Self {
            layer_list_window: layer_list_window::LayerListWindow::new(),
            graph_window: graph_window::GraphWindow::new(animation.clone()),
            preview_window: preview_window::PreviewWindow::new(),
            image_select_window: image_select_window::ImageSelectWindow::new(),
            selected_layer_id: Some(animation.clone().read().layers.first().unwrap().id.clone()),
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
        if let Some(event) = event.downcast_ref::<image_select_window::Event>() {
            self.handle_image_select_window_event(event);
        }

        self.layer_list_window.update(event);
        self.graph_window.update(event);
        self.preview_window.update(event);
        self.image_select_window.update(event);
    }
    pub fn render(&self, props: &Props) -> namui::RenderingTree {
        let animation = self.animation.read();
        let selected_layer = self
            .selected_layer_id
            .as_ref()
            .and_then(|layer_id| animation.layers.iter().find(|layer| layer.id.eq(layer_id)));

        horizontal![
            ratio!(
                1.0,
                vertical![
                    calculative!(
                        |parent_wh| { parent_wh.width / 16.0 * 9.0 },
                        &self.preview_window,
                        preview_window::Props {
                            animation: &animation,
                            playback_time: Time::zero(),
                        }
                    ),
                    ratio!(
                        1.0,
                        &self.layer_list_window,
                        layer_list_window::Props {
                            layers: animation.layers.as_slice(),
                            selected_layer_id: self.selected_layer_id.clone(),
                        }
                    ),
                    ratio!(
                        1.0,
                        &self.image_select_window,
                        image_select_window::Props {
                            selected_layer_image_url: selected_layer
                                .and_then(|layer| layer.image.image_source_url.clone()),
                        }
                    ),
                ]
            ),
            ratio!(
                4.0,
                &self.graph_window,
                graph_window::Props {
                    layer: selected_layer
                }
            )
        ](Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }

    fn handle_image_select_window_event(&mut self, event: &image_select_window::Event) {
        match event {
            image_select_window::Event::ImageSelected(url) => {
                if self.selected_layer_id.is_none() {
                    return;
                }
                let layer_id = self.selected_layer_id.clone().unwrap();

                let animation = self.animation.read();
                let layer = animation.layers.iter().find(|layer| layer.id.eq(&layer_id));
                if layer.is_none() {
                    return;
                }
                let mut layer = layer.unwrap().clone();
                layer.image.image_source_url = Some(url.clone());
                namui::event::send(Event::UpdateLayer(Arc::new(layer)));
            }
        }
    }
}

use crate::image_select_window;
use crate::layer_list_window;
use namui::{prelude::*, types::Time};
use namui_prebuilt::{table::*, *};
mod timeline_window;
mod wysiwyg_window;

pub struct TimePointEditor {
    animation: crate::ReadOnlyLock<animation::Animation>,
    wysiwyg_window: wysiwyg_window::WysiwygWindow,
    timeline_window: timeline_window::TimelineWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    layer_list_window: layer_list_window::LayerListWindow,
    selected_layer_id: Option<String>,
}

pub struct Props {
    pub wh: Wh<f32>,
}

pub(crate) enum Event {}

impl TimePointEditor {
    pub fn new(animation: crate::ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            wysiwyg_window: wysiwyg_window::WysiwygWindow::new(animation.clone()),
            timeline_window: timeline_window::TimelineWindow::new(),
            image_select_window: image_select_window::ImageSelectWindow::new(),
            layer_list_window: layer_list_window::LayerListWindow::new(),
            animation,
            selected_layer_id: None,
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

        self.wysiwyg_window.update(event);
        self.timeline_window.update(event);
        self.image_select_window.update(event);
        self.layer_list_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = self.animation.read();
        let selected_layer = self
            .selected_layer_id
            .as_ref()
            .and_then(|layer_id| animation.layers.iter().find(|layer| layer.id.eq(layer_id)));

        vertical([
            ratio(
                8.0,
                horizontal([
                    ratio(
                        2.0,
                        vertical([
                            ratio(1.0, |wh| {
                                self.layer_list_window.render(
                                    wh,
                                    layer_list_window::Props {
                                        layers: &animation.layers,
                                        selected_layer_id: self.selected_layer_id.clone(),
                                    },
                                )
                            }),
                            ratio(1.0, |wh| {
                                self.image_select_window.render(
                                    wh,
                                    image_select_window::Props {
                                        selected_layer_image_url: selected_layer
                                            .and_then(|layer| layer.image.image_source_url.clone()),
                                    },
                                )
                            }),
                        ]),
                    ),
                    ratio(8.0, |wh| {
                        self.wysiwyg_window.render(wysiwyg_window::Props { wh })
                    }),
                ]),
            ),
            ratio(2.0, |wh| {
                self.timeline_window.render(timeline_window::Props {
                    wh,
                    layers: &animation.layers,
                    selected_layer_id: self.selected_layer_id.clone(),
                })
            }),
        ])(Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
}

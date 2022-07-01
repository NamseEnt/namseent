use namui::{prelude::*, types::*};
use namui_prebuilt::table::*;
mod graph_window;
use crate::*;
mod preview_window;

pub struct GraphEditor {
    graph_window: graph_window::GraphWindow,
    preview_window: preview_window::PreviewWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    playback_time: Time,
}

pub struct Props<'a> {
    pub wh: Wh<f32>,
    pub layer_list_window: &'a layer_list_window::LayerListWindow,
    pub animation: &'a Animation,
}

pub(crate) enum Event {
    SetPlaybackTime(Time),
}

impl GraphEditor {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            graph_window: graph_window::GraphWindow::new(animation_history.clone()),
            preview_window: preview_window::PreviewWindow::new(),
            image_select_window: image_select_window::ImageSelectWindow::new(
                animation_history.clone(),
            ),
            playback_time: Time::Ms(0.0),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::SetPlaybackTime(time) => {
                    self.playback_time = *time;
                }
            }
        }

        self.graph_window.update(event);
        self.preview_window.update(event);
        self.image_select_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let selected_layer =
            props
                .layer_list_window
                .selected_layer_id
                .as_ref()
                .and_then(|layer_id| {
                    props
                        .animation
                        .layers
                        .iter()
                        .find(|layer| layer.id.eq(layer_id))
                });

        horizontal([
            ratio(
                1.0,
                vertical([
                    calculative(
                        |parent_wh| parent_wh.width / 16.0 * 9.0,
                        |wh| {
                            self.preview_window.render(preview_window::Props {
                                wh,
                                animation: props.animation,
                                playback_time: self.playback_time,
                            })
                        },
                    ),
                    ratio(1.0, |wh| {
                        props.layer_list_window.render(layer_list_window::Props {
                            wh,
                            layers: props.animation.layers.as_slice(),
                        })
                    }),
                    ratio(1.0, |wh| {
                        self.image_select_window.render(image_select_window::Props {
                            wh,
                            selected_layer_image_url: selected_layer
                                .and_then(|layer| layer.image.image_source_url.clone()),
                            selected_layer_id: selected_layer.map(|layer| layer.id.clone()),
                        })
                    }),
                ]),
            ),
            ratio(4.0, |wh| {
                self.graph_window.render(graph_window::Props {
                    wh,
                    layer: selected_layer,
                    playback_time: self.playback_time,
                })
            }),
        ])(props.wh)
    }
}

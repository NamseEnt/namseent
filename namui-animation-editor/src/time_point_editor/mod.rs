use crate::{
    image_select_window, layer_list_window,
    types::{Act, AnimationHistory},
};
use namui::{animation::Animation, prelude::*};
use namui_prebuilt::table::*;
mod timeline_window;
mod wysiwyg_window;

pub(crate) struct TimePointEditor {
    animation_history: AnimationHistory,
    wysiwyg_window: wysiwyg_window::WysiwygWindow,
    timeline_window: timeline_window::TimelineWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    editing_target: Option<EditingTarget>,
}

pub(crate) struct Props<'a> {
    pub wh: Wh<f32>,
    pub animation: &'a Animation,
    pub layer_list_window: &'a layer_list_window::LayerListWindow,
}

impl TimePointEditor {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            wysiwyg_window: wysiwyg_window::WysiwygWindow::new(animation_history.clone()),
            timeline_window: timeline_window::TimelineWindow::new(animation_history.clone()),
            image_select_window: image_select_window::ImageSelectWindow::new(
                animation_history.clone(),
            ),
            editing_target: None,
            animation_history,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<layer_list_window::Event>() {
            match event {
                layer_list_window::Event::LayerSelected(layer_id) => {
                    self.editing_target = Some(EditingTarget::PlaybackTime {
                        layer_id: layer_id.clone(),
                    });
                }
                _ => {}
            }
        }

        self.wysiwyg_window.update(event);
        self.timeline_window.update(event);
        self.image_select_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = props.animation;
        let selected_layer_id = self.get_selected_layer_id();
        let selected_layer = selected_layer_id
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
                                props.layer_list_window.render(layer_list_window::Props {
                                    wh,
                                    layers: &animation.layers,
                                })
                            }),
                            ratio(1.0, |wh| {
                                self.image_select_window.render(image_select_window::Props {
                                    wh,
                                    selected_layer_image_url: selected_layer
                                        .and_then(|layer| layer.image.image_source_url.clone()),
                                    selected_layer_id: selected_layer_id.clone(),
                                })
                            }),
                        ]),
                    ),
                    ratio(8.0, |wh| {
                        self.wysiwyg_window.render(wysiwyg_window::Props {
                            wh,
                            playback_time: self.timeline_window.get_playback_time(),
                            animation,
                            selected_layer_id: selected_layer_id.clone(),
                        })
                    }),
                ]),
            ),
            ratio(2.0, |wh| {
                self.timeline_window.render(timeline_window::Props {
                    wh,
                    layers: &animation.layers,
                    selected_layer_id: selected_layer_id.clone(),
                })
            }),
        ])(Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
    fn get_selected_layer_id(&self) -> Option<String> {
        self.editing_target
            .as_ref()
            .and_then(|editing_target| match editing_target {
                EditingTarget::PlaybackTime { layer_id } => Some(layer_id.clone()),
            })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EditingTarget {
    PlaybackTime { layer_id: String },
}

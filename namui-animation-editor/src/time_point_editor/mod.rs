use crate::{
    image_select_window, layer_list_window,
    types::{Act, AnimationHistory},
};
use namui::{animation::Animation, prelude::*};
use namui_prebuilt::table::*;
mod timeline_window;
mod wysiwyg_window;

pub struct TimePointEditor {
    animation_history: AnimationHistory,
    wysiwyg_window: wysiwyg_window::WysiwygWindow,
    timeline_window: timeline_window::TimelineWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    layer_list_window: layer_list_window::LayerListWindow,
    editing_target: Option<EditingTarget>,
}

pub struct Props<'a> {
    pub wh: Wh<f32>,
    pub animation: &'a Animation,
}

impl TimePointEditor {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            wysiwyg_window: wysiwyg_window::WysiwygWindow::new(animation_history.clone()),
            timeline_window: timeline_window::TimelineWindow::new(animation_history.clone()),
            image_select_window: image_select_window::ImageSelectWindow::new(),
            layer_list_window: layer_list_window::LayerListWindow::new(animation_history.clone()),
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
        } else if let Some(event) = event.downcast_ref::<image_select_window::Event>() {
            match event {
                image_select_window::Event::ImageSelected(url) => {
                    if let Some(layer_id) = self.get_selected_layer_id() {
                        struct SelectImageAction {
                            url: Url,
                            layer_id: String,
                        }
                        impl Act<Animation> for SelectImageAction {
                            fn act(
                                &self,
                                state: &Animation,
                            ) -> Result<Animation, Box<dyn std::error::Error>>
                            {
                                let mut animation = state.clone();

                                if let Some(layer) = animation
                                    .layers
                                    .iter_mut()
                                    .find(|layer| layer.id.eq(&self.layer_id))
                                {
                                    layer.image.image_source_url = Some(self.url.clone());
                                    Ok(animation)
                                } else {
                                    Err("layer not found".into())
                                }
                            }
                        }

                        if let Some(action_ticket) =
                            self.animation_history.try_set_action(SelectImageAction {
                                url: url.clone(),
                                layer_id: layer_id.clone(),
                            })
                        {
                            self.animation_history.act(action_ticket).unwrap();
                        }
                    }
                }
            }
        }

        self.wysiwyg_window.update(event);
        self.timeline_window.update(event);
        self.image_select_window.update(event);
        self.layer_list_window.update(event);
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
                                self.layer_list_window.render(
                                    wh,
                                    layer_list_window::Props {
                                        layers: &animation.layers,
                                        selected_layer_id: selected_layer_id.clone(),
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

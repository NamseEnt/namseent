use crate::{image_select_window, layer_list_window, types::AnimationHistory};
use namui::{animation::Animation, prelude::*, types::Time};
use namui_prebuilt::{table::*, *};
mod timeline_window;
mod wysiwyg_window;

pub struct TimePointEditor {
    wysiwyg_window: wysiwyg_window::WysiwygWindow,
    timeline_window: timeline_window::TimelineWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    layer_list_window: layer_list_window::LayerListWindow,
    playback_time: Time,
    editing_target: Option<EditingTarget>,
}

pub struct Props<'a> {
    pub wh: Wh<f32>,
    pub animation: &'a Animation,
}

pub(crate) enum Event {
    UpdatePlaybackTime(Time),
    SelectKeyframe { layer_id: String, time: Time },
}

impl TimePointEditor {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            wysiwyg_window: wysiwyg_window::WysiwygWindow::new(animation_history.clone()),
            timeline_window: timeline_window::TimelineWindow::new(animation_history.clone()),
            image_select_window: image_select_window::ImageSelectWindow::new(),
            layer_list_window: layer_list_window::LayerListWindow::new(animation_history.clone()),
            playback_time: Time::zero(),
            editing_target: None,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<layer_list_window::Event>() {
            match event {
                layer_list_window::Event::LayerSelected(layer_id) => {
                    self.editing_target = Some(EditingTarget::PlaybackTime {
                        layer_id: layer_id.clone(),
                    });
                    self.timeline_window.selected_layer_id = Some(layer_id.clone());
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::UpdatePlaybackTime(time) => {
                    self.playback_time = *time;
                }
                Event::SelectKeyframe { layer_id, time } => {
                    self.editing_target = Some(EditingTarget::Time {
                        layer_id: layer_id.clone(),
                        time: *time,
                    });
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
                            playback_time: self.playback_time,
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
                    playback_time: self.playback_time,
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
                EditingTarget::Time { layer_id, .. } => Some(layer_id.clone()),
            })
    }
}

enum EditingTarget {
    PlaybackTime { layer_id: String },
    Time { layer_id: String, time: Time },
}

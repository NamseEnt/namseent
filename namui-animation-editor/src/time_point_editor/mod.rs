use crate::{image_select_window, layer_list_window, types::AnimationHistory};
use namui::{animation::Animation, prelude::*};
use namui_prebuilt::table::*;
mod line_edit_window;
mod timeline_window;
mod wysiwyg_window;

pub(crate) struct TimePointEditor {
    wysiwyg_window: wysiwyg_window::WysiwygWindow,
    timeline_window: timeline_window::TimelineWindow,
    image_select_window: image_select_window::ImageSelectWindow,
    line_edit_window: line_edit_window::LineEditWindow,
    editing_target: Option<EditingTarget>,
    selected_layer_id: Option<String>,
}

enum Event {
    ChangEditingTarget(Option<EditingTarget>),
}

pub(crate) struct Props<'a> {
    pub wh: Wh<Px>,
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
            line_edit_window: line_edit_window::LineEditWindow::new(animation_history.clone()),
            editing_target: None,
            selected_layer_id: None,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::ChangEditingTarget(editing_target) => {
                    self.editing_target = editing_target.clone();
                }
            }
        } else if let Some(event) = event.downcast_ref::<layer_list_window::Event>() {
            match event {
                layer_list_window::Event::LayerSelected(layer_id) => {
                    self.selected_layer_id = Some(layer_id.clone());
                }
                _ => {}
            }
        }

        self.wysiwyg_window.update(event);
        self.timeline_window.update(event);
        self.image_select_window.update(event);
        self.line_edit_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = props.animation;
        let selected_layer_id = &self.selected_layer_id;
        let selected_layer = selected_layer_id
            .as_ref()
            .and_then(|layer_id| animation.layers.iter().find(|layer| layer.id.eq(layer_id)));

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
            ratio(
                6.0,
                vertical([
                    ratio(8.0, |wh| {
                        self.wysiwyg_window.render(wysiwyg_window::Props {
                            wh,
                            playback_time: self.timeline_window.get_playback_time(),
                            animation,
                            selected_layer_id: selected_layer_id.clone(),
                            editing_target: self.editing_target.clone(),
                        })
                    }),
                    ratio(2.0, |wh| {
                        self.timeline_window.render(timeline_window::Props {
                            wh,
                            layers: &animation.layers,
                            selected_layer_id: selected_layer_id.clone(),
                            editing_target: self.editing_target.clone(),
                        })
                    }),
                ]),
            ),
            ratio(2.0, |wh| {
                self.line_edit_window.render(line_edit_window::Props {
                    wh,
                    editing_target: self.editing_target.clone(),
                    selected_layer: selected_layer,
                })
            }),
        ])(Wh {
            width: props.wh.width.into(),
            height: props.wh.height.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EditingTarget {
    Keyframe { point_id: String, layer_id: String },
    Line { point_id: String, layer_id: String },
}

use crate::app::{
    editor::{events::EditorEvent, job::Job},
    types::*,
};
use namui::prelude::*;
use std::sync::{Arc, RwLock};

pub struct CameraClipEditor {
    animation_editor: namui_animation_editor::AnimationEditor,
    animation: Arc<RwLock<animation::Animation>>,
    clip_id: String,
}

pub struct CameraClipEditorProps<'a> {
    pub xywh: XywhRect<f32>,
    pub job: &'a Option<Job>,
    pub clip: &'a CameraClip,
}

impl CameraClipEditor {
    pub fn new(clip: Arc<CameraClip>) -> Self {
        let animation = Arc::new(RwLock::new(clip.animation.clone()));
        Self {
            animation_editor: namui_animation_editor::AnimationEditor::new(animation.clone()),
            animation,
            clip_id: clip.id.clone(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<namui_animation_editor::Event>() {
            match event {
                namui_animation_editor::Event::UpdateLayer(layer) => {
                    let layer = layer.clone();
                    namui::event::send(EditorEvent::CameraClipUpdateEvent {
                        clip_id: self.clip_id.clone(),
                        update: Arc::new(move |clip| {
                            let mut clip = clip.clone();
                            clip.animation
                                .layers
                                .iter_mut()
                                .find(|layer| layer.id == layer.id)
                                .unwrap()
                                .clone_from(&layer);
                            clip
                        }),
                    })
                }
                namui_animation_editor::Event::Error(error) => {
                    panic!("{:?}", error);
                }
                namui_animation_editor::Event::AddLayerButtonClicked => {
                    namui::event::send(EditorEvent::CameraClipUpdateEvent {
                        clip_id: self.clip_id.clone(),
                        update: Arc::new(|clip| {
                            let mut clip = clip.clone();
                            clip.animation.layers.push(namui::animation::Layer {
                                id: namui::nanoid(),
                                name: "New Layer".to_string(),
                                image: namui::animation::AnimatableImage::new(),
                            });
                            clip
                        }),
                    })
                }
                _ => {}
            }
        }
        self.animation_editor.update(event);
    }
    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        {
            (*self.animation.write().unwrap()) = props.clip.animation.clone();
        }
        namui::translate(
            props.xywh.x,
            props.xywh.y,
            namui::clip(
                namui::PathBuilder::new().add_rect(&namui::LtrbRect {
                    left: 0.0,
                    top: 0.0,
                    right: props.xywh.width,
                    bottom: props.xywh.height,
                }),
                namui::ClipOp::Intersect,
                namui::render![self
                    .animation_editor
                    .render(&namui_animation_editor::Props {
                        wh: Wh {
                            width: props.xywh.width.into(),
                            height: props.xywh.height.into(),
                        },
                    }),],
            ),
        )
    }
}

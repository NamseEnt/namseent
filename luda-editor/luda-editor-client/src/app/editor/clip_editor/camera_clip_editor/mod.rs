use crate::app::{
    editor::{events::EditorEvent, job::Job},
    types::*,
};
use namui::prelude::*;
use std::sync::Arc;

pub struct CameraClipEditor {
    clip_id: String,
    animation_editor: namui_animation_editor::AnimationEditor,
}

pub struct CameraClipEditorProps<'a> {
    pub camera_clip: &'a CameraClip,
    pub xywh: XywhRect<f32>,
    pub job: &'a Option<Job>,
}

impl CameraClipEditor {
    pub fn new(clip: &CameraClip) -> Self {
        Self {
            animation_editor: namui_animation_editor::AnimationEditor::new(),
            clip_id: clip.id.clone(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.animation_editor.update(event);
    }
    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        let mut layers = vec![];
        for i in 0..100 {
            layers.push(mock_layer(i));
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
                        layers: &layers,
                        wh: Wh {
                            width: props.xywh.width.into(),
                            height: props.xywh.height.into(),
                        },
                    }),],
            ),
        )
    }
}

fn mock_layer(index: i32) -> Arc<namui::animation::Layer> {
    Arc::new(namui::animation::Layer {
        name: index.to_string(),
        image: namui::animation::AnimatableImage {
            image_source_url: "".to_string(),
            x: namui::animation::KeyframeGraph::new(namui::animation::KeyframePoint {
                time: namui::types::Time::from_ms(5.0),
                value: 0.0.into(),
            }),
            y: namui::animation::KeyframeGraph::new(namui::animation::KeyframePoint {
                time: namui::types::Time::from_ms(5.0),
                value: 0.0.into(),
            }),
            width: namui::animation::KeyframeGraph::new(namui::animation::KeyframePoint {
                time: namui::types::Time::from_ms(5.0),
                value: 0.0.into(),
            }),
            height: namui::animation::KeyframeGraph::new(namui::animation::KeyframePoint {
                time: namui::types::Time::from_ms(5.0),
                value: 0.0.into(),
            }),
            opacity: namui::animation::KeyframeGraph::new(namui::animation::KeyframePoint {
                time: namui::types::Time::from_ms(5.0),
                value: 0.0.into(),
            }),
            rotation_angle: namui::animation::KeyframeGraph::new(namui::animation::KeyframePoint {
                time: namui::types::Time::from_ms(5.0),
                value: 0.0.into(),
            }),
        },
    })
}

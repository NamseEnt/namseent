use self::camera_clip_editor::{CameraClipEditor, CameraClipEditorProps};
use super::job::Job;
use crate::app::types::*;
pub mod camera_clip_editor;
use namui::prelude::*;

pub enum ClipEditor {
    Camera(CameraClipEditor),
    Subtitle,
}

pub struct ClipEditorProps<'a> {
    pub xywh: XywhRect<f32>,
    pub job: &'a Option<Job>,
}

impl ClipEditor {
    pub fn new(clip: Clip) -> Self {
        match clip {
            Clip::Camera(clip) => ClipEditor::Camera(CameraClipEditor::new(clip)),
            Clip::Subtitle(clip) => ClipEditor::Subtitle,
            _ => todo!(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        match self {
            ClipEditor::Camera(camera_clip_editor) => {
                camera_clip_editor.update(event);
            }
            ClipEditor::Subtitle => {}
        }
    }

    pub fn render(&self, props: &ClipEditorProps) -> RenderingTree {
        match &self {
            ClipEditor::Camera(camera_clip_editor) => {
                camera_clip_editor.render(&CameraClipEditorProps {
                    xywh: props.xywh,
                    job: &props.job,
                })
            }
            ClipEditor::Subtitle => RenderingTree::Empty,
        }
    }
}

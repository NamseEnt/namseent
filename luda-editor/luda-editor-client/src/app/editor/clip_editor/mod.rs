use std::collections::BTreeSet;

use self::camera_clip_editor::{
    image_browser::ImageBrowserFile, CameraClipEditor, CameraClipEditorProps,
};
use super::job::Job;
use crate::app::types::*;
pub mod camera_clip_editor;
use namui::prelude::*;

pub enum ClipEditor {
    Camera(CameraClipEditor),
    Subtitle,
}

pub struct ClipEditorProps<'a> {
    pub clip: Clip,
    pub rect: Rect<Px>,
    pub character_image_files: &'a BTreeSet<ImageBrowserFile>,
    pub background_image_files: &'a BTreeSet<ImageBrowserFile>,
    pub job: &'a Option<Job>,
}

impl ClipEditor {
    pub fn new(clip: &Clip) -> Self {
        match clip {
            Clip::Camera(clip) => ClipEditor::Camera(CameraClipEditor::new(&clip)),
            Clip::Subtitle(_) => ClipEditor::Subtitle,
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
            ClipEditor::Camera(camera_clip_editor) => match &props.clip {
                Clip::Camera(camera_clip) => camera_clip_editor.render(&CameraClipEditorProps {
                    camera_clip: &camera_clip,
                    rect: props.rect,
                    character_image_files: &props.character_image_files,
                    background_image_files: &props.background_image_files,
                    job: &props.job,
                }),
                _ => unreachable!("clip should be camera clip but received {:?}", props.clip),
            },
            ClipEditor::Subtitle => RenderingTree::Empty,
        }
    }
}

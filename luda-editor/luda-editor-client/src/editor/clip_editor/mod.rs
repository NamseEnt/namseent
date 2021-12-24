use self::camera_clip_editor::{CameraClipEditor, CameraClipEditorProps};
use super::types::*;
mod camera_clip_editor;
use namui::prelude::*;

pub struct ClipEditor {
    camera_clip_editor: CameraClipEditor,
}

impl ClipEditor {
    pub fn new() -> Self {
        Self {
            camera_clip_editor: CameraClipEditor::new(),
        }
    }
}

pub struct ClipEditorProps<'a> {
    pub selected_clip: Option<Clip<'a>>,
    pub xywh: XywhRect<f32>,
    pub image_filename_objects: &'a Vec<ImageFilenameObject>,
}

impl ClipEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.camera_clip_editor.update(event);
    }

    pub fn render(&self, props: &ClipEditorProps) -> RenderingTree {
        match &props.selected_clip {
            Some(clip) => match clip {
                Clip::Camera(camera_clip) => {
                    self.camera_clip_editor.render(&CameraClipEditorProps {
                        camera_clip: &camera_clip,
                        xywh: props.xywh,
                        image_filename_objects: &props.image_filename_objects,
                    })
                }
                Clip::Subtitle(_) => todo!(),
            },
            None => RenderingTree::Empty,
        }
    }
}

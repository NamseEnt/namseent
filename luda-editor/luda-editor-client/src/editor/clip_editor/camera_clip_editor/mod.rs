use std::rc::Rc;
use crate::editor::types::*;
use namui::prelude::*;

use self::image_browser::ImageBrowser;
mod image_browser;

pub struct CameraClipEditor {
    image_browser: ImageBrowser,
}

impl CameraClipEditor {
    pub fn new() -> Self {
        Self {
            image_browser: ImageBrowser::new(),
        }
    }
}

pub struct CameraClipEditorProps {
    pub camera_clip: Rc<CameraClip>,
    pub xywh: XywhRect<f32>,
}

impl Entity for CameraClipEditor {
    type Props = CameraClipEditorProps;

    fn update(&mut self, event: &dyn std::any::Any) {}

    fn render(&self, props: &Self::Props) -> RenderingTree {
        todo!()
    }
}

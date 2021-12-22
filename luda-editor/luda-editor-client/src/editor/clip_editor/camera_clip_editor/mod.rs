use crate::editor::types::*;
use namui::prelude::*;
use std::rc::Rc;

use self::image_browser::{ImageBrowser, ImageBrowserProps};
mod image_browser;

pub struct CameraClipEditor {
    image_browser: ImageBrowser,
}

impl CameraClipEditor {
    pub fn new(socket: &luda_editor_rpc::Socket) -> Self {
        Self {
            image_browser: ImageBrowser::new(socket),
        }
    }
}

pub struct CameraClipEditorProps<'a> {
    pub camera_clip: &'a CameraClip,
    pub xywh: XywhRect<f32>,
}

impl CameraClipEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.image_browser
            .update(event);
    }

    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        self.image_browser
            .render(&ImageBrowserProps {})
    }
}

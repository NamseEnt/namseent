pub use self::{
    image_browser::{ImageBrowser, ImageBrowserProps},
    wysiwyg_editor::{WysiwygEditor, WysiwygEditorProps},
};
use crate::editor::{job::Job, types::*};
use namui::prelude::*;
mod image_browser;
pub mod wysiwyg_editor;

pub struct CameraClipEditor {
    image_browser: ImageBrowser,
    wysiwyg_editor: WysiwygEditor,
}

impl CameraClipEditor {
    pub fn new() -> Self {
        Self {
            image_browser: ImageBrowser::new(),
            wysiwyg_editor: WysiwygEditor::new(),
        }
    }
}

pub struct CameraClipEditorProps<'a> {
    pub camera_clip: &'a CameraClip,
    pub xywh: XywhRect<f32>,
    pub image_filename_objects: &'a Vec<ImageFilenameObject>,
    pub job: &'a Option<Job>,
}

impl CameraClipEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.image_browser.update(event);
    }
    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        namui::translate(
            props.xywh.x,
            props.xywh.y,
            // namui::clip(
            //     namui::Path::new().add_rect(namui::LtrbRect {
            //         left: 0.0,
            //         top: 0.0,
            //         right: props
            //             .xywh
            //             .width,
            //         bottom: props
            //             .xywh
            //             .height,
            //     }),
            //     namui::ClipOp::Intersect,
            namui::render![
                namui::rect(namui::RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: props.xywh.width,
                    height: props.xywh.height,
                    style: namui::RectStyle {
                        stroke: Some(namui::RectStroke {
                            color: namui::Color::BLACK,
                            width: 1.0,
                            border_position: namui::BorderPosition::Inside,
                        }),
                        fill: Some(namui::RectFill {
                            color: namui::Color::WHITE,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                self.image_browser.render(&ImageBrowserProps {
                    width: props.xywh.width / 2.0,
                    height: props.xywh.height,
                    image_filename_objects: props.image_filename_objects,
                }),
                self.wysiwyg_editor.render(&WysiwygEditorProps {
                    xywh: XywhRect {
                        x: props.xywh.width / 2.0,
                        y: 0.0,
                        width: props.xywh.width / 2.0,
                        height: props.xywh.width / 2.0 / (1920.0 / 1080.0),
                    },
                    camera_angle: &props.camera_clip.camera_angle,
                    image_filename_objects: props.image_filename_objects,
                    job: &props.job,
                }),
                // WysiwygEditor(state),
                // Preview(state),
            ],
            // ),
        )
    }
}

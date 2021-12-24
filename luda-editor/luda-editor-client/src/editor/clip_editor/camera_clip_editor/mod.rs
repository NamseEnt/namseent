use self::image_browser::{ImageBrowser, ImageBrowserProps};
use crate::editor::types::*;
use namui::prelude::*;
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
                            color: namui::Color::RED,
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
                    width: props.xywh.width,
                    height: props.xywh.height,
                }),
                // WysiwygEditor(state),
                // Preview(state),
            ],
            // ),
        )
    }
}

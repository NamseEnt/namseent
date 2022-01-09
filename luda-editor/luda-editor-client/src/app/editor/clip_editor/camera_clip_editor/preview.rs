use crate::app::types::*;
use namui::prelude::*;

pub struct Preview {}

impl Preview {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct PreviewProps<'a> {
    pub xywh: &'a XywhRect<f32>,
    pub camera_angle: &'a CameraAngle,
    pub camera_angle_image_loader: &'a dyn CameraAngleImageLoader,
}

impl Preview {
    pub fn update(&mut self, event: &dyn std::any::Any) {}

    pub fn render(&self, props: &PreviewProps) -> RenderingTree {
        translate(
            props.xywh.x,
            props.xywh.y,
            render![
                rect(RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: props.xywh.width,
                    height: props.xywh.height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::BLACK,
                            width: 1.0,
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                props
                    .camera_angle
                    .render(&props.xywh.wh(), props.camera_angle_image_loader),
            ],
        )
    }
}

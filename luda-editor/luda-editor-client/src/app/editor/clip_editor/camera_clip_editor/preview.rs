use crate::app::types::*;
use namui::prelude::*;

pub struct Preview {}

impl Preview {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct PreviewProps<'a> {
    pub rect: Rect<Px>,
    pub camera_angle: &'a CameraAngle,
    pub camera_angle_image_loader: &'a dyn CameraAngleImageLoader,
}

impl Preview {
    pub fn update(&mut self, _event: &dyn std::any::Any) {}

    pub fn render(&self, props: &PreviewProps) -> RenderingTree {
        translate(
            props.rect.x(),
            props.rect.y(),
            render([
                rect(RectParam {
                    rect: Rect::Xywh {
                        x: px(0.0),
                        y: px(0.0),
                        width: props.rect.width(),
                        height: props.rect.height(),
                    },
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::BLACK,
                            width: px(1.0),
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                props
                    .camera_angle
                    .render(props.rect.wh(), props.camera_angle_image_loader),
            ]),
        )
    }
}

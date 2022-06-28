use std::sync::Arc;

use super::*;
use crate::app::types::*;
use namui::prelude::*;

pub struct BackgroundWysiwygEditor {}

pub struct BackgroundWysiwygEditorProps<'a> {
    pub xywh: XywhRect<f32>,
    pub camera_angle: &'a CameraAngle,
}

impl BackgroundWysiwygEditor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {}
    pub fn render(&self, props: &BackgroundWysiwygEditorProps) -> RenderingTree {
        let container_size = Wh {
            width: props.xywh.width,
            height: props.xywh.height,
        };

        let image_loader = LudaEditorServerCameraAngleImageLoader {};

        let background = props.camera_angle.background.as_ref();
        if background.is_none() {
            return RenderingTree::Empty;
        }
        let background = background.unwrap();

        let image_source = image_loader.get_background_image_source(background);
        let image = match image_source {
            ImageSource::Url(url) => namui::image::try_load(&url),
            ImageSource::Image(image) => Some(image),
        };
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let image_size = image.size();
        let drawn_image_wh = fit_wh_in_container(&image_size, &container_size);
        let drawn_iamge_xywh = XywhRect {
            x: 0.0,
            y: 0.0,
            width: drawn_image_wh.width,
            height: drawn_image_wh.height,
        };
        let inner_xywh = get_inner_xywh(
            &background.source_01_circumscribed,
            &Wh {
                width: 1920.0,
                height: 1080.0,
            },
            &drawn_image_wh,
        );

        translate(
            props.xywh.x,
            props.xywh.y,
            render![
                render_outer_image(image.clone(), &drawn_iamge_xywh, &inner_xywh.into_ltrb()),
                render_inner_image(
                    image.clone(),
                    &drawn_iamge_xywh,
                    &inner_xywh.into_ltrb(),
                    &drawn_iamge_xywh.wh(),
                    self.get_id()
                ),
                rect(RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: drawn_image_wh.width,
                    height: drawn_image_wh.height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::BLACK,
                            width: 2.0,
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                Resizer::new(self.get_id()).render(&ResizerProps {
                    source_rect: &inner_xywh,
                    container_size: &drawn_image_wh,
                }),
            ],
        )
    }
    pub fn get_id(&self) -> &'static str {
        "background"
    }
}

pub fn fit_wh_in_container(image_size: &Wh<f32>, container_size: &Wh<f32>) -> Wh<f32> {
    if image_size.width / image_size.height > container_size.width / container_size.height {
        Wh {
            width: container_size.width,
            height: container_size.width * image_size.height / image_size.width,
        }
    } else {
        Wh {
            width: container_size.height * image_size.width / image_size.height,
            height: container_size.height,
        }
    }
}

pub fn get_inner_xywh(
    circumscribed_01: &Circumscribed,
    image_size: &Wh<f32>,
    container_size: &Wh<f32>,
) -> XywhRect<f32> {
    let length_of_result_rect = circumscribed_01.radius * 2.0 * container_size.length();

    let image_size_length = image_size.length();
    let image_width_length_ratio = image_size.width / image_size_length;
    let image_height_length_ratio = image_size.height / image_size_length;

    let image_width_length = image_width_length_ratio * length_of_result_rect;
    let image_height_length = image_height_length_ratio * length_of_result_rect;

    XywhRect {
        x: container_size.width * circumscribed_01.center.x - image_width_length / 2.0,
        y: container_size.height * circumscribed_01.center.y - image_height_length / 2.0,
        width: image_width_length,
        height: image_height_length,
    }
}

pub fn render_source_image(
    image: Arc<Image>,
    paint_builder: Option<PaintBuilder>,
    source_rect: &XywhRect<f32>,
) -> RenderingTree {
    namui::image(ImageParam {
        xywh: *source_rect,
        style: ImageStyle {
            fit: ImageFit::Fill,
            paint_builder,
        },
        source: ImageSource::Image(image),
    })
}

fn render_outer_image(
    image: Arc<Image>,
    image_drawn_rect: &XywhRect<f32>,
    inner_rect: &LtrbRect,
) -> RenderingTree {
    let outside_image_paint = namui::PaintBuilder::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color_filter(&Color::grayscale_f01(0.5), &namui::BlendMode::Multiply);

    namui::clip(
        namui::PathBuilder::new().add_rect(inner_rect),
        namui::ClipOp::Difference,
        namui::render![render_source_image(
            image,
            Some(outside_image_paint),
            image_drawn_rect
        )],
    )
}

fn render_inner_image(
    image: Arc<Image>,
    source_rect: &XywhRect<f32>,
    dest_rect: &LtrbRect,
    container_size: &Wh<f32>,
    id: &'static str,
) -> RenderingTree {
    let container_size = container_size.clone();

    namui::clip(
        namui::PathBuilder::new().add_rect(dest_rect),
        namui::ClipOp::Intersect,
        render_source_image(image, None, &source_rect)
            .attach_event(|builder| {
                let target_id = id.to_string();
                builder.on_mouse_down(move |event| {
                    namui::event::send(WysiwygEvent::InnerImageMouseDownEvent {
                        target_id: target_id.clone(),
                        mouse_xy: event.global_xy,
                        container_size: container_size.clone(),
                    })
                });
            })
            .with_mouse_cursor(MouseCursor::Move),
    )
}

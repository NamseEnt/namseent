use super::*;
use crate::app::types::*;
use namui::prelude::*;
use std::sync::Arc;

pub struct BackgroundWysiwygEditor {}

pub struct BackgroundWysiwygEditorProps<'a> {
    pub rect: Rect<Px>,
    pub camera_angle: &'a CameraAngle,
}

impl BackgroundWysiwygEditor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _event: &dyn std::any::Any) {}
    pub fn render(&self, props: &BackgroundWysiwygEditorProps) -> RenderingTree {
        let container_size = Wh {
            width: props.rect.width(),
            height: props.rect.height(),
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
        let drawn_image_wh = fit_wh_in_container(image_size, container_size);
        let drawn_iamge_rect = Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: drawn_image_wh.width,
            height: drawn_image_wh.height,
        };
        let inner_rect = get_inner_rect(
            &background.source_01_circumscribed,
            Wh {
                width: px(1920.0),
                height: px(1080.0),
            },
            drawn_image_wh,
        );

        translate(
            props.rect.x(),
            props.rect.y(),
            render([
                render_outer_image(image.clone(), drawn_iamge_rect, inner_rect),
                render_inner_image(
                    image.clone(),
                    drawn_iamge_rect,
                    inner_rect,
                    drawn_iamge_rect.wh(),
                    self.get_id(),
                ),
                rect(RectParam {
                    rect: Rect::Xywh {
                        x: px(0.0),
                        y: px(0.0),
                        width: drawn_image_wh.width,
                        height: drawn_image_wh.height,
                    },
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::BLACK,
                            width: px(2.0),
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                Resizer::new(self.get_id()).render(&ResizerProps {
                    source_rect: inner_rect,
                    container_size: drawn_image_wh,
                }),
            ]),
        )
    }
    pub fn get_id(&self) -> &'static str {
        "background"
    }
}

pub fn fit_wh_in_container(image_size: Wh<Px>, container_size: Wh<Px>) -> Wh<Px> {
    if image_size.width / image_size.height > container_size.width / container_size.height {
        Wh {
            width: container_size.width,
            height: container_size.width * (image_size.height / image_size.width),
        }
    } else {
        Wh {
            width: container_size.height * (image_size.width / image_size.height),
            height: container_size.height,
        }
    }
}

pub fn get_inner_rect(
    circumscribed_01: &Circumscribed,
    image_size: Wh<Px>,
    container_size: Wh<Px>,
) -> Rect<Px> {
    let length_of_result_rect = circumscribed_01.radius * 2.0 * container_size.length();

    let image_size_length = image_size.length();
    let image_width_length_ratio = image_size.width / image_size_length;
    let image_height_length_ratio = image_size.height / image_size_length;

    let image_width_length = image_width_length_ratio * length_of_result_rect;
    let image_height_length = image_height_length_ratio * length_of_result_rect;

    Rect::Xywh {
        x: container_size.width * circumscribed_01.center.x - image_width_length / 2.0,
        y: container_size.height * circumscribed_01.center.y - image_height_length / 2.0,
        width: image_width_length,
        height: image_height_length,
    }
}

pub fn render_source_image(
    image: Arc<Image>,
    paint_builder: Option<PaintBuilder>,
    source_rect: Rect<Px>,
) -> RenderingTree {
    namui::image(ImageParam {
        rect: source_rect,
        style: ImageStyle {
            fit: ImageFit::Fill,
            paint_builder,
        },
        source: ImageSource::Image(image),
    })
}

fn render_outer_image(
    image: Arc<Image>,
    image_drawn_rect: Rect<Px>,
    inner_rect: Rect<Px>,
) -> RenderingTree {
    let outside_image_paint = namui::PaintBuilder::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color_filter(&Color::grayscale_f01(0.5), &namui::BlendMode::Multiply);

    namui::clip(
        namui::PathBuilder::new().add_rect(inner_rect),
        namui::ClipOp::Difference,
        render_source_image(image, Some(outside_image_paint), image_drawn_rect),
    )
}

fn render_inner_image(
    image: Arc<Image>,
    source_rect: Rect<Px>,
    dest_rect: Rect<Px>,
    container_size: Wh<Px>,
    id: &'static str,
) -> RenderingTree {
    let container_size = container_size.clone();

    namui::clip(
        namui::PathBuilder::new().add_rect(dest_rect),
        namui::ClipOp::Intersect,
        render_source_image(image, None, source_rect)
            .attach_event(|builder| {
                let target_id = id.to_string();
                builder.on_mouse_down_in(move |event| {
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

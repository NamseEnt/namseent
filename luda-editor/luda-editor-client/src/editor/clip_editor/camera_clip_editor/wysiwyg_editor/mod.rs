use self::resizer::{Resizer, ResizerProps};
use crate::editor::{events::EditorEvent, job::Job, types::*};
use namui::prelude::*;
use std::sync::Arc;
pub mod resizer;

pub struct WysiwygEditor {}

pub struct WysiwygEditorProps<'a> {
    pub xywh: XywhRect<f32>,
    pub camera_angle: &'a CameraAngle,
    pub image_filename_objects: &'a Vec<ImageFilenameObject>,
    pub job: &'a Option<Job>,
}

impl WysiwygEditor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {}
    pub fn render(&self, props: &WysiwygEditorProps) -> RenderingTree {
        let container_size = Wh {
            width: props.xywh.width,
            height: props.xywh.height,
        };

        let camera_angle = &match &props.job {
            Some(Job::WysiwygMoveImage(job)) => {
                let mut camera_angle = props.camera_angle.clone();
                job.move_camera_angle(&mut camera_angle);
                camera_angle
            }
            Some(Job::WysiwygResizeImage(job)) => {
                let mut camera_angle = props.camera_angle.clone();
                job.resize_camera_angle(&mut camera_angle);
                camera_angle
            }
            _ => props.camera_angle.clone(),
        };

        let image_url = camera_angle
            .character_pose_emotion
            .get_url(props.image_filename_objects);
        if image_url.is_none() {
            return RenderingTree::Empty;
        }
        let image_url = image_url.unwrap();

        let image = namui::managers().image_manager.clone().try_load(&image_url);
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let image_size = image.size();
        let source_rect = get_rect_in_container(
            &camera_angle.source_01_circumscribed,
            &image_size,
            &container_size,
        );
        let dest_rect = get_rect_in_container(
            &camera_angle.dest_01_circumscribed,
            &image_size,
            &container_size,
        );

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
                            width: 2.0,
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                render_outer_image(image.clone(), &source_rect, &dest_rect),
                render_inner_image(image.clone(), &source_rect, &dest_rect, &container_size),
                Resizer::new().render(&ResizerProps {
                    camera_angle: &camera_angle,
                    source_rect: &source_rect,
                    container_size: &container_size,
                }),
                // Croper(state),
            ],
        )
    }
}

pub fn get_rect_in_container(
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
    paint: Option<Paint>,
    source_rect: &XywhRect<f32>,
) -> RenderingTree {
    namui::image(ImageParam {
        xywh: *source_rect,
        style: ImageStyle {
            fit: ImageFit::Fill,
            paint,
        },
        source: ImageSource::Image(image),
    })
}

fn render_outer_image(
    image: Arc<Image>,
    source_rect: &XywhRect<f32>,
    dest_rect: &XywhRect<f32>,
) -> RenderingTree {
    let outside_image_paint = namui::Paint::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color_filter(&namui::ColorFilter::blend(
            &Color::gary_scale_f01(0.5),
            &namui::BlendMode::Multiply,
        ));

    namui::clip(
        namui::Path::new().add_rect(&dest_rect.into_ltrb()),
        namui::ClipOp::Difference,
        namui::render![render_source_image(
            image,
            Some(outside_image_paint),
            source_rect
        )],
    )
}

fn render_inner_image(
    image: Arc<Image>,
    source_rect: &XywhRect<f32>,
    dest_rect: &XywhRect<f32>,
    container_size: &Wh<f32>,
) -> RenderingTree {
    let container_size = container_size.clone();

    namui::clip(
        namui::Path::new().add_rect(&dest_rect.into_ltrb()),
        namui::ClipOp::Intersect,
        render_source_image(image, None, &source_rect)
            .attach_event(|builder| {
                builder.on_mouse_down(Box::new(move |event| {
                    namui::event::send(Box::new(
                        EditorEvent::WysiwygEditorInnerImageMouseDownEvent {
                            mouse_xy: event.global_xy,
                            container_size: container_size.clone(),
                        },
                    ))
                }))
            })
            .with_mouse_cursor(MouseCursor::Move),
    )
}

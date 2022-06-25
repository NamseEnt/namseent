use self::{
    super::*,
    cropper::{Cropper, CropperProps},
};
use crate::app::{editor::events::EditorEvent, types::*};
use namui::prelude::*;
use std::sync::Arc;
pub mod cropper;

pub struct CharacterWysiwygEditor {}

pub struct CharacterWysiwygEditorProps<'a> {
    pub xywh: XywhRect<f32>,
    pub camera_angle: &'a CameraAngle,
}

impl CharacterWysiwygEditor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {}
    pub fn render(&self, props: &CharacterWysiwygEditorProps) -> RenderingTree {
        let container_size = Wh {
            width: props.xywh.width,
            height: props.xywh.height,
        };

        let image_loader = LudaEditorServerCameraAngleImageLoader {};

        let character = props.camera_angle.character.as_ref();
        if character.is_none() {
            return RenderingTree::Empty;
        }
        let character = character.unwrap();

        let image_source = image_loader.get_character_image_source(character);
        let image = match image_source {
            ImageSource::Url(url) => namui::managers().image_manager.try_load(&url),
            ImageSource::Image(image) => Some(image),
        };
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let image_size = image.size();
        let source_rect = get_rect_in_container(
            &character.source_01_circumscribed,
            &image_size,
            &container_size,
        );
        let dest_rect = LtrbRect {
            left: character.crop_screen_01_rect.left * container_size.width,
            top: character.crop_screen_01_rect.top * container_size.height,
            right: character.crop_screen_01_rect.right * container_size.width,
            bottom: character.crop_screen_01_rect.bottom * container_size.height,
        };

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
                render_inner_image(
                    image.clone(),
                    &source_rect,
                    &dest_rect,
                    &container_size,
                    self.get_id()
                ),
                Resizer::new(self.get_id()).render(&ResizerProps {
                    source_rect: &source_rect,
                    container_size: &container_size,
                }),
                Cropper::new().render(&CropperProps {
                    dest_rect: &dest_rect,
                    container_size: &container_size,
                }),
            ],
        )
    }
    pub fn get_id(&self) -> &'static str {
        "character"
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
    source_rect: &XywhRect<f32>,
    dest_rect: &LtrbRect,
) -> RenderingTree {
    let outside_image_paint = namui::PaintBuilder::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color_filter(&Color::grayscale_f01(0.5), &namui::BlendMode::Multiply);

    namui::clip(
        namui::PathBuilder::new().add_rect(dest_rect),
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

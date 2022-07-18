pub mod cropper;

use self::{
    super::*,
    cropper::{Cropper, CropperProps},
};
use crate::app::{storage::GithubStorage, types::*};
use namui::prelude::*;
use std::sync::Arc;

pub struct CharacterWysiwygEditor {
    id: String,
    resizer: Resizer,
    cropper: Cropper,
}

pub struct CharacterWysiwygEditorProps<'a> {
    pub rect: Rect<Px>,
    pub camera_angle: &'a CameraAngle,
    pub storage: Arc<dyn GithubStorage>,
}

impl CharacterWysiwygEditor {
    pub fn new() -> Self {
        Self {
            id: namui::nanoid(),
            resizer: Resizer::new(),
            cropper: Cropper::new(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.resizer.update(event);
        self.cropper.update(event);
    }
    pub fn render(&self, props: &CharacterWysiwygEditorProps) -> RenderingTree {
        let container_size = Wh {
            width: props.rect.width(),
            height: props.rect.height(),
        };

        let image_loader = LudaEditorServerCameraAngleImageLoader {
            storage: props.storage.clone(),
        };

        let character = props.camera_angle.character.as_ref();
        if character.is_none() {
            return RenderingTree::Empty;
        }
        let character = character.unwrap();

        let image_source = image_loader.get_character_image_source(character);
        let image = match image_source {
            ImageSource::Url(url) => namui::image::try_load(&url),
            ImageSource::Image(image) => Some(image),
        };
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let image_size = image.size();
        let source_rect = get_rect_in_container(
            character.source_01_circumscribed,
            image_size,
            container_size,
        );
        let dest_rect = Rect::Ltrb {
            left: character.crop_screen_01_rect.left() * container_size.width,
            top: character.crop_screen_01_rect.top() * container_size.height,
            right: character.crop_screen_01_rect.right() * container_size.width,
            bottom: character.crop_screen_01_rect.bottom() * container_size.height,
        };

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
                            width: px(2.0),
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                render_outer_image(image.clone(), source_rect, dest_rect),
                render_inner_image(
                    image.clone(),
                    source_rect,
                    dest_rect,
                    container_size,
                    &self.id,
                ),
                self.resizer.render(&ResizerProps {
                    source_rect,
                    container_size,
                }),
                self.cropper.render(&CropperProps {
                    dest_rect,
                    container_size,
                }),
            ]),
        )
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

pub fn get_rect_in_container(
    circumscribed_01: Circumscribed,
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
    source_rect: Rect<Px>,
    dest_rect: Rect<Px>,
) -> RenderingTree {
    let outside_image_paint = namui::PaintBuilder::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color_filter(&Color::grayscale_f01(0.5), &namui::BlendMode::Multiply);

    namui::clip(
        namui::PathBuilder::new().add_rect(dest_rect),
        namui::ClipOp::Difference,
        render_source_image(image, Some(outside_image_paint), source_rect),
    )
}

fn render_inner_image(
    image: Arc<Image>,
    source_rect: Rect<Px>,
    dest_rect: Rect<Px>,
    container_size: Wh<Px>,
    id: &str,
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

pub mod image;
pub mod path;
pub mod text;
use self::image::draw_image;
use self::path::draw_path;
use self::text::draw_text;
use super::{
    render::ImageFit,
    skia::{Font, Paint, StrokeOptions},
    Namui, NamuiContext, NamuiImpl, Path, Xy,
};
use crate::{ImageSource, PaintBuilder, PathBuilder, XywhRect};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize, Clone)]
pub struct PathDrawCommand {
    #[serde(skip_serializing)]
    pub path_builder: PathBuilder,
    #[serde(skip_serializing)]
    pub paint_builder: PaintBuilder,
}

#[derive(Debug, Serialize, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageDrawCommand {
    pub xywh: XywhRect<f32>,
    pub source: ImageSource,
    pub fit: ImageFit,
    #[serde(skip_serializing)]
    pub paint_builder: Option<PaintBuilder>,
}
#[derive(Debug, Serialize, Copy, Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}
#[derive(Debug, Serialize, Copy, Clone)]
pub enum TextBaseline {
    Top,
    Bottom,
    Middle,
}
#[derive(Serialize, Clone, Debug)]
pub struct TextDrawCommand {
    pub text: String,
    #[serde(skip_serializing)]
    pub font: Arc<Font>,
    pub x: f32,
    pub y: f32,
    pub paint_builder: PaintBuilder,
    pub align: TextAlign,
    pub baseline: TextBaseline,
}
#[derive(Serialize, Clone, Debug)]
pub enum DrawCommand {
    Path(PathDrawCommand),
    Image(ImageDrawCommand),
    Text(TextDrawCommand),
}

#[derive(Serialize, Clone, Debug)]
pub struct DrawCall {
    pub commands: Vec<DrawCommand>,
}

impl DrawCall {
    pub fn draw(&self, namui_context: &NamuiContext) {
        self.commands.iter().for_each(|command| {
            command.draw(namui_context);
        });
    }

    pub(crate) fn get_bounding_box(&self) -> Option<crate::LtrbRect> {
        self.commands
            .iter()
            .map(|command| command.get_bounding_box())
            .filter_map(|bounding_box| bounding_box)
            .reduce(|acc, bounding_box| {
                crate::LtrbRect::get_minimum_rectangle_containing(&acc, &bounding_box)
            })
    }
}

impl DrawCommand {
    pub fn draw(&self, namui_context: &NamuiContext) {
        match self {
            &DrawCommand::Image(ref image_command) => {
                draw_image(namui_context, &image_command);
            }
            &DrawCommand::Path(ref path_command) => {
                draw_path(namui_context, &path_command);
            }
            &DrawCommand::Text(ref text_command) => {
                draw_text(namui_context, &text_command);
            }
        }
    }

    pub(crate) fn is_inside(&self, local_xy: &Xy<f32>) -> bool {
        match self {
            DrawCommand::Path(path_draw_command) => {
                let path = path_draw_command.path_builder.build();
                let paint = path_draw_command.paint_builder.build();

                if path.contains(local_xy) {
                    return true;
                }

                let mut stroke_path_builder = path_draw_command.path_builder.clone();
                let stroke_result = stroke_path_builder.stroke(StrokeOptions {
                    cap: Some(paint.get_stroke_cap()),
                    join: Some(paint.get_stroke_join()),
                    width: Some(paint.get_stroke_width()),
                    miter_limit: Some(paint.get_stroke_miter()),
                    precision: None,
                });

                match stroke_result {
                    Ok(()) => stroke_path_builder.build().contains(local_xy),
                    Err(()) => false,
                }
            }
            DrawCommand::Image(image_draw_command) => {
                let XywhRect {
                    x,
                    y,
                    width,
                    height,
                } = &image_draw_command.xywh;
                let x_max = x + width;
                let y_max = y + height;
                let local_x = local_xy.x;
                let local_y = local_xy.y;
                local_x >= *x && local_x <= x_max && local_y >= *y && local_y <= y_max
            }
            DrawCommand::Text(text_draw_command) => text_draw_command
                .get_bounding_box()
                .map_or(false, |bounding_box| bounding_box.is_xy_inside(&local_xy)),
        }
    }

    fn get_bounding_box(&self) -> Option<crate::LtrbRect> {
        match self {
            DrawCommand::Path(path_draw_command) => {
                let path = path_draw_command.path_builder.build();
                let paint = path_draw_command.paint_builder.build();

                let mut stroke_path_builder = path_draw_command.path_builder.clone();
                let stroke_result = stroke_path_builder.stroke(StrokeOptions {
                    cap: Some(paint.get_stroke_cap()),
                    join: Some(paint.get_stroke_join()),
                    width: Some(paint.get_stroke_width()),
                    miter_limit: Some(paint.get_stroke_miter()),
                    precision: None,
                });

                let path = match stroke_result {
                    Ok(()) => stroke_path_builder.build(),
                    Err(()) => path,
                };

                path.get_bounding_box()
            }
            DrawCommand::Image(image_draw_command) => Some(crate::LtrbRect {
                left: image_draw_command.xywh.x,
                top: image_draw_command.xywh.y,
                right: image_draw_command.xywh.x + image_draw_command.xywh.width,
                bottom: image_draw_command.xywh.y + image_draw_command.xywh.height,
            }),
            DrawCommand::Text(text_draw_command) => text_draw_command.get_bounding_box(),
        }
    }
}

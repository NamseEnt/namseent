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
use crate::{ImageSource, XywhRect};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize, Clone)]
pub struct PathDrawCommand {
    #[serde(skip_serializing)]
    pub path: Path,
    #[serde(skip_serializing)]
    pub paint: Paint,
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
    pub paint: Option<Paint>,
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
#[derive(Serialize, Clone)]
pub struct TextDrawCommand {
    pub text: String,
    #[serde(skip_serializing)]
    pub font: Arc<Font>,
    pub x: f32,
    pub y: f32,
    #[serde(skip_serializing)]
    pub paint: Paint,
    pub align: TextAlign,
    pub baseline: TextBaseline,
}
#[derive(Serialize, Clone)]
pub enum DrawCommand {
    Path(PathDrawCommand),
    Image(ImageDrawCommand),
    Text(TextDrawCommand),
}

#[derive(Serialize, Clone)]
pub struct DrawCall {
    pub commands: Vec<DrawCommand>,
}

impl DrawCall {
    pub fn draw(&self, namui_context: &NamuiContext) {
        self.commands.iter().for_each(|command| {
            command.draw(namui_context);
        });
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
                let path = &path_draw_command.path;
                let paint = &path_draw_command.paint;

                if path.contains(local_xy) {
                    return true;
                }

                let stroked_path = path.clone();
                let stroke_result = stroked_path.stroke(Some(StrokeOptions {
                    cap: Some(paint.get_stroke_cap()),
                    join: Some(paint.get_stroke_join()),
                    width: Some(paint.get_stroke_width()),
                    miter_limit: Some(paint.get_stroke_miter()),
                    precision: None,
                }));

                match stroke_result {
                    Ok(()) => stroked_path.contains(local_xy),
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
            DrawCommand::Text(text_draw_command) => todo!(),
        }
    }
}

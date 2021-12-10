pub mod path;
pub mod text;
use self::path::draw_path;
use self::text::draw_text;
use super::{
    skia::{Font, Paint, StrokeOptions},
    Engine, EngineContext, EngineImpl, Path, Xy,
};
use serde::Serialize;
use std::sync::Arc;
pub mod rendering_tree;
use crate::engine;
pub use rendering_tree::*;

#[derive(Debug, Serialize)]
pub struct PathDrawCommand {
    #[serde(skip_serializing)]
    pub path: Path,
    #[serde(skip_serializing)]
    pub paint: Paint,
}

#[derive(Debug, Serialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize)]
pub struct ImageDrawCommand {
    pub x: f32,
    pub y: f32,
    pub url: String,
    pub size: Size,
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
#[derive(Serialize)]
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
#[derive(Serialize)]
pub enum DrawCommand {
    Path(PathDrawCommand),
    Image(ImageDrawCommand),
    Text(TextDrawCommand),
}

#[derive(Serialize)]
pub struct DrawCall {
    pub commands: Vec<DrawCommand>,
}

impl DrawCall {
    pub fn draw(&self, engine_context: &EngineContext) {
        self.commands.iter().for_each(|command| {
            command.draw(engine_context);
        });
    }
}

impl DrawCommand {
    pub fn draw(&self, engine_context: &EngineContext) {
        match self {
            &DrawCommand::Image(ref image_command) => {
                Engine::log(format!("Drawing image: {}", image_command.x));
            }
            &DrawCommand::Path(ref path_command) => {
                draw_path(engine_context, &path_command);
            }
            &DrawCommand::Text(ref text_command) => {
                draw_text(engine_context, &text_command);
            }
        }
    }

    fn is_inside(&self, local_xy: &Xy<f32>) -> bool {
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
                //     case "image": {
                //       return (
                //         drawCommand.x <= vector.x &&
                //         vector.x <= drawCommand.x + drawCommand.size.width &&
                //         drawCommand.y <= vector.y &&
                //         vector.y <= drawCommand.y + drawCommand.size.height
                //       );
                //     }
                todo!()
            }
            DrawCommand::Text(text_draw_command) => todo!(),
        }
    }
}

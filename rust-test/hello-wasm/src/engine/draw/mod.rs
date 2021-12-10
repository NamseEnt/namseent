pub mod path;
pub mod text;
use self::path::draw_path;
use self::text::draw_text;
use super::{
    skia::{Font, Paint},
    Engine, EngineContext, EngineImpl, Path,
};
use serde::Serialize;
use std::sync::Arc;
pub mod rendering_tree;
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
    pub fn draw<TState>(&self, engine_context: &EngineContext<TState>) {
        self.commands.iter().for_each(|command| {
            command.draw(engine_context);
        });
    }
}

impl DrawCommand {
    pub fn draw<TState>(&self, engine_context: &EngineContext<TState>) {
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
}

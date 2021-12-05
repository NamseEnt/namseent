pub mod text;
use self::text::draw_text;
use serde::Serialize;
use std::sync::Arc;

use super::{
    skia::{Font, Paint},
    Engine, EngineContext, EngineImpl,
};

#[derive(Debug, Serialize)]
pub struct PathDrawCommand {
    pub path: String,
    pub stroke: String,
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

#[derive(Serialize)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
    // id?: string;
    // onClick?: MouseEventCallback;
    // onClickOut?: MouseEventCallback;
    // onMouseMoveIn?: MouseEventCallback;
    // onMouseMoveOut?: MouseEventCallback;
    // onMouseIn?: () => void;
    // onMouseDown?: MouseEventCallback;
    // onMouseUp?: MouseEventCallback;
}
#[derive(Serialize)]
pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<RenderingTree>),
    Empty,
}

impl RenderingTree {
    pub fn draw<TState>(&self, engine_context: &EngineContext<TState>) {
        match self {
            &RenderingTree::Children(ref children) => {
                for child in children {
                    child.draw(engine_context);
                }
            }
            &RenderingTree::Node(ref data) => {
                data.draw_calls.iter().for_each(|draw_call| {
                    draw_call.draw(engine_context);
                });
            }
            &RenderingTree::Empty => {}
        }
    }
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
            &DrawCommand::Image(ref image) => {
                Engine::log(format!("Drawing image: {}", image.x));
            }
            &DrawCommand::Path(ref path) => {
                Engine::log(format!("Drawing path: {}", path.path));
            }
            &DrawCommand::Text(ref text) => {
                draw_text(engine_context, &text);
            }
        }
    }
}

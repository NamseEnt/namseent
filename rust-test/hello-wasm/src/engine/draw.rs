use super::{Engine, EngineImpl};

pub struct PathDrawCommand {
    pub path: String,
    pub stroke: String,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct ImageDrawCommand {
    pub x: f32,
    pub y: f32,
    pub url: String,
    pub size: Size,
}

pub struct TextDrawCommand {
    pub text: String,
}

pub enum DrawCommand {
    Path(PathDrawCommand),
    Image(ImageDrawCommand),
    Text(TextDrawCommand),
}

pub struct DrawCall {
    pub commands: Vec<DrawCommand>,
}

pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
}

pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<Option<RenderingTree>>),
}

impl RenderingTree {
    pub fn draw(&self) {
        match self {
            &RenderingTree::Children(ref children) => {
                for child in children {
                    match child {
                        &Some(ref child) => child.draw(),
                        &None => {}
                    }
                }
            }
            &RenderingTree::Node(ref data) => {
                data.draw_calls.iter().for_each(|draw_call| {
                    draw_call.draw();
                });
            }
        }
    }
}

impl DrawCall {
    pub fn draw(&self) {
        self.commands.iter().for_each(|command| {
            command.draw();
        });
    }
}

impl DrawCommand {
    pub fn draw(&self) {
        match self {
            &DrawCommand::Image(ref image) => {
                Engine::log(format!("Drawing image: {}", image.x));
            }
            &DrawCommand::Path(ref path) => {
                Engine::log(format!("Drawing path: {}", path.path));
            }
            &DrawCommand::Text(ref text) => {
                Engine::log(format!("Drawing text: {}", text.text));
            }
        }
    }
}

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
    RenderingData(RenderingData),
    None(Option<()>),
    RenderingTree(Vec<RenderingTree>),
}

impl RenderingTree {
    pub fn draw(&self) {
        match self {
            &RenderingTree::RenderingTree(ref children) => {
                for child in children {
                    child.draw();
                }
            }
            &RenderingTree::RenderingData(ref data) => {
                data.draw_calls.iter().for_each(|draw_call| {
                    draw_call.draw();
                });
            }
            &RenderingTree::None(_) => {}
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
                println!("Drawing image: {}", image.x);
            }
            &DrawCommand::Path(ref path) => {
                println!("Drawing path: {}", path.path);
            }
            &DrawCommand::Text(ref text) => {
                println!("Drawing text: {}", text.text);
            }
        }
    }
}

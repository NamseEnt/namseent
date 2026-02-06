use crate::*;
use namui_type::*;

impl XyIn for DrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        match self {
            DrawCommand::Path { command } => command.xy_in(xy),
            DrawCommand::Text { command } => command.xy_in(xy),
            DrawCommand::Image { command } => command.xy_in(xy),
        }
    }
}
impl XyIn for PathDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        NativePath::get(&self.path).contains(Some(&self.paint), xy)
    }
}
impl XyIn for TextDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        self.bounding_box().is_some_and(|x| x.is_xy_inside(xy))
    }
}

impl XyIn for ImageDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        let path = Path::new().add_rect(self.rect);
        NativePath::get(&path).contains(self.paint.as_ref(), xy)
    }
}

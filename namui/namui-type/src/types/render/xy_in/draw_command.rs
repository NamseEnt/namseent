use crate::*;

impl XyIn for DrawCommand {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        match self {
            DrawCommand::Path { command } => command.xy_in(calculator, xy),
            DrawCommand::Text { command } => command.xy_in(calculator, xy),
            DrawCommand::Image { command } => command.xy_in(calculator, xy),
        }
    }
}
impl XyIn for PathDrawCommand {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        calculator.path_contains_xy(&self.path, Some(&self.paint), xy)
    }
}
impl XyIn for TextDrawCommand {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        self.bounding_box(calculator)
            .is_some_and(|x| x.is_xy_inside(xy))
    }
}

impl XyIn for ImageDrawCommand {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        let path = Path::new().add_rect(self.rect);
        calculator.path_contains_xy(&path, self.paint.as_ref(), xy)
    }
}

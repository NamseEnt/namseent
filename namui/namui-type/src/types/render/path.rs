use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Eq, Hash)]
pub struct Path {
    commands: Vec<PathCommand>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            commands: Vec::with_capacity(2),
        }
    }
    pub fn commands(&self) -> &Vec<PathCommand> {
        &self.commands
    }
    pub fn add_rect(mut self, rect: Rect<Px>) -> Self {
        self.commands.push(PathCommand::AddRect { rect });
        self
    }
    pub fn add_rrect(mut self, rect: Rect<Px>, rx: Px, ry: Px) -> Self {
        self.commands.push(PathCommand::AddRrect { rect, rx, ry });
        self
    }
    pub fn stroke(mut self, options: StrokeOptions) -> Self {
        self.commands.push(PathCommand::Stroke {
            stroke_options: options,
        });
        self
    }
    pub fn move_to(mut self, x: Px, y: Px) -> Self {
        self.commands.push(PathCommand::MoveTo { xy: Xy { x, y } });
        self
    }
    pub fn line_to(mut self, x: Px, y: Px) -> Self {
        self.commands.push(PathCommand::LineTo { xy: Xy { x, y } });
        self
    }
    pub fn cubic_to(mut self, first_xy: Xy<Px>, second_xy: Xy<Px>, end_xy: Xy<Px>) -> Self {
        self.commands.push(PathCommand::CubicTo {
            first_xy,
            second_xy,
            end_xy,
        });
        self
    }
    pub fn arc_to(mut self, oval: Rect<Px>, start_angle: Angle, delta_angle: Angle) -> Self {
        self.commands.push(PathCommand::ArcTo {
            oval,
            start_angle,
            delta_angle,
        });
        self
    }
    pub fn scale(mut self, sx: f32, sy: f32) -> Self {
        self.commands.push(PathCommand::Scale {
            xy: Xy {
                x: sx.into(),
                y: sy.into(),
            },
        });
        self
    }
    pub fn translate(mut self, x: Px, y: Px) -> Self {
        self.commands
            .push(PathCommand::Translate { xy: Xy { x, y } });
        self
    }
    pub fn transform(mut self, matrix: TransformMatrix) -> Self {
        self.commands.push(PathCommand::Transform { matrix });
        self
    }
    pub fn add_oval(mut self, rect: Rect<Px>) -> Self {
        self.commands.push(PathCommand::AddOval { rect });
        self
    }
    pub fn add_arc(mut self, oval: Rect<Px>, start_angle: Angle, delta_angle: Angle) -> Self {
        self.commands.push(PathCommand::AddArc {
            oval,
            start_angle,
            delta_angle,
        });
        self
    }
    pub fn add_poly(mut self, xy_array: &[Xy<Px>], close: bool) -> Self {
        self.commands.push(PathCommand::AddPoly {
            xys: xy_array.to_vec(),
            close,
        });
        self
    }
    pub fn close(mut self) -> Self {
        self.commands.push(PathCommand::Close);
        self
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum PathCommand {
    AddRect {
        rect: Rect<Px>,
    },
    AddRrect {
        rect: Rect<Px>,
        rx: Px,
        ry: Px,
    },
    Stroke {
        stroke_options: StrokeOptions,
    },
    MoveTo {
        xy: Xy<Px>,
    },
    LineTo {
        xy: Xy<Px>,
    },
    CubicTo {
        first_xy: Xy<Px>,
        second_xy: Xy<Px>,
        end_xy: Xy<Px>,
    },
    ArcTo {
        oval: Rect<Px>,
        start_angle: Angle,
        delta_angle: Angle,
    },
    Scale {
        xy: Xy<OrderedFloat>,
    },
    Translate {
        xy: Xy<Px>,
    },
    Transform {
        matrix: TransformMatrix,
    },
    AddOval {
        rect: Rect<Px>,
    },
    AddArc {
        oval: Rect<Px>,
        start_angle: Angle,
        delta_angle: Angle,
    },
    AddPoly {
        xys: Vec<Xy<Px>>,
        close: bool,
    },
    Close,
}

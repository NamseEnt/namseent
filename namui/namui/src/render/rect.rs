use crate::*;

#[derive(Clone, Copy, Debug)]
pub enum BorderPosition {
    Inside,
    Outside,
    Middle,
}

#[derive(Clone, Copy, Debug)]
pub struct RectStroke {
    pub color: Color,
    pub width: Px,
    pub border_position: BorderPosition,
}
#[derive(Clone, Copy, Debug)]
pub struct RectFill {
    pub color: Color,
}
#[derive(Clone, Copy, Debug)]
pub struct RectRound {
    pub radius: Px,
}
#[derive(Default, Clone, Copy, Debug)]
pub struct RectStyle {
    pub stroke: Option<RectStroke>,
    pub fill: Option<RectFill>,
    pub round: Option<RectRound>,
}
#[derive(Default, Clone, Copy, Debug)]
pub struct RectParam {
    pub rect: Rect<Px>,
    pub style: RectStyle,
}

/// # NOTE for anti-aliasing
/// If you use odd width with `BorderPosition::Inside`, the border will be
/// rendered with anti-aliasing. It will be blurred.
///
/// # Rect stroke example
/// - Rect(x: 2, y: 2, w: 4, h: 4)
/// - border_width: 1
/// - stroke: s
/// - half stroke: h (anti-alias)
/// - fill: f
/// - corner: x
///
/// ```text
/// // BorderPosition::Inside
///  0 1 2 3 4 5 6 7
/// 0┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │ │ │ │ │ │
/// 1┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │ │ │ │ │ │
/// 2┼─┼─x─┼─┼─┼─x─┼─
///  │ │ │s│s│s│s│ │
/// 3┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │s│f│f│s│ │
/// 4┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │s│f│f│s│ │
/// 5┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │s│s│s│s│ │
/// 6┼─┼─x─┼─┼─┼─x─┼─
///  │ │ │ │ │ │ │ │
/// 7┼─┼─┼─┼─┼─┼─┼─┼─
///
/// // BorderPosition::Outside
///  0 1 2 3 4 5 6 7
/// 0┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │ │ │ │ │ │
/// 1┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │s│s│s│s│s│s│
/// 2┼─┼─x─┼─┼─┼─x─┼─
///  │ │s│f│f│f│f│s│
/// 3┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │s│f│f│f│f│s│
/// 4┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │s│f│f│f│f│s│
/// 5┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │s│f│f│f│f│s│
/// 6┼─┼─x─┼─┼─┼─x─┼─
///  │ │s│s│s│s│s│s│
/// 7┼─┼─┼─┼─┼─┼─┼─┼─
///
/// // BorderPosition::Middle
///  0 1 2 3 4 5 6 7
/// 0┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │ │ │ │ │ │ │
/// 1┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │h│h│h│h│h│h│
/// 2┼─┼─x─┼─┼─┼─x─┼─
///  │ │h│h│h│h│h│h│
/// 3┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │h│h│f│f│h│h│
/// 4┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │h│h│f│f│h│h│
/// 5┼─┼─┼─┼─┼─┼─┼─┼─
///  │ │h│h│h│h│h│h│
/// 6┼─┼─x─┼─┼─┼─x─┼─
///  │ │h│h│h│h│h│h│
/// 7┼─┼─┼─┼─┼─┼─┼─┼─
/// ```
///
pub fn rect(param: RectParam) -> RenderingTree {
    let (rect, translate_xy) = get_rect_and_translate_xy(&param);

    let rect_path = get_rect_path(rect, param.style.round);

    let mut draw_commands: Vec<DrawCommand> = Vec::with_capacity(2);

    if let Some(RectFill { color }) = param.style.fill {
        let fill_paint = Paint::new(color)
            .set_style(PaintStyle::Fill)
            .set_anti_alias(true);
        draw_commands.push(DrawCommand::Path {
            command: PathDrawCommand {
                path: rect_path.clone(),
                paint: fill_paint,
            }
            .into(),
        });
    };

    if let Some(RectStroke {
        color,
        width: stroke_width,
        ..
    }) = param.style.stroke
        && stroke_width > 0.px()
    {
        let stroke_paint = Paint::new(color)
            .set_stroke_width(stroke_width)
            .set_style(PaintStyle::Stroke)
            .set_anti_alias(true);

        draw_commands.push(DrawCommand::Path {
            command: PathDrawCommand {
                path: rect_path,
                paint: stroke_paint,
            }
            .into(),
        });
    };

    translate(
        translate_xy.x,
        translate_xy.y,
        render(draw_commands.into_iter().map(RenderingTree::Node)),
    )
}

fn get_rect_path(rect: Rect<Px>, round: Option<RectRound>) -> Path {
    match round {
        Some(round) => Path::new().add_rrect(rect, round.radius, round.radius),
        None => Path::new().add_rect(rect),
    }
}

fn get_rect_and_translate_xy(param: &RectParam) -> (Rect<Px>, Xy<Px>) {
    let Xywh {
        x,
        y,
        width,
        height,
    } = param.rect.as_xywh();

    match param.style.stroke {
        Some(RectStroke {
            border_position: BorderPosition::Inside,
            width: stroke_width,
            ..
        }) => (
            Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: width - stroke_width,
                height: height - stroke_width,
            },
            Xy::new(x + stroke_width / 2.0, y + stroke_width / 2.0),
        ),
        Some(RectStroke {
            border_position: BorderPosition::Outside,
            width: stroke_width,
            ..
        }) => (
            Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: width + stroke_width,
                height: height + stroke_width,
            },
            Xy::new(x - stroke_width / 2.0, y - stroke_width / 2.0),
        ),
        None
        | Some(RectStroke {
            border_position: BorderPosition::Middle,
            ..
        }) => (
            Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width,
                height,
            },
            Xy::new(x, y),
        ),
    }
}

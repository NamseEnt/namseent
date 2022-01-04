use crate::namui::{self, *};

pub enum BorderPosition {
    Inside,
    Outside,
    Middle,
}

pub struct RectStroke {
    pub color: Color,
    pub width: f32,
    pub border_position: BorderPosition,
}
pub struct RectFill {
    pub color: Color,
}
pub struct RectRound {
    pub radius: f32,
}
#[derive(Default)]
pub struct RectStyle {
    pub stroke: Option<RectStroke>,
    pub fill: Option<RectFill>,
    pub round: Option<RectRound>,
}
#[derive(Default)]
pub struct RectParam {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub style: RectStyle,
}

pub fn rect(
    RectParam {
        x,
        y,
        width,
        height,
        style: RectStyle {
            stroke,
            fill,
            round,
        },
        ..
    }: RectParam,
) -> RenderingTree {
    let mut rendering_tree: Vec<RenderingTree> = vec![];
    let (x, y, rect) = match stroke {
        None
        | Some(RectStroke {
            border_position: BorderPosition::Outside,
            ..
        }) => (
            x,
            y,
            XywhRect {
                x: 0.0,
                y: 0.0,
                width,
                height,
            },
        ),
        Some(RectStroke {
            border_position: BorderPosition::Middle,
            width: stroke_width,
            ..
        }) => (
            x + stroke_width,
            y + stroke_width,
            XywhRect {
                x: 0.0,
                y: 0.0,
                width: width - 2.0 * stroke_width,
                height: height - 2.0 * stroke_width,
            },
        ),
        Some(RectStroke {
            border_position: BorderPosition::Inside,
            width: stroke_width,
            ..
        }) => (
            x + stroke_width / 2.0,
            y + stroke_width / 2.0,
            XywhRect {
                x: 0.0,
                y: 0.0,
                width: width - stroke_width,
                height: height - stroke_width,
            },
        ),
    };

    let rect_path = get_rect_path(rect, round);

    let mut draw_commands: Vec<DrawCommand> = vec![];

    if let Some(RectFill { color }) = fill {
        let fill_paint = namui::PaintBuilder::new()
            .set_color(color)
            .set_style(namui::PaintStyle::Fill)
            .set_anti_alias(true);

        draw_commands.push(DrawCommand::Path(PathDrawCommand {
            path_builder: rect_path.clone(),
            paint_builder: fill_paint,
        }));
    };

    if let Some(RectStroke {
        color,
        width: stroke_width,
        ..
    }) = stroke
    {
        let stroke_paint = namui::PaintBuilder::new()
            .set_color(color)
            .set_stroke_width(stroke_width)
            .set_style(namui::PaintStyle::Stroke)
            .set_anti_alias(true);

        draw_commands.push(DrawCommand::Path(PathDrawCommand {
            path_builder: rect_path.clone(),
            paint_builder: stroke_paint,
        }));
    };

    rendering_tree.push(RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: draw_commands,
        }],
    }));

    translate(x, y, RenderingTree::Children(rendering_tree))
}

fn get_rect_path(rect: XywhRect<f32>, round: Option<RectRound>) -> namui::PathBuilder {
    match round {
        Some(round) => {
            namui::PathBuilder::new().add_rrect(&rect.into_ltrb(), round.radius, round.radius)
        }
        None => namui::PathBuilder::new().add_rect(&rect.into_ltrb()),
    }
}

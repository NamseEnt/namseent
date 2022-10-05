use crate::namui::{self, *};

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

pub fn rect(
    RectParam {
        rect,
        style: RectStyle {
            stroke,
            fill,
            round,
        },
        ..
    }: RectParam,
) -> RenderingTree {
    let mut rendering_tree: Vec<RenderingTree> = vec![];
    let Xywh {
        x,
        y,
        width,
        height,
    } = rect.as_xywh();
    let (x, y, rect) = match stroke {
        None
        | Some(RectStroke {
            border_position: BorderPosition::Outside,
            ..
        }) => (
            x,
            y,
            Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
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
            Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: width - stroke_width * 2.0,
                height: height - stroke_width * 2.0,
            },
        ),
        Some(RectStroke {
            border_position: BorderPosition::Inside,
            width: stroke_width,
            ..
        }) => (
            x + stroke_width / 2.0,
            y + stroke_width / 2.0,
            Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
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

fn get_rect_path(rect: Rect<Px>, round: Option<RectRound>) -> namui::PathBuilder {
    match round {
        Some(round) => namui::PathBuilder::new().add_rrect(rect, round.radius, round.radius),
        None => namui::PathBuilder::new().add_rect(rect),
    }
}

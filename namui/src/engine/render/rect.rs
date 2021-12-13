use crate::engine::{self, *};
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
    pub id: Option<String>,
    pub style: RectStyle,
    pub on_click: Option<Box<dyn Fn(&engine::Xy<f32>)>>,
    //   pub onClickOut?: MouseEventCallback,
    //   pub onMouseIn?: () => void,
    //   pub onMouseMoveIn?: MouseEventCallback,
    //   pub onMouseMoveOut?: MouseEventCallback,
    //   pub onMouseDown?: MouseEventCallback,
    //   pub onMouseUp?: MouseEventCallback,
    //   pub onAfterDraw?: (id: string) => void,
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
        id,
        on_click,
        ..
    }: RectParam,
) -> RenderingTree {
    let mut rendering_tree: Vec<RenderingTree> = vec![];
    let rect: engine::XywhRect<f32> = match stroke {
        None
        | Some(RectStroke {
            border_position: BorderPosition::Outside,
            ..
        }) => XywhRect {
            x,
            y,
            width,
            height,
        },
        Some(RectStroke {
            border_position: BorderPosition::Middle,
            width: stroke_width,
            ..
        }) => XywhRect {
            x: x + stroke_width,
            y: y + stroke_width,
            width: width - 2.0 * stroke_width,
            height: height - 2.0 * stroke_width,
        },
        Some(RectStroke {
            border_position: BorderPosition::Inside,
            width: stroke_width,
            ..
        }) => XywhRect {
            x: x + stroke_width / 2.0,
            y: y + stroke_width / 2.0,
            width: width - stroke_width,
            height: height - stroke_width,
        },
    };

    let rect_path = get_rect_path(rect, round);

    let mut draw_commands: Vec<DrawCommand> = vec![];

    if let Some(RectStroke {
        color,
        width: stroke_width,
        ..
    }) = stroke
    {
        let stroke_paint = engine::Paint::new();
        stroke_paint.set_color(&color);
        stroke_paint.set_stroke_width(stroke_width);
        stroke_paint.set_style(&engine::PaintStyle::Stroke);
        stroke_paint.set_anti_alias(true);

        draw_commands.push(DrawCommand::Path(PathDrawCommand {
            path: rect_path.clone(),
            paint: stroke_paint,
        }));
    };

    if let Some(RectFill { color }) = fill {
        let fill_paint = engine::Paint::new();
        fill_paint.set_color(&color);
        fill_paint.set_style(&engine::PaintStyle::Fill);
        fill_paint.set_anti_alias(true);

        draw_commands.push(DrawCommand::Path(PathDrawCommand {
            path: rect_path.clone(),
            paint: fill_paint,
        }));
    };

    // TODO
    //   if (onAfterDraw) {
    //     if (!id) {
    //       id = nanoid();
    //     }
    //     renderingTree.push(
    //       AfterDraw((param) => {
    //         onAfterDraw(id!);
    //       }),
    //     );
    //   }

    rendering_tree.push(RenderingTree::Node(RenderingData {
        draw_calls: vec![DrawCall {
            commands: draw_commands,
        }],
        id,
        on_click,
    }));

    RenderingTree::Children(rendering_tree)
}

//   function getRectPath(rect: InputRect) {
fn get_rect_path(rect: XywhRect<f32>, round: Option<RectRound>) -> engine::Path {
    let rect_path = engine::Path::new();

    if let Some(round) = round {
        rect_path.add_rrect(rect.into_ltrb(), round.radius, round.radius);
    } else {
        rect_path.add_rect(rect.into_ltrb());
    }

    rect_path
}

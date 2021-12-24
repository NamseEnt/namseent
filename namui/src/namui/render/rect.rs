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
    pub id: Option<String>,
    pub style: RectStyle,
    // pub on_click: Option<MouseEventCallback>,
    //   pub onClickOut?: MouseEventCallback,
    //   pub onMouseIn?: () => void,
    pub on_mouse_move_in: Option<MouseEventCallback>,
    pub on_mouse_move_out: Option<MouseEventCallback>,
    pub on_mouse_down: Option<MouseEventCallback>,
    pub on_mouse_up: Option<MouseEventCallback>,
    //   pub onAfterDraw?: (id: string) => void,
    pub on_wheel: Option<WheelEventCallback>,
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
        on_mouse_move_in,
        on_mouse_move_out,
        on_mouse_down,
        on_mouse_up,
        on_wheel,
        ..
    }: RectParam,
) -> RenderingTree {
    let mut rendering_tree: Vec<RenderingTree> = vec![];
    let rect: namui::XywhRect<f32> = match stroke {
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

    if let Some(RectFill { color }) = fill {
        let fill_paint = namui::Paint::new()
            .set_color(color)
            .set_style(namui::PaintStyle::Fill)
            .set_anti_alias(true);

        draw_commands.push(DrawCommand::Path(PathDrawCommand {
            path: rect_path.clone(),
            paint: fill_paint,
        }));
    };

    if let Some(RectStroke {
        color,
        width: stroke_width,
        ..
    }) = stroke
    {
        let stroke_paint = namui::Paint::new()
            .set_color(color)
            .set_stroke_width(stroke_width)
            .set_style(namui::PaintStyle::Stroke)
            .set_anti_alias(true);

        draw_commands.push(DrawCommand::Path(PathDrawCommand {
            path: rect_path.clone(),
            paint: stroke_paint,
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
        on_mouse_move_in,
        on_mouse_move_out,
        on_mouse_down,
        on_mouse_up,
        on_wheel,
    }));

    RenderingTree::Children(rendering_tree)
}

//   function getRectPath(rect: InputRect) {
fn get_rect_path(rect: XywhRect<f32>, round: Option<RectRound>) -> namui::Path {
    match round {
        Some(round) => namui::Path::new().add_rrect(rect.into_ltrb(), round.radius, round.radius),
        None => namui::Path::new().add_rect(rect.into_ltrb()),
    }
}

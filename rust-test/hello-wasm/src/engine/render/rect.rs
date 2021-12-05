// use crate::engine::{self, *};
// // import { Color, InputRect } from "canvaskit-wasm";
// // import { AfterDraw } from "..";
// // import {
// //   MouseEventCallback,
// //   DrawCommand,
// //   RenderingTree,
// //   BorderPosition,
// // } from "../type";
// // import { nanoid } from "nanoid";

// enum BorderPosition {
//     Inside,
//     Outside,
//     Middle,
// }

// pub struct RectStroke {
//     color: Color,
//     width: f32,
//     border_position: BorderPosition,
// }
// pub struct RectFill {
//     color: Color,
// }
// pub struct RectRound {
//     radius: f32,
// }
// #[derive(Default)]
// pub struct RectStyle {
//     stroke: Option<RectStroke>,
//     fill: Option<RectFill>,
//     round: Option<RectRound>,
// }
// #[derive(Default)]
// pub struct RectParam<'a> {
//     pub x: f32,
//     pub y: f32,
//     pub width: f32,
//     pub height: f32,
//     pub id: Option<&'a str>,
//     pub style: RectStyle,
//     //   pub onClick?: MouseEventCallback,
//     //   pub onClickOut?: MouseEventCallback,
//     //   pub onMouseIn?: () => void,
//     //   pub onMouseMoveIn?: MouseEventCallback,
//     //   pub onMouseMoveOut?: MouseEventCallback,
//     //   pub onMouseDown?: MouseEventCallback,
//     //   pub onMouseUp?: MouseEventCallback,
//     //   pub onAfterDraw?: (id: string) => void,
// }

// struct Rect<'a> {
//     x: f32,
//     y: f32,
//     width: f32,
//     height: f32,
//     id: Option<&'a str>,
//     style: RectStyle,
//     //   onClick?: MouseEventCallback,
//     //   onClickOut?: MouseEventCallback,
//     //   onMouseIn?: () => void,
//     //   onMouseMoveIn?: MouseEventCallback,
//     //   onMouseMoveOut?: MouseEventCallback,
//     //   onMouseDown?: MouseEventCallback,
//     //   onMouseUp?: MouseEventCallback,
//     //   onAfterDraw?: (id: string) => void,
// }

// // export function Rect({
// //   x,
// //   y,
// //   width,
// //   height,
// //   id,
// //   style: { stroke, fill, round },
// //   onClick,
// //   onClickOut,
// //   onMouseIn,
// //   onMouseMoveIn,
// //   onMouseMoveOut,
// //   onMouseDown,
// //   onMouseUp,
// //   onAfterDraw,
// // }: {
// //   x: number;
// //   y: number;
// //   width: number;
// //   height: number;
// //   id?: string;
// //   style: {
// //     stroke?: {
// //       color: Color;
// //       width: number;
// //       borderPosition: BorderPosition;
// //     };
// //     fill?: {
// //       color: Color;
// //     };
// //     round?: {
// //       radius: number;
// //     };
// //   };
// //   onClick?: MouseEventCallback;
// //   onClickOut?: MouseEventCallback;
// //   onMouseIn?: () => void;
// //   onMouseMoveIn?: MouseEventCallback;
// //   onMouseMoveOut?: MouseEventCallback;
// //   onMouseDown?: MouseEventCallback;
// //   onMouseUp?: MouseEventCallback;
// //   onAfterDraw?: (id: string) => void;
// // }): RenderingTree {
// pub fn rect(
//     RectParam {
//         x,
//         y,
//         width,
//         height,
//         style: RectStyle {
//             stroke,
//             fill,
//             round,
//         },
//         ..
//     }: RectParam,
// ) -> RenderingTree {
//     //   const renderingTree = [];
//     let rendering_tree = vec![];

//     //   let rect: InputRect;
//     //   if (!stroke || stroke.borderPosition === BorderPosition.outside) {
//     //     rect = CanvasKit.XYWHRect(x, y, width, height);
//     //   } else if (stroke.borderPosition === BorderPosition.inside) {
//     //     rect = CanvasKit.XYWHRect(
//     //       x + stroke.width,
//     //       y + stroke.width,
//     //       width - 2 * stroke.width,
//     //       height - 2 * stroke.width,
//     //     );
//     //   } else {
//     //     rect = CanvasKit.XYWHRect(
//     //       x + stroke.width / 2,
//     //       y + stroke.width / 2,
//     //       width - stroke.width,
//     //       height - stroke.width,
//     //     );
//     //   }
//     let rect: engine::XywhRect<f32> = match stroke {
//         None
//         | Some(RectStroke {
//             border_position: BorderPosition::Outside,
//             ..
//         }) => XywhRect {
//             x,
//             y,
//             width,
//             height,
//         },
//         Some(RectStroke {
//             border_position: BorderPosition::Middle,
//             width: stroke_width,
//             ..
//         }) => XywhRect {
//             x: x + stroke_width,
//             y: y + stroke_width,
//             width: width - 2.0 * stroke_width,
//             height: height - 2.0 * stroke_width,
//         },
//         Some(RectStroke {
//             border_position: BorderPosition::Inside,
//             width: stroke_width,
//             ..
//         }) => XywhRect {
//             x: x + stroke_width / 2.0,
//             y: y + stroke_width / 2.0,
//             width: width - stroke_width,
//             height: height - stroke_width,
//         },
//     };

//     //   const rectPath = getRectPath(rect);

//     //   const drawCommands: DrawCommand[] = [];

//     //   if (stroke) {
//     //     const strokePaint = new CanvasKit.Paint();
//     //     strokePaint.setColor(stroke.color);
//     //     strokePaint.setStrokeWidth(stroke.width);
//     //     strokePaint.setStyle(CanvasKit.PaintStyle.Stroke);
//     //     strokePaint.setAntiAlias(true);

//     //     drawCommands.push({
//     //       type: "path",
//     //       path: rectPath,
//     //       paint: strokePaint,
//     //     });
//     //   }

//     //   if (fill) {
//     //     const fillPaint = new CanvasKit.Paint();
//     //     fillPaint.setColor(fill.color);
//     //     fillPaint.setStyle(CanvasKit.PaintStyle.Fill);
//     //     fillPaint.setAntiAlias(true);

//     //     drawCommands.push({
//     //       type: "path",
//     //       path: rectPath,
//     //       paint: fillPaint,
//     //     });
//     //   }

//     //   if (onAfterDraw) {
//     //     if (!id) {
//     //       id = nanoid();
//     //     }
//     //     renderingTree.push(
//     //       AfterDraw((param) => {
//     //         onAfterDraw(id!);
//     //       }),
//     //     );
//     //   }

//     //   renderingTree.push({
//     //     drawCalls: [
//     //       {
//     //         commands: drawCommands,
//     //       },
//     //     ],
//     //     id,
//     //     onClick,
//     //     onClickOut,
//     //     onMouseIn,
//     //     onMouseMoveIn,
//     //     onMouseMoveOut,
//     //     onMouseDown,
//     //     onMouseUp,
//     //   });

//     //   return renderingTree;
//     // }
// }

// //   function getRectPath(rect: InputRect) {
// fn get_rect_path(rect: XywhRect<f32>) -> engine::Path {
//     //     const rectPath = new CanvasKit.Path();
//     //     if (round) {
//     //       rectPath.addRRect(CanvasKit.RRectXY(rect, round.radius, round.radius));
//     //     } else {
//     //       rectPath.addRect(rect);
//     //     }
//     //     return rectPath;
//     //   }
// }

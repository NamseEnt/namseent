use super::*;
use crate::namui::{ClipOp, DrawCall, NamuiContext, Xy, *};
use serde::Serialize;
use std::{borrow::Borrow, ops::ControlFlow};
mod visit;

#[derive(Serialize, Default, Clone, Debug)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
}
#[derive(Serialize, Clone, Debug)]
pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<RenderingTree>),
    Special(SpecialRenderingNode),
    Empty,
}

impl SpecialRenderingNode {
    fn get_rendering_tree(&self) -> &RenderingTree {
        match self {
            SpecialRenderingNode::Translate(node) => &node.rendering_tree,
            SpecialRenderingNode::Clip(node) => &node.rendering_tree,
            SpecialRenderingNode::AttachEvent(node) => &node.rendering_tree,
            SpecialRenderingNode::MouseCursor(node) => &node.rendering_tree,
            SpecialRenderingNode::WithId(node) => &node.rendering_tree,
            SpecialRenderingNode::Absolute(node) => &node.rendering_tree,
            SpecialRenderingNode::Rotate(node) => &node.rendering_tree,
            SpecialRenderingNode::Custom(node) => &node.rendering_tree,
            SpecialRenderingNode::Scale(node) => &node.rendering_tree,
        }
    }
}
/// NOTE
/// Order of tree traversal is important.
/// - draw = pre-order dfs (NLR)
/// - events = Reverse post-order (RLN)
/// reference: https://en.wikipedia.org/wiki/Tree_traversal
impl RenderingTree {
    pub(crate) fn draw(&self, namui_context: &NamuiContext) {
        match self {
            RenderingTree::Children(ref children) => {
                for child in children {
                    child.draw(namui_context);
                }
            }
            RenderingTree::Node(rendering_data) => {
                rendering_data.draw_calls.iter().for_each(|draw_call| {
                    draw_call.draw(namui_context);
                });
            }
            RenderingTree::Special(special) => match special {
                SpecialRenderingNode::Translate(translate) => {
                    namui_context
                        .surface
                        .canvas()
                        .translate(translate.x, translate.y);

                    translate.rendering_tree.draw(namui_context);

                    namui_context
                        .surface
                        .canvas()
                        .translate(-translate.x, -translate.y);
                }
                SpecialRenderingNode::Clip(clip) => {
                    let canvas = namui_context.surface.canvas();

                    canvas.save();

                    let path = clip.path_builder.build();
                    canvas.clip_path(path.as_ref(), &clip.clip_op, true);

                    clip.rendering_tree.draw(namui_context);

                    canvas.restore();
                }
                SpecialRenderingNode::Absolute(absolute) => {
                    let canvas = namui_context.surface.canvas();

                    let back_up_matrix = canvas.get_matrix();

                    canvas.set_matrix(&[
                        [1.0, 0.0, absolute.x],
                        [0.0, 1.0, absolute.y],
                        [0.0, 0.0, 1.0],
                    ]);

                    absolute.rendering_tree.draw(namui_context);

                    canvas.set_matrix(&back_up_matrix);
                }
                SpecialRenderingNode::Rotate(rotate) => {
                    let canvas = namui_context.surface.canvas();

                    canvas.rotate(rotate.ccw_radian);

                    rotate.rendering_tree.draw(namui_context);

                    canvas.rotate(-rotate.ccw_radian);
                }
                SpecialRenderingNode::Scale(scale) => {
                    let canvas = namui_context.surface.canvas();

                    canvas.scale(scale.x, scale.y);

                    scale.rendering_tree.draw(namui_context);

                    canvas.scale(1.0 / scale.x, 1.0 / scale.y);
                }
                _ => {
                    special.get_rendering_tree().draw(namui_context);
                }
            },
            RenderingTree::Empty => {}
        }
    }
    pub(crate) fn call_wheel_event(
        &self,
        raw_wheel_event: &RawWheelEvent,
        namui_context: &NamuiContext,
    ) {
        self.visit_rln(|node, _| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::AttachEvent(attach_event) = special {
                    // NOTE : Should i check if the mouse is in the attach_event?
                    if let Some(on_wheel) = &attach_event.on_wheel {
                        on_wheel(&WheelEvent {
                            delta_xy: raw_wheel_event.delta_xy,
                            id: raw_wheel_event.id.clone(),
                            namui_context,
                            target: node,
                        });
                    }
                }
            }
            ControlFlow::Continue(())
        });
    }
    pub(crate) fn call_keyboard_event(
        &self,
        raw_keyboard_event: &RawKeyboardEvent,
        namui_context: &NamuiContext,
        down_up: DownUp,
    ) {
        self.visit_rln(|node, _| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::AttachEvent(attach_event) = special {
                    let callback = match down_up {
                        DownUp::Down => &attach_event.on_key_down,
                        DownUp::Up => &attach_event.on_key_up,
                    };
                    if let Some(callback) = &callback {
                        callback(&KeyboardEvent {
                            id: raw_keyboard_event.id.clone(),
                            namui_context,
                            target: node,
                            code: raw_keyboard_event.code,
                            pressing_codes: raw_keyboard_event.pressing_codes.clone(),
                        });
                    }
                }
            }
            ControlFlow::Continue(())
        });
    }
    pub(crate) fn get_mouse_cursor(&self, xy: Xy<f32>) -> Option<MouseCursor> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            match node {
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::MouseCursor(mouse_cursor) => {
                        if utils.is_xy_in(xy) {
                            result = Some(*(mouse_cursor.cursor.clone()));
                            return ControlFlow::Break(());
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
            ControlFlow::Continue(())
        });
        result
    }
    pub(crate) fn call_mouse_event(
        &self,
        mouse_event_type: MouseEventType,
        raw_mouse_event: &RawMouseEvent,
        namui_context: &NamuiContext,
    ) {
        self.visit_rln(|node, utils| {
            match node {
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::AttachEvent(attach_event) => {
                        let (in_func, out_func) = match mouse_event_type {
                            MouseEventType::Move => (
                                &attach_event.on_mouse_move_in,
                                &attach_event.on_mouse_move_out,
                            ),
                            MouseEventType::Down => (&attach_event.on_mouse_down, &None),
                            MouseEventType::Up => (&attach_event.on_mouse_up, &None),
                        };
                        if in_func.is_some() || out_func.is_some() {
                            let is_mouse_in = utils.is_xy_in(raw_mouse_event.xy);
                            let mouse_event = MouseEvent {
                                id: raw_mouse_event.id.clone(),
                                global_xy: raw_mouse_event.xy,
                                local_xy: utils.to_local_xy(raw_mouse_event.xy),
                                pressing_buttons: raw_mouse_event.pressing_buttons.clone(),
                                button: raw_mouse_event.button,
                                target: node,
                                namui_context,
                            };
                            match is_mouse_in {
                                true => {
                                    if let Some(in_func) = in_func {
                                        in_func(&mouse_event);
                                    }
                                }
                                false => {
                                    if let Some(out_func) = out_func {
                                        out_func(&mouse_event);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
            ControlFlow::Continue(())
        });
    }
    pub(crate) fn get_xy_by_id(&self, id: &str) -> Option<Xy<f32>> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            match node {
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::WithId(with_id) => {
                        if with_id.id == id {
                            result = Some(utils.get_xy());
                            return ControlFlow::Break(());
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
            ControlFlow::Continue(())
        });
        result
    }
    pub(crate) fn get_xy_of_child(&self, child: &RenderingTree) -> Option<Xy<f32>> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            if std::ptr::eq(node, child) {
                result = Some(utils.get_xy());
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        result
    }
    pub fn get_bounding_box(&self) -> Option<XywhRect<f32>> {
        fn get_bounding_box_with_matrix(
            rendering_tree: &RenderingTree,
            matrix: &Matrix3x3,
        ) -> Option<LtrbRect> {
            fn get_bounding_box_with_matrix_of_rendering_trees<'a>(
                rendering_trees: impl IntoIterator<Item = impl Borrow<RenderingTree>>,
                matrix: &Matrix3x3,
            ) -> Option<LtrbRect> {
                rendering_trees
                    .into_iter()
                    .map(|child| get_bounding_box_with_matrix(child.borrow(), &matrix))
                    .filter_map(|bounding_box| bounding_box)
                    .reduce(|acc, bounding_box| {
                        LtrbRect::get_minimum_rectangle_containing(&acc, &bounding_box)
                    })
            }

            match rendering_tree {
                RenderingTree::Children(ref children) => {
                    get_bounding_box_with_matrix_of_rendering_trees(children, matrix)
                }
                RenderingTree::Node(rendering_data) => rendering_data
                    .get_bounding_box()
                    .map(|bounding_box| matrix.transform_rect(&bounding_box)),
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        let translation_matrix = Matrix3x3::from_slice(&[
                            [1.0, 0.0, translate.x],
                            [0.0, 1.0, translate.y],
                            [0.0, 0.0, 1.0],
                        ]);
                        let matrix = translation_matrix * matrix;
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [translate.rendering_tree.borrow()],
                            &matrix,
                        )
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [clip.rendering_tree.borrow()],
                            &matrix,
                        )
                        .and_then(|bounding_box| {
                            let path = clip.path_builder.build();

                            let clip_bounding_box = path.get_bounding_box();

                            match clip.clip_op {
                                ClipOp::Intersect => {
                                    clip_bounding_box.and_then(|clip_bounding_box| {
                                        bounding_box.intersect(clip_bounding_box)
                                    })
                                }
                                ClipOp::Difference => match clip_bounding_box {
                                    Some(clip_bounding_box) => {
                                        if bounding_box == clip_bounding_box {
                                            return None;
                                        }

                                        let xs = [
                                            bounding_box.left,
                                            bounding_box.right,
                                            clip_bounding_box.left,
                                            clip_bounding_box.right,
                                        ];
                                        let ys = [
                                            bounding_box.top,
                                            bounding_box.bottom,
                                            clip_bounding_box.top,
                                            clip_bounding_box.bottom,
                                        ];

                                        let sixteen_xys = xs
                                            .iter()
                                            .zip(ys.iter())
                                            .map(|(x, y)| Xy { x: *x, y: *y });

                                        let difference_area_xys = sixteen_xys.filter(|xy| {
                                            (clip_bounding_box.is_xy_outside(*xy)
                                                || clip_bounding_box.is_xy_on_border(*xy))
                                                && !bounding_box.is_xy_outside(*xy)
                                        });

                                        difference_area_xys.fold(None, |acc, xy| match acc {
                                            Some(rect) => Some(LtrbRect {
                                                left: rect.left.min(xy.x),
                                                top: rect.top.min(xy.y),
                                                right: rect.right.max(xy.x),
                                                bottom: rect.bottom.max(xy.y),
                                            }),
                                            None => Some(LtrbRect {
                                                left: xy.x,
                                                top: xy.y,
                                                right: xy.x,
                                                bottom: xy.y,
                                            }),
                                        })
                                    }
                                    None => Some(bounding_box),
                                },
                            }
                        })
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        let matrix = Matrix3x3::from_slice(&[
                            [1.0, 0.0, absolute.x],
                            [0.0, 1.0, absolute.y],
                            [0.0, 0.0, 1.0],
                        ]);
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [absolute.rendering_tree.borrow()],
                            &matrix,
                        )
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        let matrix = matrix * rotate.get_matrix();

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [rotate.rendering_tree.borrow()],
                            &matrix,
                        )
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        let matrix = matrix * scale.get_matrix();

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [scale.rendering_tree.borrow()],
                            &matrix,
                        )
                    }
                    _ => get_bounding_box_with_matrix_of_rendering_trees(
                        [special.get_rendering_tree()],
                        &matrix,
                    ),
                },
                RenderingTree::Empty => None,
            }
        }

        get_bounding_box_with_matrix(&self, &Matrix3x3::identity()).and_then(|bounding_box| {
            Some(XywhRect {
                x: bounding_box.left,
                y: bounding_box.top,
                width: bounding_box.right - bounding_box.left,
                height: bounding_box.bottom - bounding_box.top,
            })
        })
    }
}

impl RenderingData {
    fn get_bounding_box(&self) -> Option<LtrbRect> {
        self.draw_calls
            .iter()
            .map(|draw_call| draw_call.get_bounding_box())
            .filter_map(|bounding_box| bounding_box)
            .reduce(|acc, bounding_box| {
                LtrbRect::get_minimum_rectangle_containing(&acc, &bounding_box)
            })
    }
    fn is_xy_in(&self, xy: Xy<f32>) -> bool {
        self.draw_calls
            .iter()
            .any(|draw_call| draw_call.is_xy_in(xy))
    }
}

pub(crate) enum DownUp {
    Down,
    Up,
}

// NOTE: I will uncomment this when wasm_bindgen_test support init canvas_kit
// #[cfg(test)]
// mod tests {
//     use crate::*;

//     use super::RenderingTree;
//     use wasm_bindgen_test::*;

//     #[test]
//     #[wasm_bindgen_test]
//     fn call_on_click_should_run_in_dfs_pre_order() {
//         /*
//             tree:
//                  0
//                /   \
//               1     2

//             screen:

//                 ┌─────── 0 ────────┐       ┌─────── 2 ────────┐
//                 │                  │       │                  │
//                 │                  │       │                  │
//                 │       ┌───────1────────┐ │                  │
//                 │       │                │ │                  │
//                 │       │  ●             │ │                  │
//                 └───────│                │ └──────────────────┘
//                         │                │
//                         │                │
//                         └────────────────┘
//             click on: ●
//             call order: 0, 1
//         */
//         static mut ON_MOUSE_DOWN_CALLED_ID_LIST: Vec<String> = vec![];

//         let rendering_tree = RenderingTree::Children(vec![
//             namui::rect(namui::RectParam {
//                 x: 0.0,
//                 y: 0.0,
//                 width: 100.0,
//                 height: 100.0,
//                 id: Some("0".to_string()),
//                 style: namui::RectStyle {
//                     fill: None,
//                     stroke: None,
//                     round: None,
//                 },
//                 on_mouse_down: Some(Box::new(move |xy| unsafe {
//                     ON_MOUSE_DOWN_CALLED_ID_LIST.push("0".to_string());
//                 })),
//                 ..Default::default()
//             }),
//             RenderingTree::Children(vec![namui::rect(namui::RectParam {
//                 x: 50.0,
//                 y: 50.0,
//                 width: 100.0,
//                 height: 100.0,
//                 id: Some("1".to_string()),
//                 style: namui::RectStyle {
//                     fill: None,
//                     stroke: None,
//                     round: None,
//                 },
//                 on_mouse_down: Some(Box::new(move |xy| unsafe {
//                     ON_MOUSE_DOWN_CALLED_ID_LIST.push("1".to_string());
//                 })),
//                 ..Default::default()
//             })]),
//             RenderingTree::Children(vec![namui::rect(namui::RectParam {
//                 x: 210.0,
//                 y: 0.0,
//                 width: 100.0,
//                 height: 100.0,
//                 id: Some("2".to_string()),
//                 style: namui::RectStyle {
//                     fill: None,
//                     stroke: None,
//                     round: None,
//                 },
//                 on_mouse_down: Some(Box::new(move |xy| unsafe {
//                     ON_MOUSE_DOWN_CALLED_ID_LIST.push("2".to_string());
//                 })),
//                 ..Default::default()
//             })]),
//         ]);

//         rendering_tree.call_mouse_event(MouseEventType::Down, &namui::Xy { x: 75.0, y: 75.0 });

//         unsafe {
//             assert_eq!(ON_MOUSE_DOWN_CALLED_ID_LIST, vec!["0", "1", "3", "5"]);
//         };
//     }
// }

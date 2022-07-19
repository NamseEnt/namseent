use super::*;
use crate::namui::{ClipOp, DrawCall, NamuiContext, Xy, *};
use serde::Serialize;
use std::{
    borrow::Borrow,
    ops::ControlFlow,
    sync::atomic::{AtomicBool, Ordering},
};
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

// NOTE : to support putting MouseCursor into event.
unsafe impl Send for RenderingTree {}
unsafe impl Sync for RenderingTree {}

impl SpecialRenderingNode {
    pub(crate) fn get_rendering_tree(&self) -> Arc<RenderingTree> {
        match self {
            SpecialRenderingNode::Translate(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Clip(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::AttachEvent(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::MouseCursor(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::WithId(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Absolute(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Rotate(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Custom(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Scale(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::Transform(node) => node.rendering_tree.clone(),
            SpecialRenderingNode::React(node) => node.render(),
            SpecialRenderingNode::OnTop(node) => node.rendering_tree.clone(),
        }
    }
}
/// NOTE
/// Order of tree traversal is important.
/// - draw = pre-order dfs (NLR)
/// - events = Reverse post-order (RLN)
/// reference: https://en.wikipedia.org/wiki/Tree_traversal
impl RenderingTree {
    pub(crate) fn draw(&self) {
        struct DrawContext {
            on_top_node_matrix_tuples: Vec<(OnTopNode, Matrix3x3)>,
        }
        fn draw_internal(rendering_tree: &RenderingTree, draw_context: &mut DrawContext) {
            match rendering_tree {
                RenderingTree::Children(ref children) => {
                    for child in children.iter() {
                        draw_internal(child, draw_context);
                    }
                }
                RenderingTree::Node(rendering_data) => {
                    rendering_data.draw_calls.iter().for_each(|draw_call| {
                        draw_call.draw();
                    });
                }
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface()
                            .canvas()
                            .translate(translate.x, translate.y);

                        draw_internal(&translate.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        crate::graphics::surface().canvas().save();
                        let path = clip.path_builder.build();
                        crate::graphics::surface().canvas().clip_path(
                            path.as_ref(),
                            &clip.clip_op,
                            true,
                        );
                        draw_internal(&clip.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Absolute(absolute) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface()
                            .canvas()
                            .set_matrix(Matrix3x3::from_slice([
                                [1.0, 0.0, absolute.x.as_f32()],
                                [0.0, 1.0, absolute.y.as_f32()],
                                [0.0, 0.0, 1.0],
                            ]));
                        draw_internal(&absolute.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface().canvas().rotate(rotate.angle);
                        draw_internal(&rotate.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface().canvas().scale(scale.x, scale.y);
                        draw_internal(&scale.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        crate::graphics::surface().canvas().save();
                        crate::graphics::surface()
                            .canvas()
                            .transform(transform.matrix);
                        draw_internal(&transform.rendering_tree, draw_context);
                        crate::graphics::surface().canvas().restore();
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let matrix = crate::graphics::surface().canvas().get_matrix();
                        draw_context
                            .on_top_node_matrix_tuples
                            .push((on_top.clone(), matrix));
                    }
                    SpecialRenderingNode::AttachEvent(_)
                    | SpecialRenderingNode::MouseCursor(_)
                    | SpecialRenderingNode::WithId(_)
                    | SpecialRenderingNode::Custom(_)
                    | SpecialRenderingNode::React(_) => {
                        draw_internal(&special.get_rendering_tree(), draw_context);
                    }
                },
                RenderingTree::Empty => {}
            }
        }

        let mut draw_context = DrawContext {
            on_top_node_matrix_tuples: Vec::new(),
        };
        draw_internal(self, &mut draw_context);

        for (node, matrix) in draw_context.on_top_node_matrix_tuples {
            crate::graphics::surface().canvas().save();
            crate::graphics::surface().canvas().set_matrix(matrix);
            node.rendering_tree.draw();
            crate::graphics::surface().canvas().restore();
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
    pub(crate) fn get_mouse_cursor(&self, xy: Xy<Px>) -> Option<MouseCursor> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::MouseCursor(mouse_cursor) = special {
                    if utils.is_xy_in(xy) {
                        result = Some(*(mouse_cursor.cursor.clone()));
                        return ControlFlow::Break(());
                    }
                }
            }
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
        let mut is_stop_propagation = false;

        enum Layer {
            Top,
            Down,
        }
        for layer in [Layer::Top, Layer::Down] {
            if is_stop_propagation {
                break;
            }

            self.visit_rln(|node, utils| {
                if let RenderingTree::Special(special) = node {
                    if let SpecialRenderingNode::AttachEvent(attach_event) = special {
                        let is_on_right_layer = {
                            let has_top_ancestor = utils.ancestors.iter().any(|ancestor| {
                                if let RenderingTree::Special(SpecialRenderingNode::OnTop(_)) =
                                    ancestor
                                {
                                    return true;
                                }
                                false
                            });

                            match layer {
                                Layer::Top => has_top_ancestor,
                                Layer::Down => !has_top_ancestor,
                            }
                        };
                        if !is_on_right_layer {
                            return ControlFlow::Continue(());
                        }

                        let (in_func, out_func) = match mouse_event_type {
                            MouseEventType::Move => (
                                &attach_event.on_mouse_move_in,
                                &attach_event.on_mouse_move_out,
                            ),
                            MouseEventType::Down => (
                                &attach_event.on_mouse_down_in,
                                &attach_event.on_mouse_down_out,
                            ),
                            MouseEventType::Up => {
                                (&attach_event.on_mouse_up_in, &attach_event.on_mouse_up_out)
                            }
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
                                is_stop_propagation: Arc::new(AtomicBool::new(false)),
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

                            if mouse_event.is_stop_propagation.load(Ordering::Relaxed) {
                                is_stop_propagation = true;
                                return ControlFlow::Break(());
                            }
                        }
                    }
                }
                ControlFlow::Continue(())
            });
        }
    }
    pub(crate) fn get_xy_by_id(&self, id: &str) -> Option<Xy<Px>> {
        let mut result = None;
        self.visit_rln(|node, utils| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::WithId(with_id) = special {
                    if with_id.id == id {
                        result = Some(utils.get_xy());
                        return ControlFlow::Break(());
                    }
                }
            }
            ControlFlow::Continue(())
        });
        result
    }
    pub(crate) fn get_xy_of_child(&self, child: &RenderingTree) -> Option<Xy<Px>> {
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
    pub fn get_bounding_box(&self) -> Option<Rect<Px>> {
        struct BoundingBoxContext {
            bounding_boxes_on_top: Vec<Option<Rect<Px>>>,
        }
        fn get_bounding_box_with_matrix(
            rendering_tree: &RenderingTree,
            matrix: &Matrix3x3,
            bounding_box_context: &mut BoundingBoxContext,
        ) -> Option<Rect<Px>> {
            fn get_bounding_box_with_matrix_of_rendering_trees<'a>(
                rendering_trees: impl IntoIterator<Item = impl Borrow<RenderingTree>>,
                matrix: &Matrix3x3,
                bounding_box_context: &mut BoundingBoxContext,
            ) -> Option<Rect<Px>> {
                rendering_trees
                    .into_iter()
                    .map(|child| {
                        get_bounding_box_with_matrix(child.borrow(), &matrix, bounding_box_context)
                    })
                    .filter_map(|bounding_box| bounding_box)
                    .reduce(|acc, bounding_box| {
                        Rect::get_minimum_rectangle_containing(&acc, bounding_box)
                    })
            }

            match rendering_tree {
                RenderingTree::Children(ref children) => {
                    get_bounding_box_with_matrix_of_rendering_trees(
                        children,
                        matrix,
                        bounding_box_context,
                    )
                }
                RenderingTree::Node(rendering_data) => rendering_data
                    .get_bounding_box()
                    .map(|bounding_box| matrix.transform_rect(bounding_box)),
                RenderingTree::Special(special) => match special {
                    SpecialRenderingNode::Translate(translate) => {
                        let translation_matrix = Matrix3x3::from_slice([
                            [1.0, 0.0, translate.x.as_f32()],
                            [0.0, 1.0, translate.y.as_f32()],
                            [0.0, 0.0, 1.0],
                        ]);
                        let matrix = translation_matrix * matrix;
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [translate.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Clip(clip) => {
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [clip.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
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
                                            bounding_box.left(),
                                            bounding_box.right(),
                                            clip_bounding_box.left(),
                                            clip_bounding_box.right(),
                                        ];
                                        let ys = [
                                            bounding_box.top(),
                                            bounding_box.bottom(),
                                            clip_bounding_box.top(),
                                            clip_bounding_box.bottom(),
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
                                            Some(rect) => Some(Rect::Ltrb {
                                                left: rect.left().min(xy.x),
                                                top: rect.top().min(xy.y),
                                                right: rect.right().max(xy.x),
                                                bottom: rect.bottom().max(xy.y),
                                            }),
                                            None => Some(Rect::Ltrb {
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
                        let matrix = Matrix3x3::from_slice([
                            [1.0, 0.0, absolute.x.as_f32()],
                            [0.0, 1.0, absolute.y.as_f32()],
                            [0.0, 0.0, 1.0],
                        ]);
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [absolute.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Rotate(rotate) => {
                        let matrix = matrix * rotate.get_matrix();

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [rotate.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Scale(scale) => {
                        let matrix = matrix * scale.get_matrix();

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [scale.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::Transform(transform) => {
                        let matrix = transform.matrix * matrix;

                        get_bounding_box_with_matrix_of_rendering_trees(
                            [transform.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                    SpecialRenderingNode::OnTop(on_top) => {
                        let bounding_box = get_bounding_box_with_matrix_of_rendering_trees(
                            [on_top.rendering_tree.borrow()],
                            &matrix,
                            bounding_box_context,
                        );
                        bounding_box_context
                            .bounding_boxes_on_top
                            .push(bounding_box);
                        bounding_box
                    }
                    SpecialRenderingNode::AttachEvent(_)
                    | SpecialRenderingNode::MouseCursor(_)
                    | SpecialRenderingNode::WithId(_)
                    | SpecialRenderingNode::Custom(_)
                    | SpecialRenderingNode::React(_) => {
                        get_bounding_box_with_matrix_of_rendering_trees(
                            [special.get_rendering_tree()],
                            &matrix,
                            bounding_box_context,
                        )
                    }
                },
                RenderingTree::Empty => None,
            }
        }

        let mut bounding_box_context = BoundingBoxContext {
            bounding_boxes_on_top: vec![],
        };
        let bounding_box =
            get_bounding_box_with_matrix(&self, &Matrix3x3::identity(), &mut bounding_box_context);

        bounding_box_context
            .bounding_boxes_on_top
            .into_iter()
            .filter_map(|x| x)
            .fold(bounding_box, |acc, bounding_box| {
                acc.and_then(|acc| Some(Rect::get_minimum_rectangle_containing(&acc, bounding_box)))
            })
    }
}

impl RenderingData {
    fn get_bounding_box(&self) -> Option<Rect<Px>> {
        self.draw_calls
            .iter()
            .map(|draw_call| draw_call.get_bounding_box())
            .filter_map(|bounding_box| bounding_box)
            .reduce(|acc, bounding_box| Rect::get_minimum_rectangle_containing(&acc, bounding_box))
    }
    fn is_xy_in(&self, xy: Xy<Px>) -> bool {
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
//                 on_mouse_down_in: Some(Box::new(move |xy| unsafe {
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
//                 on_mouse_down_in: Some(Box::new(move |xy| unsafe {
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
//                 on_mouse_down_in: Some(Box::new(move |xy| unsafe {
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

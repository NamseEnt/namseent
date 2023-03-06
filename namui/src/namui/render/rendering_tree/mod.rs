mod bounding_box;
mod visit;

use self::visit::VisitUtils;
use super::*;
use crate::{drag_and_drop::RawFileDropEvent, namui::*};
use serde::Serialize;
use std::{
    ops::ControlFlow,
    sync::atomic::{AtomicBool, Ordering},
};

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
    pub(crate) fn call_wheel_event(&self, raw_wheel_event: &RawWheelEvent) {
        self.visit_rln(|node, _| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::AttachEvent(attach_event) = special {
                    // NOTE : Should i check if the mouse is in the attach_event?
                    if let Some(on_wheel) = &attach_event.on_wheel {
                        let is_stop_propagation = Arc::new(AtomicBool::new(false));

                        on_wheel(&WheelEvent {
                            delta_xy: raw_wheel_event.delta_xy,
                            id: raw_wheel_event.id.clone(),
                            root: self,
                            target: node,
                            is_stop_propagation: is_stop_propagation.clone(),
                        });

                        if is_stop_propagation.load(Ordering::Relaxed) {
                            return ControlFlow::Break(());
                        }
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
    fn call_event_of_screen(
        &self,
        callback: impl Fn(CallEventOfScreenParam) -> CallEventOfScreenResult,
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
                let RenderingTree::Special(special) = node else {
                    return ControlFlow::Continue(());
                };

                let SpecialRenderingNode::AttachEvent(attach_event) = special else {
                    return ControlFlow::Continue(());
                };
                let is_on_right_layer = {
                    let has_top_ancestor = utils.ancestors.iter().any(|ancestor| {
                        if let RenderingTree::Special(SpecialRenderingNode::OnTop(_)) = ancestor {
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

                let result = callback(CallEventOfScreenParam {
                    attach_event,
                    utils,
                    node,
                });
                if result.is_stop_propagation {
                    is_stop_propagation = true;
                    return ControlFlow::Break(());
                }

                ControlFlow::Continue(())
            });
        }
    }

    pub(crate) fn call_mouse_event(
        &self,
        mouse_event_type: MouseEventType,
        raw_mouse_event: &RawMouseEvent,
    ) {
        self.call_event_of_screen(
            |CallEventOfScreenParam {
                 attach_event,
                 node,
                 utils,
             }| {
                let (in_func, out_func, on_mouse) = match mouse_event_type {
                    MouseEventType::Move => (
                        &attach_event.on_mouse_move_in,
                        &attach_event.on_mouse_move_out,
                        &attach_event.on_mouse,
                    ),
                    MouseEventType::Down => (
                        &attach_event.on_mouse_down_in,
                        &attach_event.on_mouse_down_out,
                        &attach_event.on_mouse,
                    ),
                    MouseEventType::Up => (
                        &attach_event.on_mouse_up_in,
                        &attach_event.on_mouse_up_out,
                        &attach_event.on_mouse,
                    ),
                };
                if in_func.is_none() && out_func.is_none() && on_mouse.is_none() {
                    return CallEventOfScreenResult {
                        is_stop_propagation: false,
                    };
                }
                let is_mouse_in = utils.is_xy_in(raw_mouse_event.xy);
                let mouse_event = MouseEvent {
                    id: raw_mouse_event.id.clone(),
                    global_xy: raw_mouse_event.xy,
                    local_xy: utils.to_local_xy(raw_mouse_event.xy),
                    pressing_buttons: raw_mouse_event.pressing_buttons.clone(),
                    button: raw_mouse_event.button,
                    target: node,
                    root: self,
                    event_type: mouse_event_type,
                    is_stop_propagation: Arc::new(AtomicBool::new(false)),
                };

                if let Some(on_mouse) = &attach_event.on_mouse {
                    on_mouse(&mouse_event);
                }
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

                return CallEventOfScreenResult {
                    is_stop_propagation: mouse_event.is_stop_propagation.load(Ordering::Relaxed),
                };
            },
        );
    }
    pub(crate) fn call_file_drop_event(
        &self,
        file_drop_event: &RawFileDropEvent,
        namui_context: &NamuiContext,
    ) {
        self.call_event_of_screen(
            |CallEventOfScreenParam {
                 attach_event,
                 node,
                 utils,
             }| {
                let Some(on_file_drop) = &attach_event.on_file_drop else {
                    return CallEventOfScreenResult {
                        is_stop_propagation: false,
                    };
                };
                let is_mouse_in = utils.is_xy_in(file_drop_event.global_xy);
                if !is_mouse_in {
                    return CallEventOfScreenResult {
                        is_stop_propagation: false,
                    };
                }

                let event = FileDropEvent {
                    namui_context,
                    target: node,
                    global_xy: file_drop_event.global_xy,
                    local_xy: utils.to_local_xy(file_drop_event.global_xy),
                    files: file_drop_event.files.clone(),
                    is_stop_propagation: Arc::new(AtomicBool::new(false)),
                };
                on_file_drop(&event);

                return CallEventOfScreenResult {
                    is_stop_propagation: event.is_stop_propagation.load(Ordering::Relaxed),
                };
            },
        );
    }
    pub fn get_xy_by_id(&self, id: crate::Uuid) -> Option<Xy<Px>> {
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
    pub fn get_xy_of_child(&self, child: &RenderingTree) -> Option<Xy<Px>> {
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
struct CallEventOfScreenParam<'a> {
    node: &'a RenderingTree,
    attach_event: &'a AttachEventNode,
    utils: VisitUtils<'a>,
}
struct CallEventOfScreenResult {
    is_stop_propagation: bool,
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

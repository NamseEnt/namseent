use super::{AttachEventNode, ClipNode, MouseCursor, MouseCursorNode, TranslateNode};
use crate::namui::{ClipOp, DrawCall, NamuiContext, Xy, *};
use serde::Serialize;

#[derive(Clone)]
pub struct MouseEvent {
    pub local_xy: Xy<f32>,
    pub global_xy: Xy<f32>,
}
pub enum MouseEventType {
    Down,
    Up,
    Move,
}
pub type BoxedMouseEventCallback = Box<dyn Fn(&MouseEvent)>;
pub type MouseEventCallback = Arc<dyn Fn(&MouseEvent)>;
pub type BoxedWheelEventCallback = Box<dyn Fn(&Xy<f32>)>;
pub type WheelEventCallback = Arc<dyn Fn(&Xy<f32>)>;
#[derive(Serialize, Default, Clone)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
}
#[derive(Serialize, Clone)]
pub enum SpecialRenderingNode {
    Translate(TranslateNode),
    Clip(ClipNode),
    AttachEvent(AttachEventNode),
    MouseCursor(MouseCursorNode),
}
#[derive(Serialize, Clone)]
pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<RenderingTree>),
    Special(SpecialRenderingNode),
    Empty,
}

impl SpecialRenderingNode {
    fn get_children(&self) -> &Vec<RenderingTree> {
        match self {
            SpecialRenderingNode::Translate(node) => &node.rendering_tree,
            SpecialRenderingNode::Clip(node) => &node.rendering_tree,
            SpecialRenderingNode::AttachEvent(node) => &node.rendering_tree,
            SpecialRenderingNode::MouseCursor(node) => &node.rendering_tree,
        }
    }
}

impl RenderingTree {
    pub fn draw(&self, namui_context: &NamuiContext) {
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

                    for child in &translate.rendering_tree {
                        child.draw(namui_context);
                    }

                    namui_context
                        .surface
                        .canvas()
                        .translate(-translate.x, -translate.y);
                }
                SpecialRenderingNode::Clip(clip) => {
                    let canvas = namui_context.surface.canvas();

                    canvas.save();

                    canvas.clip_path(&clip.path, &clip.clip_op, true);

                    for child in &clip.rendering_tree {
                        child.draw(namui_context);
                    }

                    canvas.restore();
                }
                SpecialRenderingNode::AttachEvent(attach_event) => {
                    for child in &attach_event.rendering_tree {
                        child.draw(namui_context);
                    }
                }
                SpecialRenderingNode::MouseCursor(mouse_cursor) => {
                    for child in &mouse_cursor.rendering_tree {
                        child.draw(namui_context);
                    }
                }
            },
            RenderingTree::Empty => {}
        }
    }
    pub fn visit(&self, callback: &dyn Fn(&Self)) {
        callback(self);
        match self {
            RenderingTree::Children(ref children) => {
                for child in children {
                    child.visit(callback);
                }
            }
            RenderingTree::Special(special) => {
                for child in special.get_children() {
                    child.visit(callback);
                }
            }
            _ => {}
        }
    }
    pub fn call_wheel_event(&self, wheel_event: &Xy<f32>) {
        self.visit(&|node| {
            if let RenderingTree::Special(special) = node {
                if let SpecialRenderingNode::AttachEvent(attach_event) = special {
                    // NOTE : Should i check if the mouse is in the attach_event?
                    if let Some(on_wheel) = &attach_event.on_wheel {
                        on_wheel(wheel_event);
                    }
                }
            }
        });
    }
    pub fn get_mouse_cursor(&self, xy: &Xy<f32>) -> Option<MouseCursor> {
        match self {
            RenderingTree::Children(ref children) => children
                .iter()
                .rev()
                .find_map(|child| child.get_mouse_cursor(xy)),
            RenderingTree::Special(special) => match special {
                SpecialRenderingNode::Translate(translate) => {
                    let next_xy = Xy {
                        x: xy.x - translate.x,
                        y: xy.y - translate.y,
                    };
                    translate
                        .rendering_tree
                        .iter()
                        .rev()
                        .find_map(|child| child.get_mouse_cursor(&next_xy))
                }
                SpecialRenderingNode::Clip(clip) => {
                    let is_path_contains = clip.path.contains(xy);
                    let is_xy_filtered = match clip.clip_op {
                        ClipOp::Intersect => !is_path_contains,
                        ClipOp::Difference => is_path_contains,
                    };
                    if is_xy_filtered {
                        None
                    } else {
                        clip.rendering_tree
                            .iter()
                            .rev()
                            .find_map(|child| child.get_mouse_cursor(&xy))
                    }
                }
                SpecialRenderingNode::AttachEvent(attach_event) => attach_event
                    .rendering_tree
                    .iter()
                    .rev()
                    .find_map(|child| child.get_mouse_cursor(&xy)),
                SpecialRenderingNode::MouseCursor(mouse_cursor) => mouse_cursor
                    .rendering_tree
                    .iter()
                    .rev()
                    .find_map(|child| child.get_mouse_cursor(&xy))
                    .or(Some(mouse_cursor.cursor.clone())),
            },
            _ => None,
        }
    }
    pub fn call_mouse_event(&self, mouse_event_type: MouseEventType, xy: &Xy<f32>) {
        self.call_mouse_event_impl(&mouse_event_type, xy, xy);
    }
    fn call_mouse_event_impl(
        &self,
        mouse_event_type: &MouseEventType,
        global_xy: &Xy<f32>,
        local_xy: &Xy<f32>,
    ) {
        match self {
            RenderingTree::Children(ref children) => {
                for child in children {
                    child.call_mouse_event_impl(mouse_event_type, global_xy, local_xy);
                }
            }
            RenderingTree::Special(special) => match special {
                SpecialRenderingNode::Translate(translate) => {
                    let next_local_xy = Xy {
                        x: local_xy.x - translate.x,
                        y: local_xy.y - translate.y,
                    };
                    for child in &translate.rendering_tree {
                        child.call_mouse_event_impl(mouse_event_type, global_xy, &next_local_xy);
                    }
                }
                SpecialRenderingNode::Clip(clip) => {
                    let is_path_contains = clip.path.contains(local_xy);
                    let is_xy_filtered = match clip.clip_op {
                        ClipOp::Intersect => !is_path_contains,
                        ClipOp::Difference => is_path_contains,
                    };
                    if !is_xy_filtered {
                        for child in &clip.rendering_tree {
                            child.call_mouse_event_impl(mouse_event_type, global_xy, local_xy);
                        }
                    }
                }
                SpecialRenderingNode::AttachEvent(attach_event) => {
                    let func = match mouse_event_type {
                        MouseEventType::Move => &attach_event.on_mouse_move_in,
                        MouseEventType::Down => &attach_event.on_mouse_down,
                        MouseEventType::Up => &attach_event.on_mouse_up,
                    };
                    if let Some(func) = func {
                        if attach_event
                            .rendering_tree
                            .iter()
                            .any(|child| child.is_point_in(global_xy, local_xy))
                        {
                            func(&MouseEvent {
                                global_xy: *global_xy,
                                local_xy: *local_xy,
                            });
                        }
                    }
                }
                SpecialRenderingNode::MouseCursor(mouse_cursor) => {
                    for child in &mouse_cursor.rendering_tree {
                        child.call_mouse_event_impl(mouse_event_type, global_xy, local_xy);
                    }
                }
            },
            _ => {}
        }
    }

    fn is_point_in(&self, global_xy: &Xy<f32>, local_xy: &Xy<f32>) -> bool {
        match self {
            RenderingTree::Children(ref children) => children
                .iter()
                .any(|child| child.is_point_in(global_xy, local_xy)),
            RenderingTree::Node(rendering_data) => rendering_data.is_inside(local_xy),
            RenderingTree::Special(special) => match special {
                SpecialRenderingNode::Translate(translate) => {
                    let next_local_xy = Xy {
                        x: local_xy.x - translate.x,
                        y: local_xy.y - translate.y,
                    };
                    translate
                        .rendering_tree
                        .iter()
                        .any(|child| child.is_point_in(global_xy, &next_local_xy))
                }
                SpecialRenderingNode::Clip(clip) => {
                    let is_path_contains = clip.path.contains(local_xy);
                    let is_xy_filtered = match clip.clip_op {
                        ClipOp::Intersect => !is_path_contains,
                        ClipOp::Difference => is_path_contains,
                    };
                    !is_xy_filtered
                        && clip
                            .rendering_tree
                            .iter()
                            .any(|child| child.is_point_in(global_xy, local_xy))
                }
                SpecialRenderingNode::AttachEvent(attach_event) => attach_event
                    .rendering_tree
                    .iter()
                    .any(|child| child.is_point_in(global_xy, local_xy)),
                SpecialRenderingNode::MouseCursor(mouse_cursor) => mouse_cursor
                    .rendering_tree
                    .iter()
                    .any(|child| child.is_point_in(global_xy, local_xy)),
            },
            RenderingTree::Empty => false,
        }
    }
}

impl RenderingData {
    fn is_inside(&self, local_xy: &Xy<f32>) -> bool {
        self.draw_calls.iter().any(|draw_call| {
            // TODO : Handle drawCall.clip
            draw_call
                .commands
                .iter()
                .any(|draw_command| draw_command.is_inside(local_xy))
        })
    }
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

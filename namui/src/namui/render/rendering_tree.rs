use super::{AttachEventNode, ClipNode, MouseCursor, MouseCursorNode, TranslateNode, WithIdNode};
use crate::namui::{ClipOp, DrawCall, NamuiContext, Xy, *};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct MouseEvent {
    pub local_xy: Xy<f32>,
    pub global_xy: Xy<f32>,
    pub pressing_buttons: HashSet<MouseButton>,
    pub button: Option<MouseButton>,
}
pub enum MouseEventType {
    Down,
    Up,
    Move,
}
pub struct WheelEvent<'a> {
    pub delta_xy: &'a Xy<f32>,
    pub namui_context: &'a NamuiContext,
}
pub type MouseEventCallback = Arc<dyn Fn(&MouseEvent)>;
pub type WheelEventCallback = Arc<dyn Fn(&WheelEvent)>;
#[derive(Serialize, Default, Clone, Debug)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
}
#[derive(Serialize, Clone, Debug)]
pub enum SpecialRenderingNode {
    Translate(TranslateNode),
    Clip(ClipNode),
    AttachEvent(AttachEventNode),
    MouseCursor(MouseCursorNode),
    WithId(WithIdNode),
}
#[derive(Serialize, Clone, Debug)]
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
            SpecialRenderingNode::WithId(node) => &node.rendering_tree,
        }
    }
}
/// NOTE
/// Order of tree traversal is important.
/// - draw = pre-order dfs (NLR)
/// - events = Reverse post-order (RLN)
/// reference: https://en.wikipedia.org/wiki/Tree_traversal
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

                    let path = clip.path_builder.build();
                    canvas.clip_path(path.as_ref(), &clip.clip_op, true);

                    for child in &clip.rendering_tree {
                        child.draw(namui_context);
                    }

                    canvas.restore();
                }
                _ => {
                    for child in special.get_children() {
                        child.draw(namui_context);
                    }
                }
            },
            RenderingTree::Empty => {}
        }
    }
    pub fn visit_rln(&self, callback: &dyn Fn(&Self)) {
        match self {
            RenderingTree::Children(ref children) => {
                children.iter().rev().for_each(|child| {
                    child.visit_rln(callback);
                });
            }
            RenderingTree::Special(special) => {
                special.get_children().iter().rev().for_each(|child| {
                    child.visit_rln(callback);
                });
            }
            _ => {}
        }
        callback(self);
    }
    pub fn call_wheel_event(&self, wheel_event: &WheelEvent) {
        self.visit_rln(&|node| {
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
                    let is_path_contains = clip.path_builder.build().contains(xy);
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
                SpecialRenderingNode::MouseCursor(mouse_cursor) => mouse_cursor
                    .rendering_tree
                    .iter()
                    .rev()
                    .find_map(|child| child.get_mouse_cursor(&xy))
                    .or(mouse_cursor
                        .rendering_tree
                        .iter()
                        .any(|child| child.is_point_in(&xy))
                        .then(|| mouse_cursor.cursor)),
                _ => special
                    .get_children()
                    .iter()
                    .rev()
                    .find_map(|child| child.get_mouse_cursor(&xy)),
            },
            _ => None,
        }
    }
    pub fn call_mouse_event(
        &self,
        mouse_event_type: MouseEventType,
        raw_mouse_event: &RawMouseEvent,
    ) {
        self.call_mouse_event_impl(&mouse_event_type, raw_mouse_event, &raw_mouse_event.xy);
    }
    fn call_mouse_event_impl(
        &self,
        mouse_event_type: &MouseEventType,
        raw_mouse_event: &RawMouseEvent,
        local_xy: &Xy<f32>,
    ) {
        match self {
            RenderingTree::Children(ref children) => {
                children.iter().rev().for_each(|child| {
                    child.call_mouse_event_impl(mouse_event_type, raw_mouse_event, local_xy);
                });
            }
            RenderingTree::Special(special) => match special {
                SpecialRenderingNode::Translate(translate) => {
                    let next_local_xy = Xy {
                        x: local_xy.x - translate.x,
                        y: local_xy.y - translate.y,
                    };
                    translate.rendering_tree.iter().rev().for_each(|child| {
                        child.call_mouse_event_impl(
                            mouse_event_type,
                            raw_mouse_event,
                            &next_local_xy,
                        );
                    });
                }
                SpecialRenderingNode::Clip(clip) => {
                    let is_path_contains = clip.path_builder.build().contains(local_xy);
                    let is_xy_filtered = match clip.clip_op {
                        ClipOp::Intersect => !is_path_contains,
                        ClipOp::Difference => is_path_contains,
                    };
                    if !is_xy_filtered {
                        clip.rendering_tree.iter().rev().for_each(|child| {
                            child.call_mouse_event_impl(
                                mouse_event_type,
                                raw_mouse_event,
                                local_xy,
                            );
                        });
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
                            .any(|child| child.is_point_in(local_xy))
                        {
                            func(&MouseEvent {
                                global_xy: raw_mouse_event.xy,
                                local_xy: *local_xy,
                                pressing_buttons: raw_mouse_event.pressing_buttons.clone(),
                                button: raw_mouse_event.button,
                            });
                        }
                    }
                }
                _ => {
                    special.get_children().iter().rev().for_each(|child| {
                        child.call_mouse_event_impl(mouse_event_type, raw_mouse_event, local_xy);
                    });
                }
            },
            _ => {}
        }
    }

    fn is_point_in(&self, local_xy: &Xy<f32>) -> bool {
        match self {
            RenderingTree::Children(ref children) => {
                children.iter().any(|child| child.is_point_in(local_xy))
            }
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
                        .any(|child| child.is_point_in(&next_local_xy))
                }
                SpecialRenderingNode::Clip(clip) => {
                    let is_path_contains = clip.path_builder.build().contains(local_xy);
                    let is_xy_filtered = match clip.clip_op {
                        ClipOp::Intersect => !is_path_contains,
                        ClipOp::Difference => is_path_contains,
                    };
                    !is_xy_filtered
                        && clip
                            .rendering_tree
                            .iter()
                            .any(|child| child.is_point_in(local_xy))
                }
                _ => special
                    .get_children()
                    .iter()
                    .any(|child| child.is_point_in(local_xy)),
            },
            RenderingTree::Empty => false,
        }
    }

    pub(crate) fn get_xy(&self, id: &str) -> Option<Xy<f32>> {
        match self {
            RenderingTree::Children(ref children) => {
                children.iter().rev().find_map(|child| child.get_xy(id))
            }
            RenderingTree::Special(special) => match special {
                SpecialRenderingNode::Translate(translate) => {
                    let next_xy = Xy {
                        x: translate.x,
                        y: translate.y,
                    };
                    translate
                        .rendering_tree
                        .iter()
                        .rev()
                        .find_map(|child| child.get_xy(id).map(|xy| xy + next_xy))
                }
                SpecialRenderingNode::WithId(with_id) => {
                    if with_id.id == id {
                        Some(Xy { x: 0.0, y: 0.0 })
                    } else {
                        special
                            .get_children()
                            .iter()
                            .rev()
                            .find_map(|child| child.get_xy(id))
                    }
                }
                _ => special
                    .get_children()
                    .iter()
                    .rev()
                    .find_map(|child| child.get_xy(id)),
            },
            _ => None,
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

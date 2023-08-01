use super::{visit::VisitUtils, *};
use crate::drag_and_drop::RawFileDropEvent;
use std::sync::atomic::Ordering;

impl RenderingTree {
    pub(crate) fn call_wheel_event(&self, raw_wheel_event: &RawWheelEvent) {
        self.call_event_of_screen(
            |CallEventOfScreenParam {
                 attach_event,
                 utils,
             }| {
                let Some(on_wheel) = attach_event.on_wheel.as_ref() else {
                    return CallEventOfScreenResult {
                        is_stop_propagation: false,
                    };
                };
                let is_mouse_in = utils.is_xy_in(raw_wheel_event.mouse_xy);
                if !is_mouse_in {
                    return CallEventOfScreenResult {
                        is_stop_propagation: false,
                    };
                }

                let is_stop_propagation = Arc::new(AtomicBool::new(false));
                on_wheel(WheelEvent {
                    delta_xy: raw_wheel_event.delta_xy,
                    id: raw_wheel_event.id.clone(),
                    mouse_local_xy: raw_wheel_event.mouse_xy,
                    is_stop_propagation: is_stop_propagation.clone(),
                });

                CallEventOfScreenResult {
                    is_stop_propagation: is_stop_propagation.load(Ordering::Relaxed),
                }
            },
        );
    }
    pub(crate) fn call_keyboard_event(
        &self,
        raw_keyboard_event: &RawKeyboardEvent,
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
                        callback(KeyboardEvent {
                            id: raw_keyboard_event.id.clone(),
                            code: raw_keyboard_event.code,
                            pressing_codes: raw_keyboard_event.pressing_codes.clone(),
                        });
                    }
                }
            }
            ControlFlow::Continue(())
        });
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
                let is_stop_propagation = Arc::new(AtomicBool::new(false));
                let mouse_event = MouseEvent {
                    id: raw_mouse_event.id.clone(),
                    global_xy: raw_mouse_event.xy,
                    local_xy: utils.to_local_xy(raw_mouse_event.xy),
                    pressing_buttons: raw_mouse_event.pressing_buttons.clone(),
                    button: raw_mouse_event.button,
                    event_type: mouse_event_type,
                    is_stop_propagation: is_stop_propagation.clone(),
                };

                if let Some(on_mouse) = &attach_event.on_mouse {
                    on_mouse(mouse_event.clone());
                }
                match is_mouse_in {
                    true => {
                        if let Some(in_func) = in_func {
                            in_func(mouse_event);
                        }
                    }
                    false => {
                        if let Some(out_func) = out_func {
                            out_func(mouse_event);
                        }
                    }
                }

                return CallEventOfScreenResult {
                    is_stop_propagation: is_stop_propagation.load(Ordering::Relaxed),
                };
            },
        );
    }
    pub(crate) fn call_file_drop_event(&self, file_drop_event: &RawFileDropEvent) {
        self.call_event_of_screen(
            |CallEventOfScreenParam {
                 attach_event,
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

                let is_stop_propagation = Arc::new(AtomicBool::new(false));

                let event = FileDropEvent {
                    global_xy: file_drop_event.global_xy,
                    local_xy: utils.to_local_xy(file_drop_event.global_xy),
                    files: file_drop_event.files.clone(),
                    is_stop_propagation: is_stop_propagation.clone(),
                };
                on_file_drop(event);

                return CallEventOfScreenResult {
                    is_stop_propagation: is_stop_propagation.load(Ordering::Relaxed),
                };
            },
        );
    }
}

struct CallEventOfScreenParam<'a> {
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

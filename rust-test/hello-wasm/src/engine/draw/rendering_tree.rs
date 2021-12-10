use crate::engine::{EngineContext, Xy};

use super::DrawCall;
use serde::Serialize;

#[derive(Serialize)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub on_click: Option<fn() -> ()>,
    // onClickOut?: MouseEventCallback;
    // onMouseMoveIn?: MouseEventCallback;
    // onMouseMoveOut?: MouseEventCallback;
    // onMouseIn?: () => void;
    // onMouseDown?: MouseEventCallback;
    // onMouseUp?: MouseEventCallback;
}
#[derive(Serialize)]
pub enum RenderingTree {
    Node(RenderingData),
    Children(Vec<RenderingTree>),
    Empty,
}

impl RenderingTree {
    pub fn draw(&self, engine_context: &EngineContext) {
        self.visit(&mut |rendering_data: &RenderingData| {
            rendering_data.draw_calls.iter().for_each(|draw_call| {
                draw_call.draw(engine_context);
            });
        });
    }
    pub fn visit(&self, callback: &mut dyn FnMut(&RenderingData)) {
        match self {
            RenderingTree::Children(ref children) => {
                for child in children {
                    child.visit(callback);
                }
            }
            RenderingTree::Node(node) => callback(&node),
            RenderingTree::Empty => {}
        }
    }
    pub fn call_on_click(&self, local_xy: &Xy<f32>) {
        self.visit(&mut |rendering_data: &RenderingData| {
            if let Some(on_click) = rendering_data.on_click {
                if rendering_data.is_inside(local_xy) {
                    on_click();
                }
            }
        });
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

#[cfg(test)]
mod tests {
    use crate::engine;

    use super::{RenderingData, RenderingTree};
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn visit_should_run_in_dfs_pre_order() {
        /*
            tree:
                 0
               /   \
              1     2
             / \   /
            3   4 5

            order:
            - 0, 1, 3, 4, 2, 5
        */

        let rendering_tree = RenderingTree::Children(vec![
            RenderingTree::Node(RenderingData {
                id: Some("0".to_string()),
                draw_calls: vec![],
                on_click: None,
            }),
            RenderingTree::Children(vec![
                RenderingTree::Node(RenderingData {
                    id: Some("1".to_string()),
                    draw_calls: vec![],
                    on_click: None,
                }),
                RenderingTree::Children(vec![
                    RenderingTree::Node(RenderingData {
                        id: Some("3".to_string()),
                        draw_calls: vec![],
                        on_click: None,
                    }),
                    RenderingTree::Node(RenderingData {
                        id: Some("4".to_string()),
                        draw_calls: vec![],
                        on_click: None,
                    }),
                ]),
            ]),
            RenderingTree::Children(vec![
                RenderingTree::Node(RenderingData {
                    id: Some("2".to_string()),
                    draw_calls: vec![],
                    on_click: None,
                }),
                RenderingTree::Children(vec![RenderingTree::Node(RenderingData {
                    id: Some("5".to_string()),
                    draw_calls: vec![],
                    on_click: None,
                })]),
            ]),
        ]);

        let mut visited_rendering_data_id_list: Vec<String> = vec![];
        rendering_tree.visit(&mut |rendering_data: &RenderingData| {
            visited_rendering_data_id_list.push(rendering_data.id.as_ref().unwrap().to_string());
        });

        assert_eq!(
            visited_rendering_data_id_list,
            vec!["0", "1", "3", "4", "2", "5"]
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn call_on_click_should_run_in_dfs_pre_order() {
        /*
            tree:
                 0
               /   \
              1     2

            screen:

                ┌─────── 0 ────────┐       ┌─────── 2 ────────┐
                │                  │       │                  │
                │                  │       │                  │
                │       ┌───────1────────┐ │                  │
                │       │                │ │                  │
                │       │  ●             │ │                  │
                └───────│                │ └──────────────────┘
                        │                │
                        │                │
                        └────────────────┘
            click on: ●
            call order: 0, 1
        */

        static mut ON_CLICK_CALLED_ID_LIST: Vec<String> = vec![];

        let rendering_tree = RenderingTree::Children(vec![
            engine::rect(engine::RectParam {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
                id: Some("0".to_string()),
                style: engine::RectStyle {
                    fill: None,
                    stroke: None,
                    round: None,
                },
                on_click: Some(|| unsafe {
                    ON_CLICK_CALLED_ID_LIST.push("0".to_string());
                }),
            }),
            RenderingTree::Children(vec![engine::rect(engine::RectParam {
                x: 50.0,
                y: 50.0,
                width: 100.0,
                height: 100.0,
                id: Some("1".to_string()),
                style: engine::RectStyle {
                    fill: None,
                    stroke: None,
                    round: None,
                },
                on_click: Some(|| unsafe {
                    ON_CLICK_CALLED_ID_LIST.push("1".to_string());
                }),
            })]),
            RenderingTree::Children(vec![engine::rect(engine::RectParam {
                x: 210.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
                id: Some("2".to_string()),
                style: engine::RectStyle {
                    fill: None,
                    stroke: None,
                    round: None,
                },
                on_click: Some(|| unsafe {
                    ON_CLICK_CALLED_ID_LIST.push("2".to_string());
                }),
            })]),
        ]);

        rendering_tree.call_on_click(&engine::Xy { x: 75.0, y: 75.0 });

        unsafe {
            assert_eq!(ON_CLICK_CALLED_ID_LIST, vec!["0", "1", "3", "5"]);
        };
    }
}

use crate::engine::EngineContext;

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
    pub fn draw<TState>(&self, engine_context: &EngineContext<TState>) {
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
}

#[cfg(test)]
mod tests {
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
}

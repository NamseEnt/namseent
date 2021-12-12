use crate::engine::{self, DrawCall, EngineContext, Xy};

use serde::Serialize;

use super::Translate;

#[derive(Serialize)]
pub struct RenderingData {
    pub draw_calls: Vec<DrawCall>,
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub on_click: Option<Box<dyn Fn()>>,
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
    Special(Translate),
    Empty,
}

impl RenderingTree {
    pub fn draw(&self, engine_context: &EngineContext) {
        match self {
            RenderingTree::Children(ref children) => {
                for child in children {
                    child.draw(engine_context);
                }
            }
            RenderingTree::Node(rendering_data) => {
                rendering_data.draw_calls.iter().for_each(|draw_call| {
                    draw_call.draw(engine_context);
                });
            }
            RenderingTree::Special(special) => {
                engine_context
                    .surface
                    .canvas()
                    .translate(special.x, special.y);

                for child in &special.rendering_tree {
                    child.draw(engine_context);
                }

                engine_context
                    .surface
                    .canvas()
                    .translate(-special.x, -special.y);
            }
            RenderingTree::Empty => {}
        }
    }
    pub fn call_on_click(&self, local_xy: &Xy<f32>) {
        match self {
            RenderingTree::Children(ref children) => {
                for child in children {
                    child.call_on_click(local_xy);
                }
            }
            RenderingTree::Node(rendering_data) => {
                if let Some(on_click) = &rendering_data.on_click {
                    if rendering_data.is_inside(local_xy) {
                        on_click();
                    }
                }
            }
            RenderingTree::Special(special) => {
                let xy = Xy {
                    x: local_xy.x - special.x,
                    y: local_xy.y - special.y,
                };
                for child in &special.rendering_tree {
                    child.call_on_click(&xy);
                }
            }
            RenderingTree::Empty => {}
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

#[cfg(test)]
mod tests {
    use crate::engine;

    use super::{RenderingData, RenderingTree};
    use wasm_bindgen_test::*;

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
                on_click: Some(Box::new(move || unsafe {
                    ON_CLICK_CALLED_ID_LIST.push("0".to_string());
                })),
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
                on_click: Some(Box::new(move || unsafe {
                    ON_CLICK_CALLED_ID_LIST.push("1".to_string());
                })),
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
                on_click: Some(Box::new(move || unsafe {
                    ON_CLICK_CALLED_ID_LIST.push("2".to_string());
                })),
            })]),
        ]);

        rendering_tree.call_on_click(&engine::Xy { x: 75.0, y: 75.0 });

        unsafe {
            assert_eq!(ON_CLICK_CALLED_ID_LIST, vec!["0", "1", "3", "5"]);
        };
    }
}

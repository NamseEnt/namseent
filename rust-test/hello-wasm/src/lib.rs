mod engine;
mod utils;
use crate::engine::{RectParam, RectStroke, RectStyle};
use std::any::{Any, TypeId};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn test(value: &JsValue);
}

enum StateActions {
    AddTodo(String),
    RemoveTodo(usize),
}
struct State {
    next_todo_id: usize,
    todo_list: Vec<Todo>,
    text_input: engine::TextInput,
}

impl engine::Update for State {
    fn update(&mut self, event: &dyn Any) {
        match event.downcast_ref::<StateActions>() {
            Some(StateActions::AddTodo(text)) => {
                engine::log(format!("AddTodo: {}", text));
                self.todo_list.push(Todo {
                    id: self.next_todo_id,
                    text: text.clone(),
                });
                self.next_todo_id += 1;
            }
            Some(StateActions::RemoveTodo(id)) => {
                self.todo_list.retain(|todo| todo.id != *id);
            }
            None => {}
        }

        match event.downcast_ref::<Box<StateActions>>() {
            Some(a) => {
                engine::log(format!("AddTodo in box"));
            }
            None => {}
        }

        self.text_input.update(event);
    }
}

impl engine::Render for State {
    fn render(&self) -> engine::RenderingTree {
        let add_button = render![
            engine::rect(RectParam {
                x: 100.0,
                y: 100.0,
                width: 100.0,
                height: 100.0,
                id: None,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: engine::Color {
                            r: 128,
                            g: 0,
                            b: 0,
                            a: 255,
                        },
                        width: 1.0,
                        border_position: engine::BorderPosition::Inside,
                    }),
                    ..Default::default()
                },
                on_click: Some(Box::new(|xy| {
                    engine::log("clicked".to_string());
                    engine::event::send(Box::new(StateActions::AddTodo("hi".to_string())));
                })),
            }),
            engine::text(engine::TextParam {
                x: 100.0,
                y: 100.0,
                align: engine::TextAlign::Left,
                baseline: engine::TextBaseline::Top,
                font_type: engine::FontType {
                    font_weight: engine::FontWeight::_400,
                    language: engine::Language::Ko,
                    serif: false,
                    size: 16,
                },
                style: engine::TextStyle {
                    color: engine::Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                    background: None,
                    border: None,
                    drop_shadow: None,
                },
                text: format!("Add Todo - {}", self.next_todo_id),
            })
        ];

        let todo_list = engine::RenderingTree::Children(
            self.todo_list
                .iter()
                .enumerate()
                .map(|(index, todo)| {
                    engine::translate(
                        100.0,
                        200.0 + 100.0 * index as f32,
                        render![
                            engine::rect(engine::RectParam {
                                x: 0.0,
                                y: 0.0,
                                width: 100.0,
                                height: 100.0,
                                id: None,
                                style: RectStyle {
                                    stroke: Some(RectStroke {
                                        color: engine::Color {
                                            r: 0,
                                            g: 128,
                                            b: 0,
                                            a: 255,
                                        },
                                        width: 1.0,
                                        border_position: engine::BorderPosition::Inside,
                                    }),
                                    ..Default::default()
                                },
                                on_click: Some(Box::new(move |xy| {
                                    engine::log(format!("clicked {}", index));
                                })),
                            }),
                            engine::text(engine::TextParam {
                                x: 0.0,
                                y: 0.0,
                                align: engine::TextAlign::Left,
                                baseline: engine::TextBaseline::Top,
                                font_type: engine::FontType {
                                    font_weight: engine::FontWeight::_400,
                                    language: engine::Language::Ko,
                                    serif: false,
                                    size: 16,
                                },
                                style: engine::TextStyle {
                                    color: engine::Color {
                                        r: 0,
                                        g: 0,
                                        b: 0,
                                        a: 255,
                                    },
                                    background: None,
                                    border: None,
                                    drop_shadow: None,
                                },
                                text: format!("{}: {}", todo.id, todo.text),
                            })
                        ],
                    )
                })
                .collect::<Vec<_>>(),
        );

        render![add_button, todo_list, self.text_input.render()]
    }
}

struct Todo {
    id: usize,
    text: String,
}

// impl engine::Update for Todo {
//     fn update(&self, event: &dyn Any) -> Self {
//         Self { value: self.value }
//     }
// }

#[wasm_bindgen]
pub async fn greet() {
    engine::start(State {
        next_todo_id: 0,
        todo_list: vec![],
        text_input: engine::TextInput::new(
            "abcdefghijklmnop".to_string(),
            300.0,
            300.0,
            100.0,
            50.0,
            engine::Color {
                r: 200,
                g: 200,
                b: 200,
                a: 255,
            },
            engine::Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            1.0,
            engine::TextAlign::Left,
            engine::TextBaseline::Top,
            engine::FontType {
                font_weight: engine::FontWeight::_400,
                language: engine::Language::Ko,
                serif: false,
                size: 16,
            },
            engine::TextStyle {
                background: None,
                border: None,
                color: engine::Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                drop_shadow: None,
            },
        ),
    })
    .await;
}

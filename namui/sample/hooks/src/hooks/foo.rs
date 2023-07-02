use super::component::{Component, WireClosures};
use crate::hooks::{component::ComponentProps, effect, render::Render, Atom, Button, List};
use namui::prelude::*;

pub static LIST_ATOM: Atom<Vec<i32>> = Atom::new(|| vec![1]);

#[derive(Debug, Clone, PartialEq)]
pub struct Foo {}

impl ComponentProps for Foo {
    fn render(&self, render: Render) -> Render {
        // let (list, set_list) = state::<Vec<i32>>(vec![1]);
        let (list, set_list) = LIST_ATOM.state();

        effect((), || {
            namui::spawn_local(async move {
                loop {
                    namui::time::delay(1.sec()).await;
                    set_list.invoke(|list| {
                        list.push(list.len() as i32);
                    })
                }
            })
        });

        let on_click_red_rect = closure(move |_| {
            set_list.invoke(|list| {
                list.push(0);
            })
        });

        let on_click_plus = closure(move |_| {
            set_list.invoke(|list| {
                list.push(0);
            })
        });

        render
            .add(
                List::new()
                    .add(Button {
                        text: format!("+"),
                        on_click: on_click_plus,
                    })
                    .add(List::from_iter(list.iter().enumerate().map(
                        |(index, value)| Button {
                            text: format!("{value}"),
                            on_click: closure(move |_| {
                                set_list.invoke(|list| {
                                    list[index] += 1;
                                })
                            }),
                        },
                    ))),
            )
            .add(
                rect(namui::RectParam {
                    rect: Rect::Xywh {
                        x: 100.px(),
                        y: 200.px(),
                        width: 100.px(),
                        height: 100.px(),
                    },
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::RED,
                            width: 1.px(),
                            border_position: BorderPosition::Inside,
                        }),
                        fill: None,
                        round: None,
                    },
                })
                .attach_event(|build| {
                    build.on_mouse_down_in(on_click_red_rect);
                }),
            )
    }
}

impl WireClosures for Foo {
    fn wire_closures(&self, _to: &dyn Component) {}
}

impl Component for Foo {}

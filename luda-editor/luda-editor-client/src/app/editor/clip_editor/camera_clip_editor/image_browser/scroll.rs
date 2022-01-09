use crate::app::editor::events::EditorEvent;
use namui::prelude::*;

#[derive(Debug)]
pub struct Scroll {
    pub scroll_y: f32,
}

pub struct ScrollProps {
    pub x: f32,
    pub y: f32,
    pub scroll_bar_width: f32,
    pub inner_width: f32,
    pub inner_height: f32,
    pub height: f32,
    pub inner_rendering_tree: RenderingTree,
}

impl Scroll {
    pub fn new() -> Self {
        Self { scroll_y: 0.0 }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::ScrolledEvent { scroll_y } => {
                    self.scroll_y = *scroll_y;
                }
                _ => {}
            }
        }
    }
    pub fn render(
        &self,
        ScrollProps {
            x,
            y,
            scroll_bar_width,
            inner_width,
            inner_height,
            height,
            inner_rendering_tree,
        }: ScrollProps,
    ) -> RenderingTree {
        let scroll_y = num::clamp(self.scroll_y, 0.0, (0.0_f32).max(inner_height - height));

        let inner = namui::clip(
            namui::PathBuilder::new().add_rect(&namui::LtrbRect {
                left: 0.0,
                top: 0.0,
                right: inner_width,
                bottom: height,
            }),
            namui::ClipOp::Intersect,
            namui::translate(0.0, -scroll_y, inner_rendering_tree),
        );

        let scroll_bar_handle_height = height * (height / inner_height);

        let scroll_bar_y = (height - scroll_bar_handle_height) * scroll_y / (inner_height - height);

        let scroll_bar = match inner_height > height {
            true => render![
                rect(RectParam {
                    x: inner_width,
                    y: 0.0,
                    width: scroll_bar_width,
                    height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            width: 1.0,
                            border_position: BorderPosition::Inside,
                            color: Color::BLACK,
                        }),
                        fill: Some(RectFill {
                            color: Color::WHITE,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                rect(RectParam {
                    x: inner_width,
                    y: scroll_bar_y,
                    width: scroll_bar_width,
                    height: scroll_bar_handle_height,
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: Color::gary_scale_f01(0.5),
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            false => RenderingTree::Empty,
        };

        let whole_rect = rect(RectParam {
            x: 0.0,
            y: 0.0,
            width: inner_width + scroll_bar_width,
            height,
            style: RectStyle {
                stroke: Some(RectStroke {
                    width: 1.0,
                    border_position: BorderPosition::Middle,
                    color: Color::BLACK,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(|builder| {
            builder.on_wheel(Box::new(move |xy| {
                let next_scroll_y =
                    num::clamp(scroll_y + xy.y, 0.0, (0.0_f32).max(inner_height - height));

                namui::event::send(Box::new(EditorEvent::ScrolledEvent {
                    scroll_y: next_scroll_y,
                }));
            }))
        });

        namui::translate(x, y, render![whole_rect, inner, scroll_bar])
    }
}

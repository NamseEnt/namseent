use crate::app::editor::events::EditorEvent;
use namui::prelude::*;

#[derive(Debug)]
pub struct Scroll {
    pub scroll_y: Px,
}

pub struct ScrollProps {
    pub x: Px,
    pub y: Px,
    pub scroll_bar_width: Px,
    pub inner_width: Px,
    pub inner_height: Px,
    pub height: Px,
    pub inner_rendering_tree: RenderingTree,
}

impl Scroll {
    pub fn new() -> Self {
        Self { scroll_y: px(0.0) }
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
        let scroll_y = num::clamp(self.scroll_y, px(0.0), px(0.0).max(inner_height - height));

        let inner = namui::clip(
            namui::PathBuilder::new().add_rect(namui::Rect::Ltrb {
                left: px(0.0),
                top: px(0.0),
                right: inner_width,
                bottom: height,
            }),
            namui::ClipOp::Intersect,
            namui::translate(px(0.0), -scroll_y, inner_rendering_tree),
        );

        let scroll_bar_handle_height = height * (height / inner_height);

        let scroll_bar_y =
            (height - scroll_bar_handle_height) * (scroll_y / (inner_height - height));

        let scroll_bar = match inner_height > height {
            true => render([
                rect(RectParam {
                    rect: Rect::Xywh {
                        x: inner_width,
                        y: px(0.0),
                        width: scroll_bar_width,
                        height,
                    },
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            width: px(1.0),
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
                    rect: Rect::Xywh {
                        x: inner_width,
                        y: scroll_bar_y,
                        width: scroll_bar_width,
                        height: scroll_bar_handle_height,
                    },
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: Color::grayscale_f01(0.5),
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ]),
            false => RenderingTree::Empty,
        };
        let whole_rect = rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: inner_width + scroll_bar_width,
                height,
            },
            style: RectStyle {
                stroke: Some(RectStroke {
                    width: px(1.0),
                    border_position: BorderPosition::Middle,
                    color: Color::BLACK,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let width = inner_width + scroll_bar_width;
            let height = height;
            builder.on_wheel(move |event| {
                let mouse_position = namui::mouse::position();
                let whole_rect_xy = event
                    .namui_context
                    .get_rendering_tree_xy(event.target)
                    .expect("failed to get whole_rect xy");

                let is_mouse_in_timeline = mouse_position.x >= whole_rect_xy.x
                    && mouse_position.x <= whole_rect_xy.x + width
                    && mouse_position.y >= whole_rect_xy.y
                    && mouse_position.y <= whole_rect_xy.y + height;
                if !is_mouse_in_timeline {
                    return;
                }

                let next_scroll_y = num::clamp(
                    scroll_y + px(event.delta_xy.y),
                    px(0.0),
                    px(0.0).max(inner_height - height),
                );

                namui::event::send(EditorEvent::ScrolledEvent {
                    scroll_y: next_scroll_y,
                });
            });
        });

        namui::translate(x, y, render([whole_rect, inner, scroll_bar]))
    }
}

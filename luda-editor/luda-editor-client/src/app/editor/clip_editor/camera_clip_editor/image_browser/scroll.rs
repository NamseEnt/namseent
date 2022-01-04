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
        let whole_rect_id = namui::nanoid();
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
        .with_id(&whole_rect_id)
        .attach_event(move |builder| {
            let width = inner_width + scroll_bar_width;
            let height = height;
            let whole_rect_id = whole_rect_id.clone();
            builder.on_wheel(Box::new(move |event| {
                let managers = namui::managers();

                let mouse_manager = &managers.mouse_manager;
                let mouse_position = mouse_manager.mouse_position();
                let whole_rect_xy = event
                    .namui_context
                    .get_rendering_tree_xy(&whole_rect_id)
                    .unwrap();

                let is_mouse_in_timeline = mouse_position.x as f32 >= whole_rect_xy.x
                    && mouse_position.x as f32 <= whole_rect_xy.x + width
                    && mouse_position.y as f32 >= whole_rect_xy.y
                    && mouse_position.y as f32 <= whole_rect_xy.y + height;
                if !is_mouse_in_timeline {
                    return;
                }

                let next_scroll_y = num::clamp(
                    scroll_y + event.delta_xy.y,
                    0.0,
                    (0.0_f32).max(inner_height - height),
                );

                namui::event::send(Box::new(EditorEvent::ScrolledEvent {
                    scroll_y: next_scroll_y,
                }));
            }))
        });

        namui::translate(x, y, render![whole_rect, inner, scroll_bar])
    }
}

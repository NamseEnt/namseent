use super::{CanvasEvent, Tool, ToolType};
use crate::app::cropper::selection::Selection;
use namui::prelude::*;
use std::sync::Arc;

pub struct CanvasProps<'a> {
    pub wh: Wh<Px>,
    pub selection_list: &'a Vec<Selection>,
}

pub struct Canvas {
    scale: f32,
    offset: Xy<Px>,
    image: Arc<Image>,
    tool: Tool,
    canvas_drag_state: CanvasDragState,
}
impl Canvas {
    pub fn new(image: Arc<Image>) -> Self {
        Self {
            scale: 1.0,
            offset: Xy {
                x: px(0.0),
                y: px(0.0),
            },
            image,
            tool: Tool::new(),
            canvas_drag_state: CanvasDragState::None,
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<CanvasEvent>() {
            match &event {
                CanvasEvent::Scrolled { offset } => self.offset = offset.clone(),
                CanvasEvent::Zoomed { offset, scale } => {
                    self.offset = offset.clone();
                    self.scale = scale.clone();
                }
                CanvasEvent::DragStarted(drag_state) => {
                    self.canvas_drag_state = drag_state.clone();
                }
                _ => (),
            }
        }
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match &event {
                NamuiEvent::MouseUp(_) => {
                    self.canvas_drag_state = CanvasDragState::None;
                }
                NamuiEvent::KeyDown(event) => match event.code {
                    namui::Code::Digit1 | namui::Code::KeyH => {
                        self.change_tool(ToolType::Hand);
                    }

                    namui::Code::Digit2 | namui::Code::KeyM => {
                        self.change_tool(ToolType::RectSelection);
                    }

                    namui::Code::Digit3 | namui::Code::KeyL => {
                        self.change_tool(ToolType::PolySelection);
                    }

                    namui::Code::Digit4 | namui::Code::KeyZ => {
                        self.change_tool(ToolType::Zoom);
                    }

                    namui::Code::Space => {
                        if namui::keyboard::any_code_press([Code::ControlLeft]) {
                            self.tool.set_secondary_tool_type(ToolType::Zoom)
                        } else {
                            self.tool.set_secondary_tool_type(ToolType::Hand)
                        }
                    }
                    _ => (),
                },
                NamuiEvent::KeyUp(event) => match event.code {
                    namui::Code::Space => self.tool.unset_secondary_tool_type(),
                    _ => (),
                },
                _ => (),
            }
        }
        self.tool.update(event);
    }

    pub fn render(&self, props: CanvasProps) -> RenderingTree {
        let image_size = self.image.size();
        let offset = self.offset.clone();
        let scale = self.scale.clone();
        let current_tool_type = self.tool.get_current_tool_type().clone();
        let canvas_drag_state = self.canvas_drag_state.clone();
        let canvas_wh = props.wh.clone();

        let scaled_image_size = Wh {
            width: image_size.width * self.scale,
            height: image_size.height * self.scale,
        };
        let scaled_offset = Xy {
            x: offset.x * self.scale,
            y: offset.y * self.scale,
        };
        render([
            render_background(props.wh).attach_event(|builder| {
                builder
                    .on_wheel(move |event| {
                        let mouse_position = namui::mouse::position();
                        let canvas_xy = event
                            .namui_context
                            .get_rendering_tree_xy(event.target)
                            .expect("failed to get canvas xy");
                        let local_mouse_position = mouse_position - canvas_xy;
                        let is_mouse_in_canvas = !(local_mouse_position.x < px(0.0)
                            || local_mouse_position.x > canvas_wh.width
                            || local_mouse_position.y < px(0.0)
                            || local_mouse_position.y > canvas_wh.height);

                        if !is_mouse_in_canvas {
                            return;
                        }

                        if namui::keyboard::any_code_press([namui::Code::ControlLeft]) {
                            zoom(
                                event.delta_xy,
                                offset,
                                local_mouse_position,
                                canvas_wh,
                                image_size,
                                scale,
                            )
                        } else if namui::keyboard::any_code_press([namui::Code::ShiftLeft]) {
                            scroll(
                                event.delta_xy.into_type(),
                                offset,
                                canvas_wh,
                                image_size,
                                scale,
                            )
                        } else {
                            scroll(
                                event.delta_xy.into_type(),
                                offset,
                                canvas_wh,
                                image_size,
                                scale,
                            )
                        }
                    })
                    .on_mouse_down(move |event| {
                        if event.pressing_buttons.contains(&namui::MouseButton::Left) {
                            let local_xy_on_image = Xy {
                                x: -offset.x + event.local_xy.x / scale,
                                y: -offset.y + event.local_xy.y / scale,
                            };
                            match current_tool_type {
                                ToolType::Hand => {
                                    namui::event::send(CanvasEvent::DragStarted(
                                        CanvasDragState::DraggingHand {
                                            image_anchor_point: local_xy_on_image.clone(),
                                        },
                                    ));
                                }
                                ToolType::Zoom => namui::event::send(CanvasEvent::DragStarted(
                                    CanvasDragState::DraggingZoom {
                                        canvas_anchor_point: event.local_xy.clone(),
                                        initial_offset: offset.clone(),
                                        initial_scale: scale.clone(),
                                        initial_mouse_xy: event.global_xy.clone(),
                                    },
                                )),
                                _ => (),
                            };
                            namui::event::send(CanvasEvent::LeftMouseDownInCanvas {
                                position: local_xy_on_image,
                                tool_type: current_tool_type,
                            });
                        }
                    })
                    .on_mouse_move_in(move |event| {
                        let local_xy_on_image = Xy {
                            x: -offset.x + event.local_xy.x / scale,
                            y: -offset.y + event.local_xy.y / scale,
                        };
                        match canvas_drag_state {
                            CanvasDragState::DraggingHand { image_anchor_point } => {
                                handle_hand_tool_drag(
                                    image_anchor_point,
                                    local_xy_on_image,
                                    offset,
                                    canvas_wh,
                                    image_size,
                                    scale,
                                )
                            }
                            CanvasDragState::DraggingZoom {
                                canvas_anchor_point: image_anchor_point,
                                initial_offset,
                                initial_scale,
                                initial_mouse_xy,
                            } => handle_zoom_tool_drag(
                                image_anchor_point,
                                initial_offset,
                                initial_scale,
                                initial_mouse_xy,
                                event.global_xy.clone(),
                                canvas_wh,
                                image_size,
                            ),
                            _ => (),
                        }
                        namui::event::send(CanvasEvent::MouseMoveInCanvas(local_xy_on_image))
                    });
            }),
            clip(
                namui::PathBuilder::new().add_rect(Rect::Ltrb {
                    left: px(0.0),
                    top: px(0.0),
                    right: props.wh.width,
                    bottom: props.wh.height,
                }),
                namui::ClipOp::Intersect,
                translate(
                    scaled_offset.x,
                    scaled_offset.y,
                    render([
                        image(ImageParam {
                            rect: Rect::Xywh {
                                x: px(0.0),
                                y: px(0.0),
                                width: scaled_image_size.width,
                                height: scaled_image_size.height,
                            },
                            source: namui::ImageSource::Image(self.image.clone()),
                            style: ImageStyle {
                                fit: ImageFit::Fill,
                                paint_builder: None,
                            },
                        }),
                        render(
                            props
                                .selection_list
                                .into_iter()
                                .map(|selection| selection.render(scale)),
                        ),
                    ]),
                ),
            ),
            self.tool.render_cursor_icon(),
        ])
    }

    fn change_tool(&mut self, to: ToolType) {
        self.canvas_drag_state = CanvasDragState::None;
        self.tool.change_tool_type(to);
    }
}

#[derive(Clone, Copy)]
pub enum CanvasDragState {
    None,
    DraggingHand {
        image_anchor_point: Xy<Px>,
    },
    DraggingZoom {
        canvas_anchor_point: Xy<Px>,
        initial_offset: Xy<Px>,
        initial_scale: f32,
        initial_mouse_xy: Xy<Px>,
    },
}

fn render_background(wh: Wh<Px>) -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: wh.width,
            height: wh.height,
        },
        style: RectStyle {
            stroke: None,
            fill: Some(RectFill {
                color: Color::from_u8(36, 37, 42, 255),
            }),
            round: None,
        },
    })
}

fn handle_hand_tool_drag(
    image_anchor_point: Xy<Px>,
    moved_to: Xy<Px>,
    offset: Xy<Px>,
    canvas_wh: Wh<Px>,
    image_size: Wh<Px>,
    scale: f32,
) {
    let scaled_delta_xy = Xy {
        x: (image_anchor_point.x - moved_to.x) * scale,
        y: (image_anchor_point.y - moved_to.y) * scale,
    };
    scroll(scaled_delta_xy, offset, canvas_wh, image_size, scale)
}

fn handle_zoom_tool_drag(
    canvas_anchor_point: Xy<Px>,
    initial_offset: Xy<Px>,
    initial_scale: f32,
    initial_mouse_xy: Xy<Px>,
    last_mouse_xy: Xy<Px>,
    canvas_wh: Wh<Px>,
    image_size: Wh<Px>,
) {
    const DRAG_ZOOM_MULTIPLIER: f32 = 5.0;
    let multiplied_reverse_delta_xy = Xy {
        x: (initial_mouse_xy.x - last_mouse_xy.x).as_f32() * DRAG_ZOOM_MULTIPLIER,
        y: (initial_mouse_xy.y - last_mouse_xy.y).as_f32() * DRAG_ZOOM_MULTIPLIER,
    };
    zoom(
        multiplied_reverse_delta_xy,
        initial_offset,
        canvas_anchor_point,
        canvas_wh,
        image_size,
        initial_scale,
    )
}

fn scroll(delta_xy: Xy<Px>, offset: Xy<Px>, canvas_wh: Wh<Px>, image_size: Wh<Px>, scale: f32) {
    let scaled_delta_xy = Xy {
        x: delta_xy.x / scale,
        y: delta_xy.y / scale,
    };
    let new_offset = clamp_offset_xy(offset - scaled_delta_xy, canvas_wh, image_size, scale);
    namui::event::send(CanvasEvent::Scrolled { offset: new_offset })
}

fn zoom(
    delta_xy: Xy<f32>,
    offset: Xy<Px>,
    canvas_anchor_point: Xy<Px>,
    canvas_wh: Wh<Px>,
    image_size: Wh<Px>,
    scale: f32,
) {
    const ZOOM_MULTIPLIER: f32 = 1.0 / 1000.0;
    let delta_scale = -(delta_xy.x + delta_xy.y) * scale * ZOOM_MULTIPLIER;
    let new_scale = clamp_scale(scale + delta_scale, canvas_wh, image_size);
    let scale_factor = (new_scale - scale) / (new_scale * scale);
    let diff_offset = Xy {
        x: canvas_wh.width * scale_factor * (canvas_anchor_point.x / canvas_wh.width),
        y: canvas_wh.height * scale_factor * (canvas_anchor_point.y / canvas_wh.height),
    };
    let new_offset = clamp_offset_xy(offset - diff_offset, canvas_wh, image_size, new_scale);
    namui::event::send(CanvasEvent::Zoomed {
        offset: new_offset,
        scale: new_scale,
    })
}

fn clamp_offset_xy(offset_xy: Xy<Px>, canvas_wh: Wh<Px>, image_wh: Wh<Px>, scale: f32) -> Xy<Px> {
    let max_diff = Wh {
        width: canvas_wh.width / scale / 2.0,
        height: canvas_wh.height / scale / 2.0,
    };
    Xy {
        x: offset_xy
            .x
            .clamp(-image_wh.width + max_diff.width, max_diff.width),
        y: offset_xy
            .y
            .clamp(-image_wh.height + max_diff.height, max_diff.height),
    }
}

fn clamp_scale(scale: f32, canvas_wh: Wh<Px>, image_wh: Wh<Px>) -> f32 {
    let ratio = Xy {
        x: image_wh.width / canvas_wh.width,
        y: image_wh.height / canvas_wh.height,
    };
    let max_ratio = ratio.x.max(ratio.y);
    let minimum_scale = 0.5 / max_ratio;
    scale.max(minimum_scale)
}

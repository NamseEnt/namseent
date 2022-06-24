use super::{CanvasEvent, Tool, ToolType};
use crate::app::cropper::selection::Selection;
use namui::{
    clip, image, render, translate, Color, Image, ImageFit, ImageParam, ImageStyle, NamuiEvent,
    RectFill, RectParam, RectStyle, RenderingTree, Wh, Xy,
};
use std::sync::Arc;

pub struct CanvasProps<'a> {
    pub wh: Wh<f32>,
    pub selection_list: &'a Vec<Selection>,
}

pub struct Canvas {
    scale: f32,
    offset: Xy<f32>,
    image: Arc<Image>,
    tool: Tool,
    hand_tool_pushed_position: Option<Xy<f32>>,
}
impl Canvas {
    pub fn new(image: Arc<Image>) -> Self {
        Self {
            scale: 1.0,
            offset: Xy { x: 0.0, y: 0.0 },
            image,
            tool: Tool::new(),
            hand_tool_pushed_position: None,
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
                _ => (),
            }
        }
        if let Some(event) = event.downcast_ref::<CanvasEvent>() {
            match &event {
                CanvasEvent::LeftMouseDownInCanvas {
                    position,
                    tool_type,
                } => match tool_type {
                    ToolType::Hand => self.hand_tool_pushed_position = Some(position.clone()),
                    _ => (),
                },
                _ => (),
            }
        }
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match &event {
                NamuiEvent::MouseUp(_) => {
                    self.handle_hand_tool_up();
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
        let hand_tool_pushed_position = self.hand_tool_pushed_position.clone();

        let scaled_image_size = Wh {
            width: image_size.width * self.scale,
            height: image_size.height * self.scale,
        };
        let scaled_offset = Xy {
            x: offset.x * self.scale,
            y: offset.y * self.scale,
        };
        render([
            render_background(&props.wh).attach_event(|builder| {
                builder
                    .on_wheel(move |event| {
                        let offset = offset.clone();
                        let scale = scale.clone();
                        let canvas_wh = props.wh.clone();
                        let image_size = image_size.clone();

                        let managers = namui::managers();
                        let mouse_manager = &managers.mouse_manager;
                        let mouse_position = mouse_manager.mouse_position();
                        let canvas_xy = event
                            .namui_context
                            .get_rendering_tree_xy(event.target)
                            .expect("failed to get canvas xy");
                        let local_mouse_position = Xy {
                            x: mouse_position.x as f32 - canvas_xy.x,
                            y: mouse_position.y as f32 - canvas_xy.y,
                        };
                        let is_mouse_in_canvas = !(local_mouse_position.x < 0.0
                            || local_mouse_position.x > canvas_wh.width
                            || local_mouse_position.y < 0.0
                            || local_mouse_position.y > canvas_wh.height);

                        if !is_mouse_in_canvas {
                            return;
                        }

                        let keyboard_manager = &managers.keyboard_manager;
                        if keyboard_manager.any_code_press([namui::Code::ControlLeft]) {
                            zoom(
                                event.delta_xy,
                                offset,
                                local_mouse_position,
                                canvas_wh,
                                image_size,
                                scale,
                            )
                        } else if keyboard_manager.any_code_press([namui::Code::ShiftLeft]) {
                            scroll(
                                Xy {
                                    x: event.delta_xy.y,
                                    y: event.delta_xy.x,
                                },
                                offset,
                                canvas_wh,
                                image_size,
                                scale,
                            )
                        } else {
                            scroll(event.delta_xy, offset, canvas_wh, image_size, scale)
                        }
                    })
                    .on_mouse_down(move |event| {
                        if event.pressing_buttons.contains(&namui::MouseButton::Left) {
                            let current_tool_type = current_tool_type.clone();
                            let offset = offset.clone();
                            let scale = scale.clone();
                            let local_xy_on_image = Xy {
                                x: -offset.x + event.local_xy.x / scale,
                                y: -offset.y + event.local_xy.y / scale,
                            };
                            namui::event::send(CanvasEvent::LeftMouseDownInCanvas {
                                position: local_xy_on_image,
                                tool_type: current_tool_type,
                            })
                        }
                    })
                    .on_mouse_move_in(move |event| {
                        let hand_tool_pushed_position = hand_tool_pushed_position.clone();
                        let canvas_wh = props.wh.clone();
                        let image_size = image_size.clone();
                        let current_tool_type = current_tool_type.clone();
                        let offset = offset.clone();
                        let scale = scale.clone();
                        let local_xy_on_image = Xy {
                            x: -offset.x + event.local_xy.x / scale,
                            y: -offset.y + event.local_xy.y / scale,
                        };
                        match current_tool_type {
                            ToolType::Hand => handle_hand_tool_drag(
                                hand_tool_pushed_position,
                                local_xy_on_image,
                                offset,
                                canvas_wh,
                                image_size,
                                scale,
                            ),
                            _ => (),
                        };
                        namui::event::send(CanvasEvent::MouseMoveInCanvas(local_xy_on_image))
                    })
            }),
            clip(
                namui::PathBuilder::new().add_rect(&namui::LtrbRect {
                    left: 0.0,
                    top: 0.0,
                    right: props.wh.width,
                    bottom: props.wh.height,
                }),
                namui::ClipOp::Intersect,
                translate(
                    scaled_offset.x,
                    scaled_offset.y,
                    render([
                        image(ImageParam {
                            xywh: namui::XywhRect {
                                x: 0.0,
                                y: 0.0,
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
        self.hand_tool_pushed_position = None;
        self.tool.change_tool_type(to);
    }

    fn handle_hand_tool_up(&mut self) {
        self.hand_tool_pushed_position = None;
    }
}

fn render_background(wh: &Wh<f32>) -> RenderingTree {
    namui::rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: wh.width,
        height: wh.height,
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
    moved_from: Option<Xy<f32>>,
    moved_to: Xy<f32>,
    offset: Xy<f32>,
    canvas_wh: Wh<f32>,
    image_size: Wh<f32>,
    scale: f32,
) {
    if let Some(last_position) = moved_from {
        let scaled_delta_xy = Xy {
            x: (last_position.x - moved_to.x) * scale,
            y: (last_position.y - moved_to.y) * scale,
        };
        scroll(scaled_delta_xy, offset, canvas_wh, image_size, scale)
    }
}

fn scroll(delta_xy: Xy<f32>, offset: Xy<f32>, canvas_wh: Wh<f32>, image_size: Wh<f32>, scale: f32) {
    let scaled_delta_xy = Xy {
        x: delta_xy.x / scale,
        y: delta_xy.y / scale,
    };
    let new_offset = clamp_offset_xy(offset - scaled_delta_xy, canvas_wh, image_size, scale);
    namui::event::send(CanvasEvent::Scrolled { offset: new_offset })
}

fn zoom(
    delta_xy: Xy<f32>,
    offset: Xy<f32>,
    local_mouse_position: Xy<f32>,
    canvas_wh: Wh<f32>,
    image_size: Wh<f32>,
    scale: f32,
) {
    const ZOOM_MULTIPLIER: f32 = 1.0 / 1000.0;
    let delta_scale = -(delta_xy.x + delta_xy.y) * scale * ZOOM_MULTIPLIER;
    let new_scale = clamp_scale(scale + delta_scale, canvas_wh, image_size);
    let scale_factor = (new_scale - scale) / (new_scale * scale);
    let diff_offset = Xy {
        x: canvas_wh.width * scale_factor * (local_mouse_position.x / canvas_wh.width),
        y: canvas_wh.height * scale_factor * (local_mouse_position.y / canvas_wh.height),
    };
    let new_offset = clamp_offset_xy(offset - diff_offset, canvas_wh, image_size, new_scale);
    namui::event::send(CanvasEvent::Zoomed {
        offset: new_offset,
        scale: new_scale,
    })
}

fn clamp_offset_xy(
    offset_xy: Xy<f32>,
    canvas_wh: Wh<f32>,
    image_wh: Wh<f32>,
    scale: f32,
) -> Xy<f32> {
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

fn clamp_scale(scale: f32, canvas_wh: Wh<f32>, image_wh: Wh<f32>) -> f32 {
    let ratio = Xy {
        x: image_wh.width / canvas_wh.width,
        y: image_wh.height / canvas_wh.height,
    };
    let max_ratio = ratio.x.max(ratio.y);
    let minimum_scale = 0.5 / max_ratio;
    scale.max(minimum_scale)
}

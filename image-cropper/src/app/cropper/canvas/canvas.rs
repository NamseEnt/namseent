use super::CanvasEvent;
use namui::{
    clip, image, render, Color, Image, ImageFit, ImageParam, ImageStyle, RectFill, RectParam,
    RectStyle, RenderingTree, Wh, Xy,
};
use std::sync::Arc;

pub struct CanvasProps {
    pub wh: Wh<f32>,
}

pub struct Canvas {
    scale: f32,
    offset: Xy<f32>,
    image: Arc<Image>,
}
impl Canvas {
    pub fn new(image: Arc<Image>) -> Self {
        Self {
            scale: 1.0,
            offset: Xy { x: 0.0, y: 0.0 },
            image,
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
            }
        }
    }

    pub fn render(&self, props: CanvasProps) -> RenderingTree {
        let image_size = self.image.size();
        let offset = self.offset.clone();
        let scale = self.scale.clone();

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
                builder.on_wheel(move |event| {
                    let canvas_wh = props.wh.clone();

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
                            &offset,
                            &local_mouse_position,
                            &canvas_wh,
                            &image_size,
                            scale,
                        )
                    } else if keyboard_manager.any_code_press([namui::Code::ShiftLeft]) {
                        scroll(
                            &Xy {
                                x: event.delta_xy.y,
                                y: event.delta_xy.x,
                            },
                            &offset,
                            &canvas_wh,
                            &image_size,
                            scale,
                        )
                    } else {
                        scroll(event.delta_xy, &offset, &canvas_wh, &image_size, scale)
                    }
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
                image(ImageParam {
                    xywh: namui::XywhRect {
                        x: scaled_offset.x,
                        y: scaled_offset.y,
                        width: scaled_image_size.width,
                        height: scaled_image_size.height,
                    },
                    source: namui::ImageSource::Image(self.image.clone()),
                    style: ImageStyle {
                        fit: ImageFit::Fill,
                        paint_builder: None,
                    },
                }),
            ),
        ])
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

fn scroll(
    delta_xy: &Xy<f32>,
    offset: &Xy<f32>,
    canvas_wh: &Wh<f32>,
    image_size: &Wh<f32>,
    scale: f32,
) {
    let scaled_delta_xy = Xy {
        x: delta_xy.x / scale,
        y: delta_xy.y / scale,
    };
    let new_offset = clamp_offset_xy(offset - scaled_delta_xy, canvas_wh, image_size, scale);
    namui::event::send(CanvasEvent::Scrolled { offset: new_offset })
}

fn zoom(
    delta_xy: &Xy<f32>,
    offset: &Xy<f32>,
    local_mouse_position: &Xy<f32>,
    canvas_wh: &Wh<f32>,
    image_size: &Wh<f32>,
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
    canvas_wh: &Wh<f32>,
    image_wh: &Wh<f32>,
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

fn clamp_scale(scale: f32, canvas_wh: &Wh<f32>, image_wh: &Wh<f32>) -> f32 {
    let ratio = Xy {
        x: image_wh.width / canvas_wh.width,
        y: image_wh.height / canvas_wh.height,
    };
    let max_ratio = ratio.x.max(ratio.y);
    let minimum_scale = 0.5 / max_ratio;
    scale.max(minimum_scale)
}

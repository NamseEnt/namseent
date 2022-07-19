use super::*;
use crate::dial_counter::Abs;
use namui::animation::ImageInterpolation;
use namui_prebuilt::table::*;
use std::{mem::discriminant, str::FromStr};

impl LineEditWindow {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let border = simple_rect(props.wh, Color::BLACK, 1.px(), Color::WHITE);

        let content = match (props.selected_layer, &props.editing_target) {
            (Some(layer), Some(EditingTarget::Line { point_id, layer_id }))
                if layer.id.eq(layer_id) =>
            {
                let (_, line) = layer
                    .image
                    .image_keyframe_graph
                    .get_point_and_line(point_id)
                    .unwrap();

                vertical([
                    fixed_no_clip(36.px(), |wh| {
                        self.render_line_select_dropdown(wh, layer, *line, point_id)
                    }),
                    ratio(1.0, |wh| {
                        self.render_line_editor(wh, layer, *line, point_id)
                    }),
                ])(props.wh)
            }
            _ => RenderingTree::Empty,
        };

        render([border, content])
    }
    fn render_line_select_dropdown(
        &self,
        wh: Wh<Px>,
        layer: &Layer,
        line: ImageInterpolation,
        point_id: &str,
    ) -> RenderingTree {
        let layer_id = layer.id.clone();
        let point_id = point_id.to_string();
        dropdown::render(dropdown::Props {
            rect: Rect::from_xy_wh(Xy::zero(), wh),
            items: ImageInterpolation::iter().map(|interpolation| dropdown::Item {
                id: interpolation.as_ref().to_string(),
                text: interpolation.as_ref().to_string(),
                is_selected: discriminant(&interpolation) == discriminant(&line),
            }),
            visible_item_count: 0,
            on_select_item: move |item_id| {
                let selected_line = match ImageInterpolation::from_str(&item_id).unwrap() {
                    ImageInterpolation::AllLinear => ImageInterpolation::AllLinear,
                    ImageInterpolation::SquashAndStretch { .. } => {
                        ImageInterpolation::SquashAndStretch {
                            frame_per_second: 60.0,
                        }
                    }
                };
                if discriminant(&selected_line) != discriminant(&line) {
                    namui::event::send(Event::SelectItem {
                        line: selected_line,
                        layer_id: layer_id.clone(),
                        point_id: point_id.clone(),
                    });
                }
            },
        })
    }

    fn render_line_editor(
        &self,
        wh: Wh<Px>,
        layer: &Layer,
        line: ImageInterpolation,
        point_id: &str,
    ) -> RenderingTree {
        match line {
            ImageInterpolation::AllLinear => RenderingTree::Empty,
            ImageInterpolation::SquashAndStretch { frame_per_second } => vertical([fixed(
                36.px(),
                horizontal([
                    ratio(1.0, |wh| {
                        typography::body::left(wh, "  Fps: ", Color::BLACK)
                    }),
                    ratio(4.0, |wh| {
                        let layer_id = layer.id.clone();
                        let point_id = point_id.to_string();
                        dropdown::render(dropdown::Props {
                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                            items: [60, 30, 24].iter().map(|fps| dropdown::Item {
                                id: fps.to_string(),
                                text: fps.to_string(),
                                is_selected: *fps == (frame_per_second as i32),
                            }),
                            visible_item_count: 0,
                            on_select_item: move |next_value| {
                                namui::event::send(Event::UpdateLine {
                                    layer_id: layer_id.clone(),
                                    point_id: point_id.clone(),
                                    func: Arc::new(move |line| {
                                        if let ImageInterpolation::SquashAndStretch {
                                            ref mut frame_per_second,
                                            ..
                                        } = line
                                        {
                                            *frame_per_second = f32::from_str(&next_value).unwrap();
                                        }
                                    }),
                                });
                            },
                        })
                    }),
                ]),
            )])(wh),
        }
    }
}

impl Abs for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }
}

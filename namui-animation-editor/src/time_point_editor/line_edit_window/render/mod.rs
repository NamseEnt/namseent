use super::*;
use crate::dial_counter::Abs;
use namui::animation::ImageInterpolation;
use namui_prebuilt::table::*;
use std::mem::discriminant;

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
                    .get_point_and_line(*point_id)
                    .unwrap();

                vertical([
                    fixed_no_clip(36.px(), |wh| {
                        self.render_line_select_dropdown(wh, layer, *line, *point_id)
                    }),
                    ratio(1.0, |wh| {
                        self.render_line_editor(wh, layer, *line, *point_id)
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
        point_id: Uuid,
    ) -> RenderingTree {
        let layer_id = layer.id.clone();
        dropdown::render(dropdown::Props {
            rect: Rect::from_xy_wh(Xy::zero(), wh),
            items: ImageInterpolation::iter().map(|interpolation| dropdown::Item {
                text: interpolation.as_ref().to_string(),
                is_selected: discriminant(&interpolation) == discriminant(&line),
                on_select_item: move |_| {
                    let selected_line = match interpolation {
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
                            layer_id,
                            point_id,
                        });
                    }
                },
            }),
            visible_item_count: 0,
        })
    }

    fn render_line_editor(
        &self,
        wh: Wh<Px>,
        layer: &Layer,
        line: ImageInterpolation,
        point_id: Uuid,
    ) -> RenderingTree {
        match line {
            ImageInterpolation::AllLinear => RenderingTree::Empty,
            ImageInterpolation::SquashAndStretch { frame_per_second } => vertical([fixed(
                36.px(),
                horizontal([
                    ratio(1.0, |wh| {
                        typography::body::left(wh.height, "  Fps: ", Color::BLACK)
                    }),
                    ratio(4.0, |wh| {
                        let layer_id = layer.id.clone();
                        dropdown::render(dropdown::Props {
                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                            items: [60, 30, 24].into_iter().map(|fps| dropdown::Item {
                                text: fps.to_string(),
                                is_selected: fps == (frame_per_second as i32),
                                on_select_item: move |_: ()| {
                                    namui::event::send(Event::UpdateLine {
                                        layer_id,
                                        point_id,
                                        func: closure(move |mut line| {
                                            if let ImageInterpolation::SquashAndStretch {
                                                frame_per_second,
                                                ..
                                            } = &mut line
                                            {
                                                *frame_per_second = fps as f32;
                                            }
                                            line
                                        }),
                                    });
                                },
                            }),
                            visible_item_count: 0,
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

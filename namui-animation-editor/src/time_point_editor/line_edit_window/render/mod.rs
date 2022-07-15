use super::*;
use crate::dial_counter::Abs;
use namui::animation::ImageInterpolation;
use namui_prebuilt::table::*;
use std::str::FromStr;

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
                        self.render_line_select_dropdown(&props, wh, layer, *line, point_id)
                    }),
                    ratio(1.0, |wh| {
                        self.render_line_editor(&props, wh, layer, *line, point_id)
                    }),
                ])(props.wh)
            }
            _ => RenderingTree::Empty,
        };

        render([border, content])
    }
    fn render_line_select_dropdown(
        &self,
        props: &Props,
        wh: Wh<Px>,
        layer: &Layer,
        line: ImageInterpolation,
        point_id: &str,
    ) -> RenderingTree {
        let layer_id = layer.id.clone();
        let point_id = point_id.to_string();
        self.dropdown.render(dropdown::Props {
            rect: Rect::from_xy_wh(Xy::zero(), wh),
            items: ImageInterpolation::iter().map(|interpolation| dropdown::Item {
                id: interpolation.as_ref().to_string(),
                text: interpolation.as_ref().to_string(),
                is_selected: interpolation.as_ref().eq(line.as_ref()),
            }),
            visible_item_count: 0,
            on_select_item: move |item_id| {
                namui::event::send(Event::SelectItem {
                    line: ImageInterpolation::from_str(&item_id).unwrap(),
                    layer_id: layer_id.clone(),
                    point_id: point_id.clone(),
                });
            },
        })
    }

    fn render_line_editor(
        &self,
        props: &Props,
        wh: Wh<Px>,
        layer: &Layer,
        line: ImageInterpolation,
        point_id: &str,
    ) -> RenderingTree {
        match line {
            ImageInterpolation::AllLinear => RenderingTree::Empty,
            ImageInterpolation::SquashAndStretch { velocity_ratio } => {
                vertical([fixed(36.px(), |wh| {
                    horizontal([
                        ratio(1.0, |wh| {
                            typography::body::left(wh, "  Velocity: ", Color::BLACK)
                        }),
                        ratio(4.0, |wh| {
                            let layer_id = layer.id.clone();
                            let point_id = point_id.to_string();
                            crate::dial_counter::DialCounter::new().render(
                                crate::dial_counter::Props {
                                    rect: Rect::from_xy_wh(Xy::zero(), wh),
                                    value: velocity_ratio,
                                    value_per_px: Per::new(0.01, 1.px()),
                                    small_gradation_value_interval: 0.2,
                                    big_gradation_per_small_gradation: 5,
                                    on_value_changed: move |next_value| {
                                        namui::event::send(
                                            Event::SquashAndStretchVelocityRatioUpdated {
                                                layer_id: layer_id.clone(),
                                                point_id: point_id.clone(),
                                                velocity_ratio: next_value,
                                            },
                                        );
                                    },
                                },
                            )
                        }),
                    ])(wh)
                })])(wh)
            }
        }
    }
}

impl Abs for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }
}

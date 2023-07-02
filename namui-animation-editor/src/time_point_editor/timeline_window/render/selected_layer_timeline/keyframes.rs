use super::*;

impl TimelineWindow {
    pub(super) fn render_keyframes(
        &self,
        props: &Props,
        wh: Wh<Px>,
        layer: &Layer,
    ) -> RenderingTree {
        let path_builder = PathBuilder::new()
            .move_to(0.px(), 0.px())
            .line_to(20.px(), 0.px())
            .line_to(20.px(), 20.px())
            .line_to(1.px(), 40.px())
            .line_to(1.px(), wh.height)
            .line_to(0.px(), wh.height)
            .close();
        let fill_paint_builder = PaintBuilder::new()
            .set_style(PaintStyle::Fill)
            .set_color(Color::BLACK)
            .set_anti_alias(true);
        let selected_fill_paint_builder = fill_paint_builder.clone().set_color(Color::RED);

        let stroke_paint_builder = PaintBuilder::new()
            .set_style(PaintStyle::Stroke)
            .set_color(Color::WHITE)
            .set_stroke_width(1.px())
            .set_anti_alias(true);

        let sign = render([
            namui::path(path_builder.clone(), stroke_paint_builder.clone()),
            namui::path(path_builder.clone(), fill_paint_builder.clone()),
        ]);
        let selected_sign = render([
            namui::path(path_builder.clone(), stroke_paint_builder.clone()),
            namui::path(path_builder.clone(), selected_fill_paint_builder.clone()),
        ]);

        let points = layer
            .image
            .image_keyframe_graph
            .get_point_line_tuples()
            .map(|(point, _)| point);

        let signs = points
            .filter(|point| {
                self.start_at <= point.time
                    && point.time <= self.start_at + (self.time_per_px * Px::from(wh.width))
            })
            .map(|point| {
                let x = (point.time - self.start_at) / self.time_per_px;
                let is_selected = {
                    if let Some(editing_target) = &props.editing_target {
                        if let EditingTarget::Keyframe { point_id, layer_id } = editing_target {
                            layer.id.eq(layer_id) && point.id() == *point_id
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                let sign = match is_selected {
                    true => selected_sign.clone(),
                    false => sign.clone(),
                };
                translate(
                    x,
                    px(0.0),
                    sign.attach_event(move |builder| {
                        let window_id = self.window_id.clone();
                        let point_id = point.id();
                        let keyframe_time = point.time;
                        let layer_id = layer.id.clone();

                        builder.on_mouse_down_in(move |event: MouseEvent| {
                            let root = namui::last_rendering_tree();
                            let window_global_xy = root.get_xy_by_id(window_id).unwrap();

                            namui::event::send(Event::KeyframeMouseDown {
                                layer_id,
                                point_id: point_id.clone(),
                                anchor_xy: event.local_xy,
                                keyframe_time,
                                mouse_local_xy: event.global_xy - window_global_xy,
                            });
                        });
                    }),
                )
            });

        render(signs)
    }
}

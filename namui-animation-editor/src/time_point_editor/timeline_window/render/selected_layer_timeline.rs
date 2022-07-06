use super::*;

struct MergedKeyframe {
    point_ids: Vec<String>,
    time: Time,
}

impl TimelineWindow {
    pub(super) fn render_selected_layer_timeline(
        &self,
        wh: Wh<Px>,
        props: &Props,
    ) -> RenderingTree {
        let selected_layer = props
            .selected_layer_id
            .as_ref()
            .and_then(|layer_id| props.layers.iter().find(|layer| layer.id.eq(layer_id)));

        if selected_layer.is_none() {
            return RenderingTree::Empty;
        }
        let selected_layer = selected_layer.unwrap();

        let mut keyframes: Vec<MergedKeyframe> = vec![];

        get_all_time_and_ids(&selected_layer)
            .into_iter()
            .for_each(|(time, id)| {
                let same_time_keyframe = keyframes.iter_mut().find(|k| k.time == time);

                match same_time_keyframe {
                    Some(keyframe) => {
                        keyframe.point_ids.push(id);
                    }
                    None => {
                        keyframes.push(MergedKeyframe {
                            point_ids: vec![id],
                            time,
                        });
                    }
                }
            });

        let path_builder = PathBuilder::new()
            .move_to(px(-20.0), px(0.0))
            .line_to(px(-1.0), px(20.0))
            .line_to(px(-1.0), wh.height)
            .line_to(px(0.0), wh.height)
            .line_to(px(0.0), px(0.0))
            .close();
        let paint_builder = PaintBuilder::new()
            .set_style(PaintStyle::Fill)
            .set_color(Color::BLACK)
            .set_anti_alias(true);
        let selected_paint_builder = paint_builder.clone().set_color(Color::RED);

        let sign = namui::path(path_builder.clone(), paint_builder.clone());
        let selected_sign = namui::path(path_builder.clone(), selected_paint_builder.clone());

        let signs = keyframes
            .iter()
            .filter(|keyframe| {
                self.start_at <= keyframe.time
                    && keyframe.time <= self.start_at + (self.time_per_px * Px::from(wh.width))
            })
            .map(|keyframe| {
                let x = (keyframe.time - self.start_at) / self.time_per_px;
                let is_selected = keyframe.time == self.get_playback_time();

                let sign = match is_selected {
                    true => selected_sign.clone(),
                    false => sign.clone(),
                };
                translate(
                    x,
                    px(0.0),
                    sign.attach_event(|builder| {
                        let point_ids = keyframe.point_ids.clone();
                        let keyframe_time = keyframe.time;
                        let window_id = self.window_id.clone();
                        let layer_id = selected_layer.id.clone();
                        builder.on_mouse_down_in(move |event| {
                            let window_global_xy = event
                                .namui_context
                                .get_rendering_tree_xy_by_id(&window_id)
                                .unwrap();

                            namui::event::send(Event::KeyframeMouseDown {
                                layer_id: layer_id.clone(),
                                point_ids: point_ids.clone(),
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

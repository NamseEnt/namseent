use super::*;
use namui::animation::{KeyframeGraph, KeyframeValue};

struct MergedKeyframe {
    point_ids: Vec<String>,
    time: Time,
}

impl TimelineWindow {
    pub(super) fn render_selected_layer_timeline(
        &self,
        wh: Wh<f32>,
        props: &Props,
    ) -> RenderingTree {
        let selected_layer = self
            .selected_layer_id
            .as_ref()
            .and_then(|layer_id| props.layers.iter().find(|layer| layer.id.eq(layer_id)));

        if selected_layer.is_none() {
            return RenderingTree::Empty;
        }
        let selected_layer = selected_layer.unwrap();

        let mut keyframes: Vec<MergedKeyframe> = vec![];

        let image = &selected_layer.image;

        [
            get_time_and_id(&image.x),
            get_time_and_id(&image.y),
            get_time_and_id(&image.width),
            get_time_and_id(&image.height),
            get_time_and_id(&image.rotation_angle),
            get_time_and_id(&image.opacity),
        ]
        .concat()
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
            .move_to(-20.0, 0.0)
            .line_to(-1.0, 20.0)
            .line_to(-1.0, wh.height)
            .line_to(0.0, wh.height)
            .line_to(0.0, 0.0)
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
                    && keyframe.time
                        <= self.start_at + (self.time_per_pixel * PixelSize::from(wh.width))
            })
            .map(|keyframe| {
                let x = (keyframe.time - self.start_at) / self.time_per_pixel;
                let is_selected = match &self.selected_point_ids {
                    Some(selected_point_ids) => selected_point_ids
                        .iter()
                        .any(|id| keyframe.point_ids.contains(id)),
                    None => false,
                };

                let sign = match is_selected {
                    true => selected_sign.clone(),
                    false => sign.clone(),
                };
                translate(
                    x.into(),
                    0.0,
                    sign.attach_event(|builder| {
                        let point_ids = keyframe.point_ids.clone();
                        builder.on_mouse_down(move |event| {
                            namui::event::send(Event::KeyframeMouseDown {
                                point_ids: point_ids.clone(),
                                anchor_xy: event.local_xy,
                            });
                        })
                    }),
                )
            });

        render(signs)
    }
}

fn get_time_and_id<T: KeyframeValue + Clone>(graph: &KeyframeGraph<T>) -> Vec<(Time, String)> {
    graph
        .get_points_with_lines()
        .iter()
        .map(|(point, _)| (point.time, point.id().to_string()))
        .collect()
}

use super::*;
use namui::animation::{KeyframeGraph, KeyframeValue};

impl TimelineWindow {
    pub(super) fn render_selected_layer_timeline(
        &self,
        wh: Wh<f32>,
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

        let mut times_of_points = vec![];

        let image = &selected_layer.image;

        [
            get_times(&image.x),
            get_times(&image.y),
            get_times(&image.width),
            get_times(&image.height),
            get_times(&image.rotation_angle),
            get_times(&image.opacity),
        ]
        .concat()
        .iter()
        .for_each(|time| {
            if !times_of_points.contains(time) {
                times_of_points.push(*time);
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

        let lines = times_of_points
            .iter()
            .filter(|time| {
                self.start_at <= *time
                    && *time <= self.start_at + (self.time_per_pixel * PixelSize::from(wh.width))
            })
            .map(|time| {
                let x = (time - self.start_at) / self.time_per_pixel;
                translate(
                    x.into(),
                    0.0,
                    namui::path(path_builder.clone(), paint_builder.clone()),
                )
            });

        render(lines)
    }
}

fn get_times<T: KeyframeValue + Clone>(graph: &KeyframeGraph<T>) -> Vec<Time> {
    graph
        .get_points_with_lines()
        .iter()
        .map(|(point, _)| point.time)
        .collect()
}

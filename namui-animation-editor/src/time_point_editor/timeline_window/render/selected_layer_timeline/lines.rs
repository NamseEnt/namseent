use super::*;

impl TimelineWindow {
    pub(super) fn render_lines(&self, wh: Wh<Px>, layer: &Layer) -> RenderingTree {
        let mut point_line_tuples = layer
            .image
            .image_keyframe_graph
            .get_point_line_tuples()
            .peekable();

        let mut lines = vec![];

        while let Some((left_point, _)) = point_line_tuples.next() {
            if self.start_at + self.time_per_px * wh.width < left_point.time {
                break;
            }

            if point_line_tuples.peek().is_none() {
                break;
            }

            let (right_point, _) = point_line_tuples.peek().unwrap();

            if right_point.time < self.start_at {
                break;
            }

            let left_x = (left_point.time - self.start_at) / self.time_per_px;
            let right_x = (right_point.time - self.start_at) / self.time_per_px;

            lines.push(render_line(left_x, right_x, wh));
        }

        render(lines)
    }
}

fn render_line(left_x: Px, right_x: Px, wh: Wh<Px>) -> RenderingTree {
    let line_height = wh.height / 3.0;
    translate(
        left_x,
        (wh.height - line_height) / 2.0,
        simple_rect(
            Wh {
                width: right_x - left_x,
                height: line_height,
            },
            Color::BLACK,
            2.px(),
            Color::GREEN,
        ),
    )
}

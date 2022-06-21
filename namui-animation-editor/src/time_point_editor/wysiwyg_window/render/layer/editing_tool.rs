use super::*;

impl WysiwygWindow {
    pub(super) fn render_editing_tool(
        &self,
        layer: &Layer,
        playback_time: Time,
        rendered_image: &RenderingTree,
    ) -> namui::RenderingTree {
        if self.selected_layer_id != Some(layer.id.clone()) {
            return namui::RenderingTree::Empty;
        }

        let bounding_box = rendered_image.get_bounding_box().unwrap();

        let wh = bounding_box.wh();

        translate(
            bounding_box.x,
            bounding_box.y,
            render([
                self.render_border(wh),
                self.render_circles(wh, playback_time),
            ]),
        )
    }

    fn render_border(&self, wh: Wh<f32>) -> RenderingTree {
        simple_rect(wh, Color::grayscale_f01(0.2), 2.0, Color::TRANSPARENT)
    }
    fn render_circles(&self, wh: Wh<f32>, playback_time: Time) -> RenderingTree {
        const CIRCLE_RADIUS: f32 = 10.0;
        let circle_path = PathBuilder::new().add_oval(&LtrbRect {
            left: -CIRCLE_RADIUS,
            top: -CIRCLE_RADIUS,
            right: CIRCLE_RADIUS,
            bottom: CIRCLE_RADIUS,
        });
        let circle_fill_paint = PaintBuilder::new()
            .set_style(PaintStyle::Fill)
            .set_color(Color::WHITE);
        let circle_stroke_paint = PaintBuilder::new()
            .set_style(PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(3.0)
            .set_anti_alias(true);

        let circle_rendering_tree = render([
            path(circle_path.clone(), circle_fill_paint),
            path(circle_path, circle_stroke_paint),
        ]);

        render(
            [
                (
                    ResizeCircleLocation::LeftTop,
                    0.0,
                    0.0,
                    MouseCursor::LeftTopRightBottomResize,
                ),
                (
                    ResizeCircleLocation::Top,
                    wh.width / 2.0,
                    0.0,
                    MouseCursor::TopBottomResize,
                ),
                (
                    ResizeCircleLocation::RightTop,
                    wh.width,
                    0.0,
                    MouseCursor::RightTopLeftBottomResize,
                ),
                (
                    ResizeCircleLocation::Left,
                    0.0,
                    wh.height / 2.0,
                    MouseCursor::LeftRightResize,
                ),
                (
                    ResizeCircleLocation::Right,
                    wh.width,
                    wh.height / 2.0,
                    MouseCursor::LeftRightResize,
                ),
                (
                    ResizeCircleLocation::LeftBottom,
                    0.0,
                    wh.height,
                    MouseCursor::RightTopLeftBottomResize,
                ),
                (
                    ResizeCircleLocation::Bottom,
                    wh.width / 2.0,
                    wh.height,
                    MouseCursor::TopBottomResize,
                ),
                (
                    ResizeCircleLocation::RightBottom,
                    wh.width,
                    wh.height,
                    MouseCursor::LeftTopRightBottomResize,
                ),
            ]
            .into_iter()
            .map(|(location, x, y, cursor)| {
                translate(
                    x,
                    y,
                    circle_rendering_tree
                        .clone()
                        .with_mouse_cursor(cursor)
                        .attach_event(|builder| {
                            let mouse_local_xy = self.mouse_local_xy.unwrap();
                            builder.on_mouse_down(move |_| {
                                namui::event::send(Event::ResizeCircleClicked {
                                    location,
                                    anchor_xy: mouse_local_xy,
                                    playback_time,
                                });
                            })
                        }),
                )
            }),
        )
    }
}

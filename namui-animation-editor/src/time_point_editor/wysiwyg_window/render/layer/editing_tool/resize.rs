use super::*;

impl WysiwygWindow {
    pub(super) fn render_resize_circles(
        &self,
        wh: Wh<Px>,
        playback_time: Time,
        layer_id: &str,
        rotation_angle: Angle,
    ) -> RenderingTree {
        let circle_radius = px(6.0) * self.real_px_per_screen_px;
        let circle_path = PathBuilder::new().add_oval(Rect::Ltrb {
            left: -circle_radius,
            top: -circle_radius,
            right: circle_radius,
            bottom: circle_radius,
        });
        let circle_fill_paint = PaintBuilder::new()
            .set_style(PaintStyle::Fill)
            .set_color(Color::WHITE);
        let circle_stroke_paint = PaintBuilder::new()
            .set_style(PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(px(self.real_px_per_screen_px))
            .set_anti_alias(true);

        let circle_rendering_tree = render([
            path(circle_path.clone(), circle_fill_paint),
            path(circle_path, circle_stroke_paint),
        ]);

        render(
            [
                (
                    ResizeCircleLocation::LeftTop,
                    Px::from(0.0),
                    Px::from(0.0),
                    MouseCursor::LeftTopRightBottomResize,
                ),
                (
                    ResizeCircleLocation::Top,
                    wh.width / 2.0,
                    Px::from(0.0),
                    MouseCursor::TopBottomResize,
                ),
                (
                    ResizeCircleLocation::RightTop,
                    wh.width,
                    Px::from(0.0),
                    MouseCursor::RightTopLeftBottomResize,
                ),
                (
                    ResizeCircleLocation::Left,
                    Px::from(0.0),
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
                    Px::from(0.0),
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
                            let window_id = self.window_id.clone();
                            let layer_id = layer_id.to_string();
                            builder.on_mouse_down_in(move |event| {
                                let window_global_xy = event
                                    .namui_context
                                    .get_rendering_tree_xy_by_id(&window_id)
                                    .unwrap();
                                let anchor_xy = event.global_xy - window_global_xy;

                                namui::event::send(Event::ResizeCircleMouseDown {
                                    location,
                                    anchor_xy,
                                    playback_time,
                                    layer_id: layer_id.clone(),
                                    rotation_angle,
                                });
                            });
                        }),
                )
            }),
        )
    }
}

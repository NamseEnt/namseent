use std::f32::consts::PI;

use super::*;

impl WysiwygWindow {
    pub(super) fn render_rotation_tool(
        &self,
        wh: Wh<Px>,
        playback_time: Time,
        selected_layer_id: String,
        image_anchor_local_xy: Xy<f32>,
    ) -> RenderingTree {
        let arrow_height = 8.0 * self.real_px_per_screen_px;
        let arrow_width = 15.0 * self.real_px_per_screen_px;
        let inner_radius = 5.0 * self.real_px_per_screen_px;
        let outer_radius = 10.0 * self.real_px_per_screen_px;
        let tail_radian = PI * 1.6;

        let rotation_tool_path = PathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(arrow_width / 2.0, arrow_height)
            .line_to(arrow_width, 0.0)
            .line_to(outer_radius, 0.0)
            .arc_to(
                &LtrbRect {
                    left: -outer_radius,
                    top: -outer_radius,
                    right: outer_radius,
                    bottom: outer_radius,
                },
                0.0,
                -tail_radian,
            )
            .line_to(
                inner_radius * (-tail_radian).cos(),
                inner_radius * (-tail_radian).sin(),
            )
            .arc_to(
                &LtrbRect {
                    left: -inner_radius,
                    top: -inner_radius,
                    right: inner_radius,
                    bottom: inner_radius,
                },
                -tail_radian,
                tail_radian,
            )
            .line_to(0.0, 0.0)
            .close();

        let fill_paint = PaintBuilder::new()
            .set_style(PaintStyle::Fill)
            .set_color(Color::WHITE);
        let stroke_paint = PaintBuilder::new()
            .set_style(PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(2.0 * self.real_px_per_screen_px)
            .set_anti_alias(true);

        let guide_line_length = 50.0 * self.real_px_per_screen_px;

        let guide_line_path = PathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(0.0, -guide_line_length);

        let tool_rendering_tree = translate(
            (wh.width / 2.0_f32).into(),
            -guide_line_length - outer_radius,
            render([
                path(rotation_tool_path.clone(), fill_paint),
                path(rotation_tool_path, stroke_paint.clone()),
            ]),
        );

        let cursor_and_event_handler = path(
            PathBuilder::new()
                .add_rect(&tool_rendering_tree.get_bounding_box().unwrap().into_ltrb()),
            PaintBuilder::new().set_color(Color::TRANSPARENT),
        )
        .with_mouse_cursor(MouseCursor::Custom(get_mouse_cursor()))
        .attach_event(|builder| {
            let window_id = self.window_id.clone();
            let layer_id = selected_layer_id.clone();
            let real_px_per_screen_px = self.real_px_per_screen_px;
            builder.on_mouse_down(move |event| {
                let window_global_xy = event
                    .namui_context
                    .get_rendering_tree_xy_by_id(&window_id)
                    .unwrap();
                let mouse_local_xy = real_px_per_screen_px * (event.global_xy - window_global_xy);

                namui::event::send(Event::RotationToolMouseDown {
                    layer_id: layer_id.clone(),
                    playback_time,
                    mouse_local_xy,
                    image_center_real_xy: image_anchor_local_xy,
                });
            });
        });

        let guide_line_rendering_tree = translate(
            (wh.width / 2.0_f32).into(),
            0.0,
            render([path(guide_line_path, stroke_paint)]),
        );

        render([
            guide_line_rendering_tree,
            tool_rendering_tree,
            cursor_and_event_handler,
        ])
    }
}

fn get_mouse_cursor() -> RenderingTree {
    const TRIANGLE_HEIGHT: f32 = 8.0;
    const ARC_RADIUS: f32 = 16.0;
    const ARC_RADIAN: f32 = PI / 3.0;

    let most_left = -(ARC_RADIUS * (ARC_RADIAN / 2.0).sin() + TRIANGLE_HEIGHT / 2.0);
    let most_right = -most_left;

    let mouse_cursor_path = PathBuilder::new()
        .move_to(most_left, 0.0)
        .line_to(most_left, TRIANGLE_HEIGHT)
        .line_to(most_left + TRIANGLE_HEIGHT, TRIANGLE_HEIGHT)
        .line_to(most_left, 0.0)
        .move_to(most_left + TRIANGLE_HEIGHT / 2.0, TRIANGLE_HEIGHT / 2.0)
        .arc_to(
            &LtrbRect {
                left: -ARC_RADIUS,
                top: 0.0,
                right: ARC_RADIUS,
                bottom: ARC_RADIUS * 2.0,
            },
            PI + (PI - ARC_RADIAN) / 2.0,
            ARC_RADIAN,
        )
        .move_to(most_right, 0.0)
        .line_to(most_right - TRIANGLE_HEIGHT, TRIANGLE_HEIGHT)
        .line_to(most_right, TRIANGLE_HEIGHT)
        .line_to(most_right, 0.0);

    let inner_stroke_paint = PaintBuilder::new()
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.0)
        .set_color(Color::BLACK)
        .set_anti_alias(true);
    let outer_stroke_paint = PaintBuilder::new()
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.0)
        .set_color(Color::WHITE)
        .set_anti_alias(true);

    render([
        path(mouse_cursor_path.clone(), outer_stroke_paint),
        path(mouse_cursor_path, inner_stroke_paint),
    ])
}

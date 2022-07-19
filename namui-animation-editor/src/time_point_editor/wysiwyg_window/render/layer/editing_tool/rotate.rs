use super::*;
use crate::time_point_editor::wysiwyg_window::update::dragging;
use std::f32::consts::PI;

impl WysiwygWindow {
    pub(super) fn render_rotation_tool(
        &self,
        wh: Wh<Px>,
        keyframe_point_id: &str,
        selected_layer_id: String,
        image_anchor_local_xy: Xy<Px>,
    ) -> RenderingTree {
        let arrow_height = px(8.0) * self.real_px_per_screen_px;
        let arrow_width = px(15.0) * self.real_px_per_screen_px;
        let inner_radius = px(5.0) * self.real_px_per_screen_px;
        let outer_radius = px(10.0) * self.real_px_per_screen_px;
        let tail_angle = Angle::Radian(PI * 1.6);

        let rotation_tool_path = PathBuilder::new()
            .move_to(px(0.0), px(0.0))
            .line_to(arrow_width / 2.0, arrow_height)
            .line_to(arrow_width, px(0.0))
            .line_to(outer_radius, px(0.0))
            .arc_to(
                Rect::Ltrb {
                    left: -outer_radius,
                    top: -outer_radius,
                    right: outer_radius,
                    bottom: outer_radius,
                },
                Angle::Degree(0.0),
                -tail_angle,
            )
            .line_to(
                inner_radius * (-tail_angle).cos(),
                inner_radius * (-tail_angle).sin(),
            )
            .arc_to(
                Rect::Ltrb {
                    left: -inner_radius,
                    top: -inner_radius,
                    right: inner_radius,
                    bottom: inner_radius,
                },
                -tail_angle,
                tail_angle,
            )
            .line_to(px(0.0), px(0.0))
            .close();

        let fill_paint = PaintBuilder::new()
            .set_style(PaintStyle::Fill)
            .set_color(Color::WHITE);
        let stroke_paint = PaintBuilder::new()
            .set_style(PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.5))
            .set_stroke_width(px(2.0) * self.real_px_per_screen_px)
            .set_anti_alias(true);

        let guide_line_length = px(50.0) * self.real_px_per_screen_px;

        let guide_line_path = PathBuilder::new()
            .move_to(px(0.0), px(0.0))
            .line_to(px(0.0), -guide_line_length);

        let tool_rendering_tree = translate(
            (wh.width / 2.0_f32).into(),
            -guide_line_length - outer_radius,
            render([
                path(rotation_tool_path.clone(), fill_paint),
                path(rotation_tool_path, stroke_paint.clone()),
            ]),
        );

        let keyframe_point_id = keyframe_point_id.to_string();

        let cursor_and_event_handler = path(
            PathBuilder::new().add_rect(tool_rendering_tree.get_bounding_box().unwrap()),
            PaintBuilder::new().set_color(Color::TRANSPARENT),
        )
        .with_mouse_cursor(MouseCursor::Custom(get_mouse_cursor()))
        .attach_event(|builder| {
            let window_id = self.window_id.clone();
            let layer_id = selected_layer_id.clone();
            let real_px_per_screen_px = self.real_px_per_screen_px;
            let keyframe_point_id = keyframe_point_id.clone();
            builder.on_mouse_down_in(move |event| {
                let window_global_xy = event
                    .namui_context
                    .get_rendering_tree_xy_by_id(&window_id)
                    .unwrap();
                let mouse_local_xy = real_px_per_screen_px * (event.global_xy - window_global_xy);

                namui::event::send(Event::RotationToolMouseDown {
                    layer_id: layer_id.clone(),
                    keyframe_point_id: keyframe_point_id.to_string(),
                    mouse_local_xy,
                    image_center_real_xy: image_anchor_local_xy,
                });
            });
        });

        let guide_line_rendering_tree = translate(
            (wh.width / 2.0_f32).into(),
            px(0.0),
            render([path(guide_line_path, stroke_paint)]),
        );

        let is_dragging_rotation_tool = self
            .animation_history
            .check_action(|_: &dragging::DragRotationAction| true);

        let cursor_on_drag = if is_dragging_rotation_tool {
            absolute(
                0.px(),
                0.px(),
                simple_rect(
                    namui::screen::size(),
                    Color::TRANSPARENT,
                    0.px(),
                    Color::TRANSPARENT,
                ),
            )
            .with_mouse_cursor(MouseCursor::Custom(get_mouse_cursor()))
        } else {
            RenderingTree::Empty
        };

        render([
            guide_line_rendering_tree,
            tool_rendering_tree,
            cursor_and_event_handler,
            cursor_on_drag,
        ])
    }
}

fn get_mouse_cursor() -> RenderingTree {
    const TRIANGLE_HEIGHT: Px = px(8.0);
    const ARC_RADIUS: Px = px(16.0);
    const ARC_ANGLE: Angle = Angle::Degree(60.0);

    let most_left = -(ARC_RADIUS * (ARC_ANGLE / 2.0).sin() + TRIANGLE_HEIGHT / 2.0);
    let most_right = -most_left;

    let mouse_cursor_path = PathBuilder::new()
        .move_to(most_left, px(0.0))
        .line_to(most_left, TRIANGLE_HEIGHT)
        .line_to(most_left + TRIANGLE_HEIGHT, TRIANGLE_HEIGHT)
        .line_to(most_left, px(0.0))
        .move_to(most_left + TRIANGLE_HEIGHT / 2.0, TRIANGLE_HEIGHT / 2.0)
        .arc_to(
            Rect::Ltrb {
                left: -ARC_RADIUS,
                top: px(0.0),
                right: ARC_RADIUS,
                bottom: ARC_RADIUS * 2.0,
            },
            Angle::Radian(1.5 * PI) - ARC_ANGLE / 2.0,
            ARC_ANGLE,
        )
        .move_to(most_right, px(0.0))
        .line_to(most_right - TRIANGLE_HEIGHT, TRIANGLE_HEIGHT)
        .line_to(most_right, TRIANGLE_HEIGHT)
        .line_to(most_right, px(0.0));

    let inner_stroke_paint = PaintBuilder::new()
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(px(2.0))
        .set_color(Color::BLACK)
        .set_anti_alias(true);
    let outer_stroke_paint = PaintBuilder::new()
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(px(3.0))
        .set_color(Color::WHITE)
        .set_anti_alias(true);

    render([
        path(mouse_cursor_path.clone(), outer_stroke_paint),
        path(mouse_cursor_path, inner_stroke_paint),
    ])
}

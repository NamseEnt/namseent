use crate::app::{
    editor::{events::EditorEvent, TimelineRenderContext},
    types::SubtitleClip,
};
use namui::prelude::*;

pub struct SubtitleClipBody {}
pub struct SubtitleClipBodyProps<'a> {
    pub track_body_wh: Wh<Px>,
    pub clip: &'a SubtitleClip,
    pub context: &'a TimelineRenderContext<'a>,
}
impl SubtitleClipBody {
    pub fn render(props: &SubtitleClipBodyProps) -> RenderingTree {
        let SubtitleClipBodyProps { clip, context, .. } = props;
        let x = (clip.start_at - context.start_at) / context.time_per_px;
        let duration = context
            .subtitle_play_duration_measurer
            .get_play_duration(&clip.subtitle, &context.language);
        let width: Px = duration / context.time_per_px;

        let is_out_of_bounds = x + width < px(0.0) || x > props.track_body_wh.width;
        if is_out_of_bounds {
            return RenderingTree::Empty;
        }
        let subtitle_text = clip
            .subtitle
            .language_text_map
            .get(&context.language)
            .unwrap();

        let is_highlight = props.context.selected_clip_ids.contains(&&props.clip.id);

        let border_width = if is_highlight { px(2.0) } else { px(1.0) };

        let fill_color = if is_highlight {
            Color::from_u8(255, 165, 0, 255)
        } else if clip.is_needed_to_update_position {
            Color::from_u8(255, 0, 255, 255)
        } else {
            Color::from_f01(0.4, 0.4, 0.8, 1.0)
        };
        let border_color = if is_highlight {
            Color::RED
        } else {
            Color::BLACK
        };

        let clip_wh = Wh {
            width,
            height: props.track_body_wh.height,
        };

        let circle_radius = Time::Ms(100.0) / context.time_per_px;

        let head_left_top = namui::Xy {
            x: px(0.0),
            y: px(0.0),
        };

        let tail_right_bottom = namui::Xy {
            x: clip_wh.width,
            y: clip_wh.height,
        };
        let head_height = clip_wh.height / 2.5;

        let head_circle_center_xy = Xy {
            x: head_left_top.x + circle_radius,
            y: head_left_top.y + head_height,
        };

        let tail_height = clip_wh.height / 3.0;
        let tail_circle_center_xy = Xy {
            x: tail_right_bottom.x - circle_radius,
            y: tail_right_bottom.y - tail_height,
        };

        let render_setting = SubtitleClipBodyRenderSetting {
            circle_radius,
            head_left_top,
            tail_right_bottom,
            head_height,
            tail_height,
            head_circle_center_xy,
            tail_circle_center_xy,
        };

        let clip_body_path = get_clip_body_path(render_setting);

        let fill_paint = namui::PaintBuilder::new()
            .set_anti_alias(true)
            .set_style(namui::PaintStyle::Fill)
            .set_color(fill_color);

        let stroke_paint = namui::PaintBuilder::new()
            .set_anti_alias(true)
            .set_style(namui::PaintStyle::Stroke)
            .set_stroke_width(border_width)
            .set_color(border_color);

        let check_flag = match clip.is_needed_to_update_position {
            true => text(TextParam {
                x: head_left_top.x + clip_wh.width / 2.0,
                y: head_left_top.y + clip_wh.height / 2.0,
                text: "Check!".to_string(),
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font_type: FontType {
                    font_weight: FontWeight::BOLD,
                    language: context.language.clone(),
                    serif: false,
                    size: int_px(10),
                },
                style: TextStyle {
                    color: Color::from_u8(0xFF, 0x00, 0xFF, 0xFF),
                    border: Some(TextStyleBorder {
                        color: Color::WHITE,
                        width: px(1.0),
                    }),
                    ..Default::default()
                },
            }),
            false => RenderingTree::Empty,
        };

        let start_at = context.start_at;
        let time_per_px = context.time_per_px;

        translate(
            x,
            px(0.0),
            render([
                namui::path(clip_body_path.clone(), fill_paint),
                namui::path(clip_body_path.clone(), stroke_paint)
                    .with_mouse_cursor(MouseCursor::Grab)
                    .attach_event(move |builder| {
                        let clip_id = props.clip.id.clone();
                        builder.on_mouse_down_in(move |event| {
                            namui::event::send(EditorEvent::SubtitleClipHeadMouseDownEvent {
                                mouse_event_id: event.id.clone(),
                                clip_id: clip_id.clone(),
                                click_in_time: start_at + (event.local_xy.x + x) * time_per_px,
                            });
                        });
                    }),
                render_text_box(
                    &subtitle_text,
                    head_circle_center_xy,
                    tail_circle_center_xy,
                    circle_radius,
                ),
                check_flag,
            ]),
        )
    }
}

struct SubtitleClipBodyRenderSetting {
    circle_radius: Px,
    head_left_top: Xy<Px>,
    tail_right_bottom: Xy<Px>,
    head_height: Px,
    head_circle_center_xy: Xy<Px>,
    tail_height: Px,
    tail_circle_center_xy: Xy<Px>,
}

fn render_text_box(
    text: &str,
    head_circle_center_xy: Xy<Px>,
    tail_circle_center_xy: Xy<Px>,
    circle_radius: Px,
) -> RenderingTree {
    let horizontal_padding = px(1.0);
    let vertical_padding = px(1.0);
    let text_box_width =
        (tail_circle_center_xy - head_circle_center_xy).length() - horizontal_padding * 2.0;
    let text_box_height = circle_radius * 2.0 - vertical_padding * 2.0;

    let circle_to_circle_vector = tail_circle_center_xy - head_circle_center_xy;
    let angle = circle_to_circle_vector.atan2();
    let text_box_left_top = head_circle_center_xy
        + Xy {
            x: horizontal_padding,
            y: -text_box_height / 2.0,
        };
    translate(
        text_box_left_top.x,
        text_box_left_top.y,
        rotate(
            angle,
            render([
                rect(RectParam {
                    rect: Rect::Xywh {
                        x: px(0.0),
                        y: px(0.0),
                        width: text_box_width,
                        height: text_box_height,
                    },
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: namui::Color::WHITE,
                        }),
                        ..Default::default()
                    },
                }),
                namui::text(TextParam {
                    x: px(0.0),
                    y: text_box_height / 2.0,
                    text: text.to_string(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Middle,
                    font_type: FontType {
                        font_weight: FontWeight::REGULAR,
                        language: Language::Ko,
                        serif: false,
                        size: (text_box_height * 0.65).into(),
                    },
                    style: TextStyle {
                        color: namui::Color::BLACK,
                        ..Default::default()
                    },
                }),
            ]),
        ),
    )
}

fn get_clip_body_path(render_setting: SubtitleClipBodyRenderSetting) -> PathBuilder {
    let SubtitleClipBodyRenderSetting {
        circle_radius,
        head_left_top,
        tail_right_bottom,
        head_height,
        head_circle_center_xy,
        tail_height,
        tail_circle_center_xy,
        ..
    } = render_setting;

    let length_to_head_center = Xy {
        x: head_height,
        y: circle_radius,
    }
    .length();
    let sin_of_head_half_circumcircle = head_height / length_to_head_center;
    let cos_of_head_half_circumcircle = circle_radius / length_to_head_center;
    let head_circumcircle_angle = Angle::Radian(sin_of_head_half_circumcircle.asin() * 2.0);

    let head_circle_to_tail_circle_vector = tail_circle_center_xy - head_circle_center_xy;
    let head_circle_to_tail_circle_angle = head_circle_to_tail_circle_vector.atan2();

    let head_circle_oval_rect = Rect::Ltrb {
        left: head_circle_center_xy.x - circle_radius,
        top: head_circle_center_xy.y - circle_radius,
        right: head_circle_center_xy.x + circle_radius,
        bottom: head_circle_center_xy.y + circle_radius,
    };

    let tail_circle_oval_rect = Rect::Ltrb {
        left: tail_circle_center_xy.x - circle_radius,
        top: tail_circle_center_xy.y - circle_radius,
        right: tail_circle_center_xy.x + circle_radius,
        bottom: tail_circle_center_xy.y + circle_radius,
    };

    let head_circle_right_point_of_contact_from_head_left_top = Xy {
        x: 2.0 * circle_radius * sin_of_head_half_circumcircle.powf(2.0),
        y: head_height
            - 2.0 * circle_radius * (sin_of_head_half_circumcircle * cos_of_head_half_circumcircle),
    };

    let length_to_tail_center = Xy {
        x: tail_height,
        y: circle_radius,
    }
    .length();
    let sin_of_tail_half_circumcircle = head_height / length_to_tail_center;
    let cos_of_tail_half_circumcircle = circle_radius / length_to_tail_center;
    let tail_circumcircle_angle = Angle::Radian(sin_of_tail_half_circumcircle.asin() * 2.0);

    let tail_circle_left_point_of_contact_from_tail_right_bottom = Xy {
        x: tail_circle_center_xy.x
            - circle_radius * (2.0 * sin_of_tail_half_circumcircle.powf(2.0) - 1.0),
        y: tail_circle_center_xy.y
            + 2.0 * circle_radius * (sin_of_tail_half_circumcircle * cos_of_tail_half_circumcircle),
    };

    let mut clip_body_path = PathBuilder::new().move_to(head_left_top.x, head_left_top.y);

    let head_circle_right_point_of_contact_from_circle_to_circle_vector = head_circle_center_xy
        + Xy {
            x: circle_radius * (-(Angle::Degree(90.0) - head_circle_to_tail_circle_angle)).cos(),
            y: circle_radius * (-(Angle::Degree(90.0) - head_circle_to_tail_circle_angle)).sin(),
        };

    let head_top_arc_delta_angle =
        Angle::Degree(90.0) + head_circle_to_tail_circle_angle - head_circumcircle_angle;

    let is_need_to_draw_head_top_arc = head_top_arc_delta_angle > Angle::Degree(0.0);
    if is_need_to_draw_head_top_arc {
        let start_angle = Angle::Degree(180.0) + head_circumcircle_angle;
        clip_body_path = clip_body_path
            .line_to(
                head_circle_right_point_of_contact_from_head_left_top.x,
                head_circle_right_point_of_contact_from_head_left_top.y,
            )
            .arc_to(head_circle_oval_rect, start_angle, head_top_arc_delta_angle);
    } else {
        clip_body_path = clip_body_path.line_to(
            head_circle_right_point_of_contact_from_circle_to_circle_vector.x,
            head_circle_right_point_of_contact_from_circle_to_circle_vector.y,
        );
    }

    let head_top_arc_end_point = head_circle_right_point_of_contact_from_circle_to_circle_vector;

    let tail_bridge_top_xy = head_top_arc_end_point + head_circle_to_tail_circle_vector;

    clip_body_path = clip_body_path
        .line_to(tail_bridge_top_xy.x, tail_bridge_top_xy.y)
        .arc_to(
            tail_circle_oval_rect,
            -(Angle::Degree(90.0) - head_circle_to_tail_circle_angle),
            Angle::Degree(90.0) - head_circle_to_tail_circle_angle,
        )
        .line_to(tail_right_bottom.x, tail_right_bottom.y);

    let tail_circle_left_point_of_contact_from_circle_to_circle_vector = tail_circle_center_xy
        + Xy {
            x: -circle_radius * (Angle::Degree(90.0) - head_circle_to_tail_circle_angle).cos(),
            y: circle_radius * (Angle::Degree(90.0) - head_circle_to_tail_circle_angle).sin(),
        };

    let tail_bottom_arc_angle =
        Angle::Degree(90.0) + head_circle_to_tail_circle_angle - tail_circumcircle_angle;

    let is_need_to_draw_tail_bottom_arc = tail_bottom_arc_angle > Angle::Degree(0.0);
    if is_need_to_draw_tail_bottom_arc {
        let start_angle = tail_circumcircle_angle;
        clip_body_path = clip_body_path
            .line_to(
                tail_circle_left_point_of_contact_from_tail_right_bottom.x,
                tail_circle_left_point_of_contact_from_tail_right_bottom.y,
            )
            .arc_to(head_circle_oval_rect, start_angle, tail_bottom_arc_angle);
    } else {
        clip_body_path = clip_body_path.line_to(
            tail_circle_left_point_of_contact_from_circle_to_circle_vector.x,
            tail_circle_left_point_of_contact_from_circle_to_circle_vector.y,
        );
    }

    let head_circle_left_point_of_contact_from_circle_to_circle_vector =
        tail_circle_left_point_of_contact_from_circle_to_circle_vector
            - head_circle_to_tail_circle_vector;
    clip_body_path = clip_body_path
        .line_to(
            head_circle_left_point_of_contact_from_circle_to_circle_vector.x,
            head_circle_left_point_of_contact_from_circle_to_circle_vector.y,
        )
        .arc_to(
            head_circle_oval_rect,
            Angle::Degree(90.0) + head_circle_to_tail_circle_angle,
            Angle::Degree(90.0) - head_circle_to_tail_circle_angle,
        )
        .line_to(head_left_top.x, head_left_top.y)
        .close();
    clip_body_path
}

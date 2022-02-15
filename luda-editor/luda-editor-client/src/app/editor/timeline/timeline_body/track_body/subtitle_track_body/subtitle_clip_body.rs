use std::f32::consts::PI;

use crate::app::{
    editor::{events::EditorEvent, TimelineRenderContext},
    types::{PixelSize, SubtitleClip, Time},
};
use namui::prelude::*;

pub struct SubtitleClipBody {}
pub struct SubtitleClipBodyProps<'a> {
    pub track_body_wh: &'a Wh<f32>,
    pub clip: &'a SubtitleClip,
    pub context: &'a TimelineRenderContext<'a>,
}
impl SubtitleClipBody {
    pub fn render(props: &SubtitleClipBodyProps) -> RenderingTree {
        let SubtitleClipBodyProps { clip, context, .. } = props;
        let timeline_start_at = context.start_at;
        let time_per_pixel = context.time_per_pixel;
        let x = ((clip.start_at - context.start_at) / context.time_per_pixel).into();
        let duration = context
            .subtitle_play_duration_measurer
            .get_play_duration(&clip.subtitle, &context.language);
        let width: f32 = (duration / context.time_per_pixel).into();

        let is_out_of_bounds = x + width < 0.0 || x > props.track_body_wh.width;
        if is_out_of_bounds {
            return RenderingTree::Empty;
        }
        let subtitle_text = clip
            .subtitle
            .language_text_map
            .get(&context.language)
            .unwrap();

        let is_highlight = props.context.selected_clip_ids.contains(&&props.clip.id);

        let border_width = if is_highlight { 2.0 } else { 1.0 };

        let fill_color = if is_highlight {
            Color::from_u8(255, 165, 0, 255)
        } else if clip.is_needed_to_update_position {
            Color::from_u8(255, 0, 255, 255)
        } else {
            Color::from_f01(0.4, 0.4, 0.8, 1.0)
            // Color::from_u8(0x48, 0xBF, 0xEF, 255)
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

        let circle_radius: f32 = (Time::from_ms(100.0) / time_per_pixel).into();

        let head_left_top = namui::Xy { x: 0.0, y: 0.0 };

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

        let clip_body_path = get_clip_body_path(&render_setting);

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
                    size: 10,
                },
                style: TextStyle {
                    color: Color::from_u8(0xFF, 0x00, 0xFF, 0xFF),
                    border: Some(TextStyleBorder {
                        color: Color::WHITE,
                        width: 1.0,
                    }),
                    ..Default::default()
                },
            }),
            false => RenderingTree::Empty,
        };

        translate(
            x,
            0.0,
            render![
                namui::path(clip_body_path.clone(), fill_paint),
                namui::path(clip_body_path.clone(), stroke_paint)
                    .with_mouse_cursor(MouseCursor::Grab)
                    .attach_event(|builder| {
                        let clip_id = props.clip.id.clone();
                        builder.on_mouse_down(move |event| {
                            namui::event::send(EditorEvent::SubtitleClipHeadMouseDownEvent {
                                mouse_event_id: event.id.clone(),
                                clip_id: clip_id.clone(),
                                click_in_time: timeline_start_at
                                    + PixelSize(event.local_xy.x + x) * time_per_pixel,
                            });
                        })
                    }),
                render_text_box(
                    &subtitle_text,
                    &head_circle_center_xy,
                    &tail_circle_center_xy,
                    circle_radius
                ),
                check_flag,
            ],
        )
    }
}

struct SubtitleClipBodyRenderSetting {
    circle_radius: f32,
    head_left_top: Xy<f32>,
    tail_right_bottom: Xy<f32>,
    head_height: f32,
    head_circle_center_xy: Xy<f32>,
    tail_height: f32,
    tail_circle_center_xy: Xy<f32>,
}

fn render_text_box(
    text: &str,
    head_circle_center_xy: &Xy<f32>,
    tail_circle_center_xy: &Xy<f32>,
    circle_radius: f32,
) -> RenderingTree {
    let horizontal_padding = 1.0;
    let vertical_padding = 1.0;
    let text_box_width =
        (tail_circle_center_xy - head_circle_center_xy).length() - horizontal_padding * 2.0;
    let text_box_height = circle_radius * 2.0 - vertical_padding * 2.0;

    let circle_to_circle_vector = tail_circle_center_xy - head_circle_center_xy;
    let radian = circle_to_circle_vector.y.atan2(circle_to_circle_vector.x);
    let text_box_left_top = head_circle_center_xy
        + Xy {
            x: horizontal_padding,
            y: -text_box_height / 2.0,
        };
    translate(
        text_box_left_top.x,
        text_box_left_top.y,
        rotate(
            radian,
            render![
                rect(RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: text_box_width,
                    height: text_box_height,
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: namui::Color::WHITE,
                        }),
                        ..Default::default()
                    },
                }),
                namui::text(TextParam {
                    x: 0.0,
                    y: text_box_height / 2.0,
                    text: text.to_string(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Middle,
                    font_type: FontType {
                        font_weight: FontWeight::REGULAR,
                        language: Language::Ko,
                        serif: false,
                        size: (text_box_height * 0.65) as i16,
                    },
                    style: TextStyle {
                        color: namui::Color::BLACK,
                        ..Default::default()
                    },
                }),
            ],
        ),
    )
}

fn get_clip_body_path(render_setting: &SubtitleClipBodyRenderSetting) -> PathBuilder {
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

    let length_to_head_center = (head_height.powf(2.0) + circle_radius.powf(2.0)).powf(0.5);
    let sin_of_head_half_circumcircle = head_height / length_to_head_center;
    let cos_of_head_half_circumcircle = circle_radius / length_to_head_center;
    let head_circumcircle_radian = sin_of_head_half_circumcircle.asin() * 2.0;

    let head_circle_to_tail_circle_vector = tail_circle_center_xy - head_circle_center_xy;
    let head_circle_to_tail_circle_radian =
        (head_circle_to_tail_circle_vector.y).atan2(head_circle_to_tail_circle_vector.x);

    let head_circle_oval_rect = LtrbRect {
        left: head_circle_center_xy.x - circle_radius,
        top: head_circle_center_xy.y - circle_radius,
        right: head_circle_center_xy.x + circle_radius,
        bottom: head_circle_center_xy.y + circle_radius,
    };

    let tail_circle_oval_rect = LtrbRect {
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

    let length_to_tail_center = (tail_height.powf(2.0) + circle_radius.powf(2.0)).powf(0.5);
    let sin_of_tail_half_circumcircle = head_height / length_to_tail_center;
    let cos_of_tail_half_circumcircle = circle_radius / length_to_tail_center;
    let tail_circumcircle_radian = sin_of_tail_half_circumcircle.asin() * 2.0;

    let tail_circle_left_point_of_contact_from_tail_right_bottom = Xy {
        x: tail_circle_center_xy.x
            - circle_radius * (2.0 * sin_of_tail_half_circumcircle.powf(2.0) - 1.0),
        y: tail_circle_center_xy.y
            + 2.0 * circle_radius * (sin_of_tail_half_circumcircle * cos_of_tail_half_circumcircle),
    };

    let mut clip_body_path = PathBuilder::new().move_to(head_left_top.x, head_left_top.y);

    let head_circle_right_point_of_contact_from_circle_to_circle_vector = head_circle_center_xy
        + Xy {
            x: circle_radius * (-(PI / 2.0 - head_circle_to_tail_circle_radian)).cos(),
            y: circle_radius * (-(PI / 2.0 - head_circle_to_tail_circle_radian)).sin(),
        };

    let head_top_arc_delta_radian =
        PI / 2.0 + head_circle_to_tail_circle_radian - head_circumcircle_radian;

    let is_need_to_draw_head_top_arc = head_top_arc_delta_radian > 0.0;
    if is_need_to_draw_head_top_arc {
        let start_radian = PI + head_circumcircle_radian;
        clip_body_path = clip_body_path
            .line_to(
                head_circle_right_point_of_contact_from_head_left_top.x,
                head_circle_right_point_of_contact_from_head_left_top.y,
            )
            .arc_to(
                &head_circle_oval_rect,
                start_radian,
                head_top_arc_delta_radian,
            );
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
            &tail_circle_oval_rect,
            -(PI / 2.0 - head_circle_to_tail_circle_radian),
            PI / 2.0 - head_circle_to_tail_circle_radian,
        )
        .line_to(tail_right_bottom.x, tail_right_bottom.y);

    let tail_circle_left_point_of_contact_from_circle_to_circle_vector = tail_circle_center_xy
        + Xy {
            x: -circle_radius * (PI / 2.0 - head_circle_to_tail_circle_radian).cos(),
            y: circle_radius * (PI / 2.0 - head_circle_to_tail_circle_radian).sin(),
        };

    let tail_bottom_arc_radian =
        PI / 2.0 + head_circle_to_tail_circle_radian - tail_circumcircle_radian;

    let is_need_to_draw_tail_bottom_arc = tail_bottom_arc_radian > 0.0;
    if is_need_to_draw_tail_bottom_arc {
        let start_radian = tail_circumcircle_radian;
        clip_body_path = clip_body_path
            .line_to(
                tail_circle_left_point_of_contact_from_tail_right_bottom.x,
                tail_circle_left_point_of_contact_from_tail_right_bottom.y,
            )
            .arc_to(&head_circle_oval_rect, start_radian, tail_bottom_arc_radian);
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
            &head_circle_oval_rect,
            PI / 2.0 + head_circle_to_tail_circle_radian,
            PI / 2.0 - head_circle_to_tail_circle_radian,
        )
        .line_to(head_left_top.x, head_left_top.y)
        .close();
    clip_body_path
}

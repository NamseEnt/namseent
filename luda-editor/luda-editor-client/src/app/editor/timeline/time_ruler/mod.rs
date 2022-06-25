use crate::app::{
    editor::events::EditorEvent,
    types::{PixelSize, Time, TimePerPixel},
};
use namui::prelude::*;
mod gradations;
use gradations::*;
mod time_text;
use time_text::*;

pub struct TimeRulerProps {
    pub xywh: XywhRect<f32>,
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}

pub struct Gradation {
    pub x: PixelSize,
    pub at: Time,
}

pub(super) fn render_time_ruler(props: &TimeRulerProps) -> RenderingTree {
    let gradation_gap_time =
        get_gradation_gap_time(PixelSize(100.0), PixelSize(500.0), props.time_per_pixel);

    let gradations = get_gradations(
        props.xywh.width.into(),
        gradation_gap_time,
        props.time_per_pixel,
        props.start_at,
    );

    translate(
        props.xywh.x,
        props.xywh.y,
        clip(
            PathBuilder::new().add_rect(
                &XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: props.xywh.width,
                    height: props.xywh.height,
                }
                .into_ltrb(),
            ),
            ClipOp::Intersect,
            render![
                rect(RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: props.xywh.width,
                    height: props.xywh.height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            border_position: BorderPosition::Inside,
                            color: Color::BLACK,
                            width: 1.0,
                        }),
                        fill: Some(RectFill {
                            color: Color::WHITE,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .attach_event(|event_builder| {
                    let time_per_pixel = props.time_per_pixel;
                    let start_at = props.start_at;
                    let time_ruler_dragging_closure = move |event: &MouseEvent| {
                        if !event.pressing_buttons.contains(&MouseButton::Left) {
                            return;
                        }
                        let click_position_in_time =
                            PixelSize(event.local_xy.x) * time_per_pixel + start_at;
                        namui::event::send(EditorEvent::TimelineTimeRulerClickEvent {
                            click_position_in_time,
                        });
                    };
                    event_builder
                        .on_mouse_down(time_ruler_dragging_closure)
                        .on_mouse_move_in(time_ruler_dragging_closure);
                }),
                render_time_texts(&TimeTextsProps {
                    gradations: &gradations,
                    height: props.xywh.height,
                    time_per_pixel: props.time_per_pixel,
                }),
                render_gradations(&GradationsProps {
                    wh: props.xywh.wh(),
                    gap_px: gradation_gap_time / props.time_per_pixel,
                    gradations: &gradations,
                }),
            ],
        ),
    )
}

fn get_gradation_gap_time(min: PixelSize, max: PixelSize, time_per_pixel: TimePerPixel) -> Time {
    [
        100, 250, 500, 1000, 5000, 10000, 30000, 60000, 300000, 600000, 1800000,
    ]
    .iter()
    .map(|&ms| Time::from_ms(ms as f32))
    .find(|&time| {
        let px = time / time_per_pixel;
        min <= px && px <= max
    })
    .unwrap_or(max * time_per_pixel)
}

/// NOTE: This code has been designed not to care about negative start_at.
fn get_gradations(
    time_ruler_width: PixelSize,
    gradation_gap_time: Time,
    time_per_pixel: TimePerPixel,
    start_at: Time,
) -> Vec<Gradation> {
    let gradation_start_at = start_at - (start_at % gradation_gap_time);
    let gradation_start_px = (gradation_start_at - start_at) / time_per_pixel;
    let gap_px = gradation_gap_time / time_per_pixel;

    let mut gradations = vec![];
    let mut index = 0;
    loop {
        let x = gradation_start_px + index * gap_px;
        if x >= time_ruler_width {
            break;
        }
        let at = gradation_start_at + (index * gradation_gap_time);
        gradations.push(Gradation { x, at });
        index += 1;
    }
    return gradations;
}

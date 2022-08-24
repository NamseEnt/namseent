mod gradations;
mod time_text;

use gradations::*;
use namui::prelude::*;
use time_text::*;

pub struct TimeRulerProps {
    pub rect: Rect<Px>,
    pub start_at: Time,
    pub time_per_px: TimePerPx,
}

pub struct Gradation {
    pub x: Px,
    pub at: Time,
}

pub enum Event {
    Click(Time),
}

pub(super) fn render_time_ruler(props: TimeRulerProps) -> RenderingTree {
    let gradation_gap_time = get_gradation_gap_time(px(100.0), px(500.0), props.time_per_px);

    let gradations = get_gradations(
        props.rect.width(),
        gradation_gap_time,
        props.time_per_px,
        props.start_at,
    );

    translate(
        props.rect.x(),
        props.rect.y(),
        clip(
            PathBuilder::new().add_rect(Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: props.rect.width(),
                height: props.rect.height(),
            }),
            ClipOp::Intersect,
            render([
                rect(RectParam {
                    rect: Rect::Xywh {
                        x: px(0.0),
                        y: px(0.0),
                        width: props.rect.width(),
                        height: props.rect.height(),
                    },
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            border_position: BorderPosition::Inside,
                            color: Color::WHITE,
                            width: px(1.0),
                        }),
                        fill: Some(RectFill {
                            color: Color::BLACK,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .attach_event(|event_builder| {
                    let time_per_px = props.time_per_px;
                    let start_at = props.start_at;
                    let time_ruler_dragging_closure = move |event: &MouseEvent| {
                        if !event.pressing_buttons.contains(&MouseButton::Left) {
                            return;
                        }
                        let click_position_in_time = event.local_xy.x * time_per_px + start_at;
                        namui::event::send(Event::Click(click_position_in_time));
                    };
                    event_builder
                        .on_mouse_down_in(time_ruler_dragging_closure)
                        .on_mouse_move_in(time_ruler_dragging_closure);
                }),
                render_time_texts(TimeTextsProps {
                    gradations: &gradations,
                    height: props.rect.height(),
                    time_per_px: props.time_per_px,
                }),
                render_gradations(&GradationsProps {
                    wh: props.rect.wh(),
                    gap_px: gradation_gap_time / props.time_per_px,
                    gradations: &gradations,
                }),
            ]),
        ),
    )
}

fn get_gradation_gap_time(min: Px, max: Px, time_per_px: TimePerPx) -> Time {
    [
        100, 250, 500, 1000, 5000, 10000, 30000, 60000, 300000, 600000, 1800000,
    ]
    .iter()
    .map(|&ms| Time::Ms(ms as f32))
    .find(|&time| {
        let px = time / time_per_px;
        min <= px && px <= max
    })
    .unwrap_or(max * time_per_px)
}

/// NOTE: This code has been designed not to care about negative start_at.
fn get_gradations(
    time_ruler_width: Px,
    gradation_gap_time: Time,
    time_per_px: TimePerPx,
    start_at: Time,
) -> Vec<Gradation> {
    let gradation_start_at = start_at - (start_at % gradation_gap_time);
    let gradation_start_px = (gradation_start_at - start_at) / time_per_px;
    let gap_px = gradation_gap_time / time_per_px;

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

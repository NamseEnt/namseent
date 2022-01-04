use namui::prelude::*;
mod track_body;
use super::TimelineRenderContext;
use crate::app::{
    editor::events::EditorEvent,
    types::{PixelSize, Track},
};
use track_body::*;

pub struct TimelineBody {}
pub struct TimelineBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub tracks: &'a Vec<Track>,
    pub context: &'a TimelineRenderContext<'a>,
}
impl TimelineBody {
    pub fn render(props: &TimelineBodyProps) -> RenderingTree {
        let track_body_height = 80.0; // TODO
        let track_bodies = props
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                namui::translate(
                    0.0,
                    track_body_height * index as f32,
                    TrackBody::render(&TrackBodyProps {
                        width: props.width,
                        height: track_body_height,
                        track,
                        context: props.context,
                    }),
                )
            })
            .collect::<Vec<_>>();
        let border_id = "timeline_border";
        let border = namui::rect(namui::RectParam {
            x: 0.0,
            y: 0.0,
            width: props.width,
            height: props.height,
            style: namui::RectStyle {
                fill: Some(namui::RectFill {
                    color: namui::Color::from_f01(0.4, 0.4, 0.4, 1.0),
                }),
                stroke: Some(namui::RectStroke {
                    color: namui::Color::BLACK,
                    width: 1.0,
                    border_position: namui::BorderPosition::Inside,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_id(border_id)
        .attach_event(move |builder| {
            let width = props.width;
            let height = props.height;
            builder.on_wheel(Box::new(move |event| {
                let managers = namui::managers();

                let mouse_manager = &managers.mouse_manager;
                let mouse_position = mouse_manager.mouse_position();
                let timeline_xy = event
                    .namui_context
                    .get_rendering_tree_xy(border_id)
                    .unwrap();

                let is_mouse_in_timeline = mouse_position.x as f32 >= timeline_xy.x
                    && mouse_position.x as f32 <= timeline_xy.x + width
                    && mouse_position.y as f32 >= timeline_xy.y
                    && mouse_position.y as f32 <= timeline_xy.y + height;
                if !is_mouse_in_timeline {
                    return;
                }

                let keyboard_manager = &managers.keyboard_manager;
                if keyboard_manager
                    .any_code_press(&[namui::Code::ShiftLeft, namui::Code::ShiftRight])
                {
                    namui::event::send(Box::new(EditorEvent::TimelineMoveEvent {
                        pixel: PixelSize(event.delta_xy.y),
                    }))
                } else if keyboard_manager
                    .any_code_press(&[namui::Code::AltLeft, namui::Code::AltRight])
                {
                    let anchor_x_in_timeline = PixelSize(mouse_position.x as f32 - timeline_xy.x);

                    namui::event::send(Box::new(EditorEvent::TimelineZoomEvent {
                        delta: event.delta_xy.y,
                        anchor_x_in_timeline,
                    }))
                }
            }))
        });
        render![
            border,
            namui::clip(
                namui::PathBuilder::new().add_rect(
                    &namui::XywhRect {
                        x: 0.0,
                        y: 0.0,
                        width: props.width,
                        height: props.height,
                    }
                    .into_ltrb()
                ),
                namui::ClipOp::Intersect,
                render![track_bodies],
            )
        ]
    }
}

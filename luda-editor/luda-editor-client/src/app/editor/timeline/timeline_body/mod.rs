use namui::prelude::*;
use std::sync::Arc;
pub mod track_body;
use super::TimelineRenderContext;
use crate::app::{
    editor::events::EditorEvent,
    types::{PixelSize, Time, Track},
};
use track_body::*;

pub struct TimelineBody {
    last_clip_clicked_mouse_event_id: Option<String>,
}
pub struct TimelineBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub tracks: &'a [Arc<Track>],
    pub context: &'a TimelineRenderContext<'a>,
}

struct TimelineBodyLeftClickEvent {
    pub mouse_event_id: String,
    pub mouse_position_in_time: Time,
}

impl TimelineBody {
    pub(crate) fn new() -> TimelineBody {
        TimelineBody {
            last_clip_clicked_mouse_event_id: None,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<TimelineBodyLeftClickEvent>() {
            let is_mouse_on_clip = if let Some(last_clip_clicked_mouse_event_id) =
                &self.last_clip_clicked_mouse_event_id
            {
                last_clip_clicked_mouse_event_id.eq(&event.mouse_event_id)
            } else {
                false
            };
            namui::event::send(EditorEvent::TimelineBodyLeftClickEvent {
                is_mouse_on_clip,
                mouse_position_in_time: event.mouse_position_in_time,
            });
        } else if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::ResizableClipBodyMouseDownEvent { mouse_event_id, .. }
                | EditorEvent::SubtitleClipHeadMouseDownEvent { mouse_event_id, .. } => {
                    self.last_clip_clicked_mouse_event_id = Some(mouse_event_id.clone());
                }
                _ => {}
            }
        }
    }
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
        .attach_event(move |builder| {
            let width = props.width;
            let height = props.height;
            let time_per_pixel = props.context.time_per_pixel;
            let start_at = props.context.start_at;
            let get_mouse_position_in_time =
                move |local_x| PixelSize(local_x) * time_per_pixel + start_at;
            builder
                .on_wheel(move |event| {
                    let managers = namui::managers();

                    let mouse_manager = &managers.mouse_manager;
                    let mouse_position = mouse_manager.mouse_position();
                    let timeline_xy = event
                        .namui_context
                        .get_rendering_tree_xy(event.target)
                        .expect("failed to get timeline xy");

                    let is_mouse_in_timeline = mouse_position.x as f32 >= timeline_xy.x
                        && mouse_position.x as f32 <= timeline_xy.x + width
                        && mouse_position.y as f32 >= timeline_xy.y
                        && mouse_position.y as f32 <= timeline_xy.y + height;
                    if !is_mouse_in_timeline {
                        return;
                    }

                    let keyboard_manager = &managers.keyboard_manager;
                    if keyboard_manager
                        .any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight])
                    {
                        namui::event::send(EditorEvent::TimelineMoveEvent {
                            pixel: PixelSize(event.delta_xy.y),
                        })
                    } else if keyboard_manager
                        .any_code_press([namui::Code::AltLeft, namui::Code::AltRight])
                    {
                        let anchor_x_in_timeline =
                            PixelSize(mouse_position.x as f32 - timeline_xy.x);

                        namui::event::send(EditorEvent::TimelineZoomEvent {
                            delta: event.delta_xy.y,
                            anchor_x_in_timeline,
                        })
                    }
                })
                .on_mouse_move_in(move |event| {
                    namui::event::send(EditorEvent::TimelineBodyMouseMoveEvent {
                        mouse_position_in_time: get_mouse_position_in_time(event.local_xy.x),
                    })
                })
                .on_mouse_down(move |event| {
                    if event.button == Some(MouseButton::Left) {
                        namui::event::send(TimelineBodyLeftClickEvent {
                            mouse_event_id: event.id.clone(),
                            mouse_position_in_time: get_mouse_position_in_time(event.local_xy.x),
                        })
                    }
                })
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

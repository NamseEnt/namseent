pub mod track_body;
use super::TimelineRenderContext;
use crate::app::{editor::events::EditorEvent, storage::GithubStorage, types::Track};
use namui::prelude::*;
use std::sync::Arc;
use track_body::*;

pub struct TimelineBody {
    last_clip_clicked_mouse_event_id: Option<String>,
}
pub struct TimelineBodyProps<'a> {
    pub width: Px,
    pub height: Px,
    pub tracks: &'a [Arc<Track>],
    pub context: &'a TimelineRenderContext<'a>,
    pub storage: Arc<dyn GithubStorage>,
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
    pub fn render(props: TimelineBodyProps) -> RenderingTree {
        let track_body_height = px(80.0); // TODO
        let track_bodies = props
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                namui::translate(
                    px(0.0),
                    track_body_height * index,
                    TrackBody::render(&TrackBodyProps {
                        width: props.width,
                        height: track_body_height,
                        track,
                        context: props.context,
                        storage: props.storage.clone(),
                    }),
                )
            })
            .collect::<Vec<_>>();
        let border = namui::rect(namui::RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: props.width,
                height: props.height,
            },
            style: namui::RectStyle {
                fill: Some(namui::RectFill {
                    color: namui::Color::from_f01(0.4, 0.4, 0.4, 1.0),
                }),
                stroke: Some(namui::RectStroke {
                    color: namui::Color::BLACK,
                    width: px(1.0),
                    border_position: namui::BorderPosition::Inside,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let width = props.width;
            let height = props.height;
            let time_per_px = props.context.time_per_px;
            let start_at = props.context.start_at;
            let get_mouse_position_in_time =
                move |local_x: Px| -> Time { local_x * time_per_px + start_at };
            builder
                .on_wheel(move |event| {
                    let mouse_position = namui::mouse::position();
                    let timeline_xy = event
                        .namui_context
                        .get_rendering_tree_xy(event.target)
                        .expect("failed to get timeline xy");

                    let is_mouse_in_timeline = mouse_position.x >= timeline_xy.x
                        && mouse_position.x <= timeline_xy.x + width
                        && mouse_position.y >= timeline_xy.y
                        && mouse_position.y <= timeline_xy.y + height;
                    if !is_mouse_in_timeline {
                        return;
                    }

                    if namui::keyboard::any_code_press([
                        namui::Code::ShiftLeft,
                        namui::Code::ShiftRight,
                    ]) {
                        namui::event::send(EditorEvent::TimelineMoveEvent {
                            px: px(event.delta_xy.y),
                        })
                    } else if namui::keyboard::any_code_press([
                        namui::Code::AltLeft,
                        namui::Code::AltRight,
                    ]) {
                        let anchor_x_in_timeline = mouse_position.x - timeline_xy.x;

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
                .on_mouse_down_in(move |event| {
                    if event.button == Some(MouseButton::Left) {
                        namui::event::send(TimelineBodyLeftClickEvent {
                            mouse_event_id: event.id.clone(),
                            mouse_position_in_time: get_mouse_position_in_time(event.local_xy.x),
                        })
                    }
                });
        });
        render([
            border,
            namui::clip(
                namui::PathBuilder::new().add_rect(namui::Rect::Xywh {
                    x: px(0.0),
                    y: px(0.0),
                    width: props.width,
                    height: props.height,
                }),
                namui::ClipOp::Intersect,
                render(track_bodies),
            ),
        ])
    }
}

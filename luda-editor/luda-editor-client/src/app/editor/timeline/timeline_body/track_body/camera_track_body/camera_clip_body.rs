use crate::app::{
    editor::{events::EditorEvent, TimelineRenderContext},
    types::CameraClip,
};
use namui::prelude::*;

pub struct CameraClipBody {}
pub struct CameraClipBodyProps<'a> {
    pub track_body_wh: &'a Wh<f32>,
    pub clip: &'a CameraClip,
    pub context: &'a TimelineRenderContext<'a>,
}
impl CameraClipBody {
    pub fn render(props: &CameraClipBodyProps) -> RenderingTree {
        let x = ((props.clip.start_at - props.context.start_at) / props.context.time_per_pixel).0;
        let duration = props.clip.end_at - props.clip.start_at;
        let width = (duration / props.context.time_per_pixel).0;

        let clip_rect = namui::XywhRect {
            x: x + 1.0,
            y: 1.0,
            width: width - 2.0,
            height: props.track_body_wh.height - 2.0,
        };
        let is_highlight = props
            .context
            .selected_clip_id
            .as_ref()
            .map_or(false, |id| id.eq(&props.clip.id));

        render![namui::rect(namui::RectParam {
            x: clip_rect.x,
            y: clip_rect.y,
            width: clip_rect.width,
            height: clip_rect.height,
            style: namui::RectStyle {
                fill: Some(namui::RectFill {
                    color: namui::Color::from_f01(0.4, 0.4, 0.8, 1.0),
                }),
                stroke: Some(if is_highlight {
                    namui::RectStroke {
                        color: namui::Color::RED,
                        width: 3.0,
                        border_position: namui::BorderPosition::Inside,
                    }
                } else {
                    namui::RectStroke {
                        color: namui::Color::BLACK,
                        width: 1.0,
                        border_position: namui::BorderPosition::Inside,
                    }
                }),
                round: Some(namui::RectRound { radius: 5.0 }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let clip_id = props.clip.id.clone();
            builder.on_mouse_down(Box::new(move |event| {
                let event = EditorEvent::CameraClipBodyMouseDownEvent {
                    clip_id: clip_id.clone(),
                    local_mouse_xy: event.local_xy,
                    global_mouse_xy: event.global_xy,
                };
                namui::event::send(Box::new(event));
            }))
        })]
    }
}

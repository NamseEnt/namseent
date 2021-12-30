use crate::editor::{job::Job, types::Track};
use namui::prelude::*;
mod track_body;
use super::TimelineRenderContext;
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

        render![
            namui::rect(namui::RectParam {
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
            }),
            namui::clip(
                namui::Path::new().add_rect(
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
            ),
            // setWheelZoomHandler(state.timelineState),
            // setWheelMoveHandler(state.timelineState),
        ]
    }
}

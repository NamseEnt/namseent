use crate::editor::{
    timeline::track_header::{TrackHeader, TrackHeaderProps},
    types::Track,
};
use ::namui::*;

pub struct TimelineHeader {}
pub struct TimelineHeaderProps<'a> {
    pub width: f32,
    pub height: f32,
    pub tracks: &'a Vec<Track>,
}
impl TimelineHeader {
    pub fn render(props: &TimelineHeaderProps) -> RenderingTree {
        let track_header_height = 80.0; // TODO
        let track_headers = props
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                namui::translate(
                    0.0,
                    track_header_height * index as f32,
                    (TrackHeader {}).render(&TrackHeaderProps {
                        width: props.width,
                        height: track_header_height,
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
            track_headers,
        ]
    }
}

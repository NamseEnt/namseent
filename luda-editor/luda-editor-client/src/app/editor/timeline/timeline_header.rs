use crate::app::{
    editor::timeline::track_header::{TrackHeader, TrackHeaderProps},
    types::Track,
};
use namui::prelude::*;
use std::sync::Arc;

pub struct TimelineHeader {}
pub struct TimelineHeaderProps<'a> {
    pub width: Px,
    pub height: Px,
    pub tracks: &'a [Arc<Track>],
}
impl TimelineHeader {
    pub fn render(props: TimelineHeaderProps) -> RenderingTree {
        let track_header_height = px(80.0); // TODO
        let track_headers = props
            .tracks
            .iter()
            .enumerate()
            .map(|(index, _track)| {
                namui::translate(
                    px(0.0),
                    track_header_height * index,
                    (TrackHeader {}).render(&TrackHeaderProps {
                        width: props.width,
                        height: track_header_height,
                    }),
                )
            })
            .collect::<Vec<_>>();

        render([
            namui::rect(namui::RectParam {
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
            }),
            render(track_headers),
        ])
    }
}

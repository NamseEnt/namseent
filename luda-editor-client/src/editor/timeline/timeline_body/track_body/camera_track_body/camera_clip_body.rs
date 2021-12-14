use crate::editor::{
    types::{CameraClip, Track},
    Timeline,
};
use ::namui::*;

pub struct CameraClipBody {}
pub struct CameraClipBodyProps<'a> {
    pub width: f32,
    pub height: f32,
    pub clip: &'a CameraClip,
    pub timeline: &'a Timeline,
}
impl CameraClipBody {
    pub fn render(props: &CameraClipBodyProps) -> RenderingTree {
        let x = ((props.clip.start_at - props.timeline.start_at) / props.timeline.time_per_pixel).0;
        let duration = props.clip.end_at - props.clip.start_at;
        let width = (duration / props.timeline.time_per_pixel).0;

        let clip_rect = namui::XywhRect {
            x: x + 1.0,
            y: 1.0,
            width: width - 2.0,
            height: props.height - 2.0,
        };
        let is_highlight = false; // TODO

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
        })]
    }
}

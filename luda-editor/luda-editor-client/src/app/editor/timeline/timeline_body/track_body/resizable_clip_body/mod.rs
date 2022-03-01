use crate::app::{
    editor::{events::EditorEvent, TimelineRenderContext},
    types::*,
};
use namui::prelude::*;
mod sash;
pub use sash::*;

pub struct ResizableClipBody {}
pub struct ResizableClipBodyProps<'a> {
    pub track_body_wh: &'a Wh<f32>,
    pub clip: &'a dyn ResizableClip,
    pub context: &'a TimelineRenderContext<'a>,
}
const RESIZABLE_CLIP_ROUND_RADIUS: f32 = 5.0;

pub trait ResizableClip {
    fn id(&self) -> String;
    fn start_at(&self) -> Time;
    fn end_at(&self) -> Time;
    fn render(&self, wh: &Wh<f32>) -> RenderingTree;
}

#[derive(Debug, Clone, Copy)]
pub enum ResizableClipBodyPart {
    Sash(SashDirection),
    Body,
}

/// NOTE : No left sash yet. it's intended to be added later if it's really needed.
const AVAILABLE_SASH_DIRECTIONS: [sash::SashDirection; 1] = [SashDirection::Right];

impl ResizableClipBody {
    pub fn render(props: &ResizableClipBodyProps) -> RenderingTree {
        let clip_id = props.clip.id();
        let timeline_start_at = props.context.start_at;
        let time_per_pixel = props.context.time_per_pixel;

        let x = ((props.clip.start_at() - timeline_start_at) / time_per_pixel).0;
        let duration = props.clip.end_at() - props.clip.start_at();
        let width = (duration / time_per_pixel).0;

        let clip_rect = namui::XywhRect {
            x: x + 1.0,
            y: 1.0,
            width: width - 2.0,
            height: props.track_body_wh.height - 2.0,
        };
        let is_selected = props.context.selected_clip_ids.contains(&&clip_id);
        let is_highlight = is_selected;
        let is_sashes_showing = is_selected && props.context.selected_clip_ids.len() == 1;

        let background = namui::rect(namui::RectParam {
            x: clip_rect.x,
            y: clip_rect.y,
            width: clip_rect.width,
            height: clip_rect.height,
            style: namui::RectStyle {
                fill: Some(namui::RectFill {
                    color: if is_highlight {
                        namui::Color::from_f01(0.8, 0.6, 0.8, 1.0)
                    } else {
                        namui::Color::from_f01(0.4, 0.4, 0.8, 1.0)
                    },
                }),
                round: Some(namui::RectRound {
                    radius: RESIZABLE_CLIP_ROUND_RADIUS,
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        let border = namui::rect(namui::RectParam {
            x: clip_rect.x,
            y: clip_rect.y,
            width: clip_rect.width,
            height: clip_rect.height,
            style: namui::RectStyle {
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
                round: Some(namui::RectRound {
                    radius: RESIZABLE_CLIP_ROUND_RADIUS,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let clip_id = props.clip.id();
            builder.on_mouse_down(move |event| {
                let clicked_part = if is_sashes_showing {
                    AVAILABLE_SASH_DIRECTIONS
                        .iter()
                        .find_map(|direction| {
                            let sash_rect = get_sash_rect(&clip_rect, *direction);
                            if sash_rect.is_xy_in(&event.local_xy) {
                                Some(ResizableClipBodyPart::Sash(*direction))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| ResizableClipBodyPart::Body)
                } else {
                    ResizableClipBodyPart::Body
                };

                namui::event::send(EditorEvent::ResizableClipBodyMouseDownEvent {
                    mouse_event_id: event.id.clone(),
                    clip_id: clip_id.clone(),
                    click_in_time: timeline_start_at + PixelSize(event.local_xy.x) * time_per_pixel,
                    clicked_part,
                });
            })
        });

        let sashes = if is_sashes_showing {
            RenderingTree::Children(
                AVAILABLE_SASH_DIRECTIONS
                    .iter()
                    .map(|direction| {
                        render_sash(&SashBodyProps {
                            context: props.context,
                            direction: *direction,
                            clip_rect: &clip_rect,
                        })
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            RenderingTree::Empty
        };

        namui::render![
            background,
            render_resizable_clip_preview(&clip_rect, props.track_body_wh.width, props.clip),
            border,
            sashes,
        ]
    }
}

fn render_resizable_clip_preview(
    resizable_clip_rect: &XywhRect<f32>,
    track_body_width: f32,
    clip: &dyn ResizableClip,
) -> RenderingTree {
    let xywh: XywhRect<f32> =
        get_resizable_clip_preview_xywh(resizable_clip_rect, track_body_width);

    if xywh.width <= 8.0 {
        return RenderingTree::Empty;
    }
    let width_by_fixed_height = xywh.height * 16.0 / 9.0;

    let letter_box_half_width = (width_by_fixed_height - xywh.width) / 2.0;
    let background = rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: width_by_fixed_height,
        height: xywh.height,
        style: RectStyle {
            fill: Some(RectFill {
                color: Color::WHITE,
            }),

            ..Default::default()
        },
        ..Default::default()
    });

    translate(
        xywh.x - letter_box_half_width,
        xywh.y,
        namui::clip(
            PathBuilder::new().add_rrect(
                &LtrbRect {
                    left: letter_box_half_width,
                    top: 0.0,
                    right: xywh.width + letter_box_half_width,
                    bottom: xywh.height,
                },
                RESIZABLE_CLIP_ROUND_RADIUS,
                RESIZABLE_CLIP_ROUND_RADIUS,
            ),
            ClipOp::Intersect,
            render![
                background,
                clip.render(&Wh {
                    width: width_by_fixed_height,
                    height: xywh.height,
                },),
            ],
        ),
    )
}

fn get_resizable_clip_preview_xywh(
    resizable_clip_rect: &XywhRect<f32>,
    track_body_width: f32,
) -> XywhRect<f32> {
    // NOTE : The coordinate is based on the timeline.start_at as a zero point.
    let resizable_clip_right = resizable_clip_rect.x + resizable_clip_rect.width;
    let preview_right = resizable_clip_right.min(track_body_width);
    let preview_x = resizable_clip_rect.x.max(0.0);
    let preview_width = preview_right - preview_x;
    XywhRect {
        x: preview_x,
        y: resizable_clip_rect.y,
        width: preview_width,
        height: resizable_clip_rect.height,
    }
}

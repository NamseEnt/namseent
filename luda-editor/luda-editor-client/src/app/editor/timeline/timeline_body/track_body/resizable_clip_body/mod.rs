mod sash;
use crate::app::{
    editor::{events::EditorEvent, TimelineRenderContext},
    storage::Storage,
};
use namui::prelude::*;
pub use sash::*;
use std::sync::Arc;

pub struct ResizableClipBody {}
pub struct ResizableClipBodyProps<'a> {
    pub track_body_wh: Wh<Px>,
    pub clip: &'a dyn ResizableClip,
    pub context: &'a TimelineRenderContext<'a>,
    pub storage: Arc<Storage>,
}
const RESIZABLE_CLIP_ROUND_RADIUS: Px = px(5.0);

pub trait ResizableClip {
    fn id(&self) -> String;
    fn start_at(&self) -> Time;
    fn end_at(&self) -> Time;
    fn render(&self, wh: Wh<Px>, storage: Arc<Storage>) -> RenderingTree;
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
        let time_per_px = props.context.time_per_px;

        let x = (props.clip.start_at() - timeline_start_at) / time_per_px;
        let duration = props.clip.end_at() - props.clip.start_at();
        let width = duration / time_per_px;

        let clip_rect = namui::Rect::Xywh {
            x: x + px(1.0),
            y: px(1.0),
            width: width - px(2.0),
            height: props.track_body_wh.height - px(2.0),
        };
        let is_selected = props.context.selected_clip_ids.contains(&&clip_id);
        let is_highlight = is_selected;
        let is_sashes_showing = is_selected && props.context.selected_clip_ids.len() == 1;

        let background = namui::rect(namui::RectParam {
            rect: clip_rect,
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
            rect: Rect::Xywh {
                x: clip_rect.x(),
                y: clip_rect.y(),
                width: clip_rect.width(),
                height: clip_rect.height(),
            },
            style: namui::RectStyle {
                stroke: Some(if is_highlight {
                    namui::RectStroke {
                        color: namui::Color::RED,
                        width: px(3.0),
                        border_position: namui::BorderPosition::Inside,
                    }
                } else {
                    namui::RectStroke {
                        color: namui::Color::BLACK,
                        width: px(1.0),
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
            builder.on_mouse_down_in(move |event| {
                let clicked_part = if is_sashes_showing {
                    AVAILABLE_SASH_DIRECTIONS
                        .iter()
                        .find_map(|direction| {
                            let sash_rect = get_sash_rect(clip_rect, *direction);
                            if sash_rect.is_xy_inside(event.local_xy) {
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
                    click_in_time: timeline_start_at + event.local_xy.x * time_per_px,
                    clicked_part,
                });
            });
        });

        let sashes = if is_sashes_showing {
            RenderingTree::Children(
                AVAILABLE_SASH_DIRECTIONS
                    .iter()
                    .map(|direction| {
                        render_sash(&SashBodyProps {
                            direction: *direction,
                            clip_rect,
                        })
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            RenderingTree::Empty
        };

        namui::render([
            background,
            render_resizable_clip_preview(
                clip_rect,
                props.track_body_wh.width,
                props.clip,
                props.storage.clone(),
            ),
            border,
            sashes,
        ])
    }
}

fn render_resizable_clip_preview(
    resizable_clip_rect: Rect<Px>,
    track_body_width: Px,
    clip: &dyn ResizableClip,
    storage: Arc<Storage>,
) -> RenderingTree {
    let rect: Rect<Px> = get_resizable_clip_preview_rect(resizable_clip_rect, track_body_width);

    if rect.width() <= px(8.0) {
        return RenderingTree::Empty;
    }
    let width_by_fixed_height = rect.height() * 16.0 / 9.0;

    let letter_box_half_width = (width_by_fixed_height - rect.width()) / 2.0;
    let background = namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: width_by_fixed_height,
            height: rect.height(),
        },
        style: RectStyle {
            fill: Some(RectFill {
                color: Color::WHITE,
            }),

            ..Default::default()
        },
        ..Default::default()
    });

    translate(
        rect.x() - letter_box_half_width,
        rect.y(),
        namui::clip(
            PathBuilder::new().add_rrect(
                Rect::Ltrb {
                    left: letter_box_half_width,
                    top: px(0.0),
                    right: rect.width() + letter_box_half_width,
                    bottom: rect.height(),
                },
                RESIZABLE_CLIP_ROUND_RADIUS,
                RESIZABLE_CLIP_ROUND_RADIUS,
            ),
            ClipOp::Intersect,
            render([
                background,
                clip.render(
                    Wh {
                        width: width_by_fixed_height,
                        height: rect.height(),
                    },
                    storage,
                ),
            ]),
        ),
    )
}

fn get_resizable_clip_preview_rect(
    resizable_clip_rect: Rect<Px>,
    track_body_width: Px,
) -> Rect<Px> {
    // NOTE : The coordinate is based on the timeline.start_at as a zero point.
    let resizable_clip_right = resizable_clip_rect.x() + resizable_clip_rect.width();
    let preview_right = resizable_clip_right.min(track_body_width);
    let preview_x = resizable_clip_rect.x().max(px(0.0));
    let preview_width = preview_right - preview_x;
    Rect::Xywh {
        x: preview_x,
        y: resizable_clip_rect.y(),
        width: preview_width,
        height: resizable_clip_rect.height(),
    }
}

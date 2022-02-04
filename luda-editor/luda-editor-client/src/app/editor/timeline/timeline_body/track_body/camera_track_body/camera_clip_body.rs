use crate::app::{
    editor::{events::EditorEvent, TimelineRenderContext},
    types::*,
};
use namui::prelude::*;

pub struct CameraClipBody {}
pub struct CameraClipBodyProps<'a> {
    pub track_body_wh: &'a Wh<f32>,
    pub clip: &'a CameraClip,
    pub context: &'a TimelineRenderContext<'a>,
}
const CAMERA_CLIP_ROUND_RADIUS: f32 = 5.0;

impl CameraClipBody {
    pub fn render(props: &CameraClipBodyProps) -> RenderingTree {
        let timeline_start_at = props.context.start_at;
        let time_per_pixel = props.context.time_per_pixel;

        let x = ((props.clip.start_at - timeline_start_at) / time_per_pixel).0;
        let duration = props.clip.end_at - props.clip.start_at;
        let width = (duration / time_per_pixel).0;

        let clip_rect = namui::XywhRect {
            x: x + 1.0,
            y: 1.0,
            width: width - 2.0,
            height: props.track_body_wh.height - 2.0,
        };
        let is_highlight = props.context.selected_clip_ids.contains(&&props.clip.id);

        let background = namui::rect(namui::RectParam {
            x: clip_rect.x,
            y: clip_rect.y,
            width: clip_rect.width,
            height: clip_rect.height,
            style: namui::RectStyle {
                fill: Some(namui::RectFill {
                    color: namui::Color::from_f01(0.4, 0.4, 0.8, 1.0),
                }),
                round: Some(namui::RectRound {
                    radius: CAMERA_CLIP_ROUND_RADIUS,
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
                    radius: CAMERA_CLIP_ROUND_RADIUS,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let clip_id = props.clip.id.clone();
            builder.on_mouse_down(move |event| {
                let event = EditorEvent::CameraClipBodyMouseDownEvent {
                    clip_id: clip_id.clone(),
                    click_in_time: timeline_start_at + PixelSize(event.local_xy.x) * time_per_pixel,
                };
                namui::event::send(event);
            })
        });

        namui::render![
            background,
            render_camera_clip_preview(
                &clip_rect,
                props.track_body_wh.width,
                &props.clip.camera_angle,
                LudaEditorServerCameraAngleImageLoader {},
            ),
            border,
        ]
    }
}

fn render_camera_clip_preview(
    camera_clip_rect: &XywhRect<f32>,
    track_body_width: f32,
    camera_angle: &CameraAngle,
    camera_angle_image_loader: impl CameraAngleImageLoader,
) -> RenderingTree {
    let xywh: XywhRect<f32> = get_camera_clip_preview_xywh(camera_clip_rect, track_body_width);

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
        clip(
            PathBuilder::new().add_rrect(
                &LtrbRect {
                    left: letter_box_half_width,
                    top: 0.0,
                    right: xywh.width + letter_box_half_width,
                    bottom: xywh.height,
                },
                CAMERA_CLIP_ROUND_RADIUS,
                CAMERA_CLIP_ROUND_RADIUS,
            ),
            ClipOp::Intersect,
            render![
                background,
                camera_angle.render(
                    &Wh {
                        width: width_by_fixed_height,
                        height: xywh.height,
                    },
                    &camera_angle_image_loader,
                ),
            ],
        ),
    )
}

fn get_camera_clip_preview_xywh(
    camera_clip_rect: &XywhRect<f32>,
    track_body_width: f32,
) -> XywhRect<f32> {
    // NOTE : The coordinate is based on the timeline.start_at as a zero point.
    let camera_clip_right = camera_clip_rect.x + camera_clip_rect.width;
    let preview_right = camera_clip_right.min(track_body_width);
    let preview_x = camera_clip_rect.x.max(0.0);
    let preview_width = preview_right - preview_x;
    XywhRect {
        x: preview_x,
        y: camera_clip_rect.y,
        width: preview_width,
        height: camera_clip_rect.height,
    }
}

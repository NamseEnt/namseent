use crate::app::editor::timeline::timeline_body::track_body::ResizableClipBodyPart;
use crate::app::types::*;
use std::sync::Arc;

pub enum EditorEvent {
    ResizableClipBodyMouseDownEvent {
        mouse_event_id: String,
        clip_id: String,
        click_in_time: Time,
        clicked_part: ResizableClipBodyPart,
    },
    SubtitleClipHeadMouseDownEvent {
        mouse_event_id: String,
        clip_id: String,
        click_in_time: Time,
    },
    ScrolledEvent {
        scroll_y: f32,
    },
    TimelineMoveEvent {
        pixel: PixelSize,
    },
    TimelineZoomEvent {
        delta: f32,
        anchor_x_in_timeline: PixelSize,
    },
    TimelineTimeRulerClickEvent {
        click_position_in_time: Time,
    },
    TimelineBodyMouseMoveEvent {
        mouse_position_in_time: Time,
    },
    TimelineBodyLeftClickEvent {
        is_mouse_on_clip: bool,
        mouse_position_in_time: Time,
    },
    SequenceUpdateEvent {
        sequence: Arc<Sequence>,
    },
    CameraTrackBodyRightClickEvent {
        mouse_global_xy: namui::Xy<f32>,
        mouse_position_in_time: Time,
    },
    SubtitleSyncRequestEvent {
        subtitles: Vec<Subtitle>,
    },
    CameraClipUpdateEvent {
        clip_id: String,
        update: Arc<dyn Fn(&CameraClip) -> CameraClip + Send + Sync>,
    },
}

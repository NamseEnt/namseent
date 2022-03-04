use super::clip_editor::camera_clip_editor::image_browser::ImageBrowserFile;
use super::clip_editor::camera_clip_editor::wysiwyg_editor::cropper::CropperHandle;
use super::clip_editor::camera_clip_editor::wysiwyg_editor::ResizerHandle;
use super::clip_editor::camera_clip_editor::WysiwygTarget;
use crate::app::editor::timeline::timeline_body::track_body::ResizableClipBodyPart;
use crate::app::types::*;
use std::collections::BTreeSet;
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
    CharacterImageFilesUpdatedEvent {
        character_image_files: BTreeSet<ImageBrowserFile>,
    },
    BackgroundImageFilesUpdatedEvent {
        background_image_files: BTreeSet<ImageBrowserFile>,
    },
    CharacterImageBrowserSelectEvent {
        character_pose_emotion: Option<CharacterPoseEmotion>,
    },
    BackgroundImageBrowserSelectEvent {
        background_name: Option<String>,
    },
    ScrolledEvent {
        scroll_y: f32,
    },
    WysiwygEditorInnerImageMouseDownEvent {
        target: WysiwygTarget,
        mouse_xy: namui::Xy<f32>,
        container_size: namui::Wh<f32>,
    },
    WysiwygEditorResizerHandleMouseDownEvent {
        target: WysiwygTarget,
        mouse_xy: namui::Xy<f32>,
        handle: ResizerHandle,
        center_xy: namui::Xy<f32>,
        container_size: namui::Wh<f32>,
        image_size_ratio: namui::Wh<f32>,
    },
    CharacterWysiwygEditorCropperHandleMouseDownEvent {
        mouse_xy: namui::Xy<f32>,
        handle: CropperHandle,
        container_size: namui::Wh<f32>,
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
}

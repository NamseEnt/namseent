use super::clip_editor::camera_clip_editor::wysiwyg_editor::{
    cropper::CropperHandle, resizer::ResizerHandle,
};
use crate::app::editor::clip_editor::camera_clip_editor::image_browser::ImageBrowserItem;
use crate::app::types::*;
use std::sync::Arc;

pub enum EditorEvent {
    CameraClipBodyMouseDownEvent {
        clip_id: String,
        click_in_time: Time,
    },
    SubtitleClipHeadMouseDownEvent {
        clip_id: String,
        click_in_time: Time,
    },
    ImageFilenameObjectsUpdatedEvent {
        image_filename_objects: Vec<ImageFilenameObject>,
    },
    ImageBrowserSelectEvent {
        selected_item: ImageBrowserItem,
    },
    ScrolledEvent {
        scroll_y: f32,
    },
    WysiwygEditorInnerImageMouseDownEvent {
        mouse_xy: namui::Xy<f32>,
        container_size: namui::Wh<f32>,
    },
    WysiwygEditorResizerHandleMouseDownEvent {
        mouse_xy: namui::Xy<f32>,
        handle: ResizerHandle,
        center_xy: namui::Xy<f32>,
        container_size: namui::Wh<f32>,
        image_size_ratio: namui::Wh<f32>,
    },
    WysiwygEditorCropperHandleMouseDownEvent {
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
    SequenceUpdateEvent {
        sequence: Arc<Sequence>,
    },
    CameraTrackBodyRightClickEvent {
        mouse_global_xy: namui::Xy<f32>,
        mouse_position_in_time: Time,
    },
}

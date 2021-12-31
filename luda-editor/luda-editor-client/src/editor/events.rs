use super::{
    clip_editor::camera_clip_editor::wysiwyg_editor::{
        cropper::CropperHandle, resizer::ResizerHandle,
    },
    types::ImageFilenameObject,
};

pub enum EditorEvent {
    CameraClipBodyMouseDownEvent {
        clip_id: String,
        local_mouse_xy: namui::Xy<f32>,
        global_mouse_xy: namui::Xy<f32>,
    },
    ImageFilenameObjectsUpdatedEvent {
        image_filename_objects: Vec<ImageFilenameObject>,
    },
    ImageBrowserSelectEvent {
        selected_key: String,
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
}

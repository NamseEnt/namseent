use super::types::ImageFilenameObject;

pub enum EditorEvent {
    CameraClipBodyMouseDownEvent {
        clip_id: String,
        local_mouse_xy: namui::Xy<f32>,
        global_mouse_xy: namui::Xy<f32>,
    },
    ImageFilenameObjectsUpdatedEvent {
        image_filename_objects: Vec<ImageFilenameObject>,
    },
}

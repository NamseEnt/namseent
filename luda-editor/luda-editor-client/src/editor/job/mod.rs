mod move_camera_clip;
pub use self::move_camera_clip::*;
mod wysiwyg_move_image;
pub use self::wysiwyg_move_image::*;
mod wysiwyg_resize_image;
pub use self::wysiwyg_resize_image::*;

#[derive(Debug, Clone)]
pub enum Job {
    MoveCameraClip(MoveCameraClipJob),
    WysiwygMoveImage(WysiwygMoveImageJob),
    WysiwygResizeImage(WysiwygResizeImageJob),
}

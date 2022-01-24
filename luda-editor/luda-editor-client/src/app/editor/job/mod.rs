use crate::app::types::Sequence;
mod move_camera_clip;
pub use self::move_camera_clip::*;
// mod move_subtitle_clip;
// pub use self::move_subtitle_clip::*;
// mod wysiwyg_move_image;
// pub use self::wysiwyg_move_image::*;
// mod wysiwyg_resize_image;
// pub use self::wysiwyg_resize_image::*;
// mod wysiwyg_crop_image;
// pub use self::wysiwyg_crop_image::*;

#[derive(Debug, Clone)]
pub enum Job {
    MoveCameraClip(MoveCameraClipJob),
    // MoveSubtitleClip(MoveSubtitleClipJob),
    // WysiwygMoveImage(WysiwygMoveImageJob),
    // WysiwygResizeImage(WysiwygResizeImageJob),
    // WysiwygCropImage(WysiwygCropImageJob),
}

impl Job {
    pub fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        match self {
            Job::MoveCameraClip(job) => job.execute(sequence),
            // Job::MoveSubtitleClip(job) => job.execute(sequence),
            // Job::WysiwygMoveImage(job) => job.execute(sequence),
            // Job::WysiwygResizeImage(job) => job.execute(sequence),
            // Job::WysiwygCropImage(job) => job.execute(sequence),
        }
    }
}

use crate::app::types::Sequence;
mod move_camera_clip;
pub use self::move_camera_clip::*;
mod move_subtitle_clip;
pub use self::move_subtitle_clip::*;
mod wysiwyg_move_image;
pub use self::wysiwyg_move_image::*;
mod wysiwyg_resize_image;
pub use self::wysiwyg_resize_image::*;
mod wysiwyg_crop_image;
pub use self::wysiwyg_crop_image::*;
mod change_image;
pub use self::change_image::*;

#[derive(Debug, Clone)]
pub enum Job {
    MoveCameraClip(MoveCameraClipJob),
    MoveSubtitleClip(MoveSubtitleClipJob),
    WysiwygMoveImage(WysiwygMoveImageJob),
    WysiwygResizeImage(WysiwygResizeImageJob),
    WysiwygCropImage(WysiwygCropImageJob),
    ChangeImage(ChangeImageJob),
}

impl Job {
    pub fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let job_execute: &dyn JobExecute = match self {
            Job::MoveCameraClip(job) => job,
            Job::MoveSubtitleClip(job) => job,
            Job::WysiwygMoveImage(job) => job,
            Job::WysiwygResizeImage(job) => job,
            Job::WysiwygCropImage(job) => job,
            Job::ChangeImage(job) => job,
        };
        job_execute.execute(sequence)
    }
}

trait JobExecute {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String>;
}

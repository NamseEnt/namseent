use crate::app::types::Sequence;
mod move_clip;
pub use self::move_clip::*;
mod wysiwyg_move_image;
pub use self::wysiwyg_move_image::*;
mod wysiwyg_resize_image;
pub use self::wysiwyg_resize_image::*;
mod wysiwyg_crop_image;
pub use self::wysiwyg_crop_image::*;
mod change_image;
pub use self::change_image::*;
mod add_background_clip;
pub use self::add_background_clip::*;
mod add_camera_clip;
pub use self::add_camera_clip::*;
mod resize_clip;
pub use self::resize_clip::*;
mod delete_camera_clip;
pub use self::delete_camera_clip::*;
mod sync_subtitles;
pub use self::sync_subtitles::*;
#[cfg(test)]
pub mod test_utils;
#[cfg(test)]
pub use test_utils::*;

#[derive(Debug, Clone)]
pub enum Job {
    MoveClip(MoveClipJob),
    WysiwygMoveImage(WysiwygMoveImageJob),
    WysiwygResizeImage(WysiwygResizeImageJob),
    WysiwygCropImage(WysiwygCropImageJob),
    ChangeImage(ChangeImageJob),
    AddBackgroundClip(AddBackgroundClipJob),
    AddCameraClip(AddCameraClipJob),
    ResizeClip(ResizeClipJob),
    DeleteCameraClip(DeleteCameraClipJob),
    SyncSubtitles(SyncSubtitlesJob),
}

impl Job {
    pub fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let job_execute: &dyn JobExecute = match self {
            Job::MoveClip(job) => job,
            Job::WysiwygMoveImage(job) => job,
            Job::WysiwygResizeImage(job) => job,
            Job::WysiwygCropImage(job) => job,
            Job::ChangeImage(job) => job,
            Job::AddBackgroundClip(job) => job,
            Job::AddCameraClip(job) => job,
            Job::ResizeClip(job) => job,
            Job::DeleteCameraClip(job) => job,
            Job::SyncSubtitles(job) => job,
        };
        job_execute.execute(sequence)
    }
}

pub trait JobExecute {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String>;
}

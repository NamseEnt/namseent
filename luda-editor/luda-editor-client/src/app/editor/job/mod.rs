use crate::app::types::Sequence;
mod move_clip;
pub use self::move_clip::*;
mod add_camera_clip;
pub use self::add_camera_clip::*;
mod resize_clip;
pub use self::resize_clip::*;
mod delete_camera_clip;
pub use self::delete_camera_clip::*;
mod sync_subtitles;
pub use self::sync_subtitles::*;
mod update_camera_clip;
pub use self::update_camera_clip::*;
#[cfg(test)]
pub mod test_utils;
#[cfg(test)]
pub use test_utils::*;

pub enum Job {
    MoveClip(MoveClipJob),
    AddCameraClip(AddCameraClipJob),
    ResizeClip(ResizeClipJob),
    DeleteCameraClip(DeleteCameraClipJob),
    SyncSubtitles(SyncSubtitlesJob),
    UpdateCameraClip(UpdateCameraClipJob),
}

impl Job {
    pub fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let job_execute: &dyn JobExecute = match self {
            Job::MoveClip(job) => job,
            Job::AddCameraClip(job) => job,
            Job::ResizeClip(job) => job,
            Job::DeleteCameraClip(job) => job,
            Job::SyncSubtitles(job) => job,
            Job::UpdateCameraClip(job) => job,
        };
        job_execute.execute(sequence)
    }
}

pub trait JobExecute {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String>;
}

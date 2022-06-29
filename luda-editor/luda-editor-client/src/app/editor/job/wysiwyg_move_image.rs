use super::JobExecute;
use crate::app::{editor::clip_editor::camera_clip_editor::WysiwygTarget, types::*};

#[derive(Debug, Clone)]
pub struct WysiwygMoveImageJob {
    pub target: WysiwygTarget,
    pub clip_id: String,
    pub start_global_mouse_xy: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
    pub container_size: namui::Wh<f32>,
}

impl JobExecute for WysiwygMoveImageJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        match sequence.replace_clip(&self.clip_id, |clip: &CameraClip| {
            let mut clip = clip.clone();
            self.move_camera_angle(&mut clip.camera_angle);
            Ok(clip)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Clip not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl WysiwygMoveImageJob {
    pub fn move_camera_angle(&self, camera_angle: &mut CameraAngle) {
        let mouse_diff_xy = self.last_global_mouse_xy - self.start_global_mouse_xy;
        match self.target {
            WysiwygTarget::Character => {
                camera_angle.character.as_mut().map(|character| {
                    character.source_01_circumscribed.center.x +=
                        mouse_diff_xy.x / self.container_size.width;
                    character.source_01_circumscribed.center.y +=
                        mouse_diff_xy.y / self.container_size.height;
                });
            }
            WysiwygTarget::Background => {
                camera_angle.background.as_mut().map(|background| {
                    background.source_01_circumscribed.center.x +=
                        mouse_diff_xy.x / self.container_size.width;
                    background.source_01_circumscribed.center.y +=
                        mouse_diff_xy.y / self.container_size.height;
                });
            }
        }
    }
}

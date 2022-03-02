use super::JobExecute;
use crate::app::types::*;

#[derive(Debug, Clone)]
pub struct ChangeImageJob {
    pub clip_id: String,
    pub character_pose_emotion: Option<CharacterPoseEmotion>,
    pub background: Option<String>,
}

impl JobExecute for ChangeImageJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        match sequence.replace_clip(&self.clip_id, |clip: &CameraClip| {
            let mut clip = clip.clone();
            if clip.camera_angle.character_pose_emotion != self.character_pose_emotion {
                clip.camera_angle.character_pose_emotion = self.character_pose_emotion.clone();
            }
            if clip.camera_angle.background != self.background {
                clip.camera_angle.background = self.background.clone();
            }
            Ok(clip)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera clip not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

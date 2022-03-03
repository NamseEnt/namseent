use super::JobExecute;
use crate::app::types::*;

#[derive(Debug, Clone)]
pub struct ChangeImageJob {
    pub clip_id: String,
    pub character_pose_emotion: Option<CharacterPoseEmotion>,
    pub background_name: Option<String>,
}

impl JobExecute for ChangeImageJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        match sequence.replace_clip(&self.clip_id, |clip: &CameraClip| {
            let mut clip = clip.clone();
            match clip.camera_angle.character.as_mut() {
                Some(character) => match &self.character_pose_emotion {
                    Some(character_pose_emotion) => {
                        character.character_pose_emotion = character_pose_emotion.clone();
                    }
                    None => {
                        clip.camera_angle.character = None;
                    }
                },
                None => match &self.character_pose_emotion {
                    Some(character_pose_emotion) => {
                        clip.camera_angle.character =
                            Some(CameraAngleCharacter::default(character_pose_emotion));
                    }
                    None => {}
                },
            }
            match clip.camera_angle.background.as_mut() {
                Some(background) => match &self.background_name {
                    Some(background_name) => {
                        background.name = background_name.clone();
                    }
                    None => {
                        clip.camera_angle.background = None;
                    }
                },
                None => match &self.background_name {
                    Some(background_name) => {
                        clip.camera_angle.background =
                            Some(CameraAngleBackground::default(background_name));
                    }
                    None => {}
                },
            }
            Ok(clip)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera clip not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

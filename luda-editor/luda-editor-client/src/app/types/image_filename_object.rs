use super::CharacterPoseEmotion;

#[derive(Debug, Clone)]
pub struct ImageFilenameObject {
    pub character: String,
    pub pose: String,
    pub emotion: String,
    pub url: String,
}

impl ImageFilenameObject {
    pub fn into_character_pose_emotion(&self) -> CharacterPoseEmotion {
        CharacterPoseEmotion(
            self.character.clone(),
            self.pose.clone(),
            self.emotion.clone(),
        )
    }
}

use super::ImageFilenameObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterPoseEmotion(pub String, pub String, pub String);

impl CharacterPoseEmotion {
    pub fn get_url(&self, image_filename_objects: &Vec<ImageFilenameObject>) -> Option<String> {
        for image_filename_object in image_filename_objects {
            if image_filename_object.character == self.0
                && image_filename_object.pose == self.1
                && image_filename_object.emotion == self.2
            {
                return Some(image_filename_object.url.clone());
            }
        }
        None
    }
}

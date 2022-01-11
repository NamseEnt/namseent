use crate::app::types::{CharacterPoseEmotion, ImageFilenameObject};

impl ImageFilenameObject {
    pub fn into_character_pose_emotion(&self) -> CharacterPoseEmotion {
        CharacterPoseEmotion(
            self.character.clone(),
            self.pose.clone(),
            self.emotion.clone(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageBrowserItem {
    Back,
    Character(String),
    CharacterPose(String, String),
    CharacterPoseEmotion(String, String, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageBrowserDirectory {
    Root,
    Character(String),
    CharacterPose(String, String),
}
impl ImageBrowserDirectory {
    pub(crate) fn parent(&self) -> ImageBrowserDirectory {
        match self {
            ImageBrowserDirectory::Root => unreachable!(),
            ImageBrowserDirectory::Character(character) => ImageBrowserDirectory::Root,
            ImageBrowserDirectory::CharacterPose(character, pose) => {
                ImageBrowserDirectory::Character(character.clone())
            }
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            ImageBrowserDirectory::Root => vec![],
            ImageBrowserDirectory::Character(character) => vec![character.clone()],
            ImageBrowserDirectory::CharacterPose(character, pose) => {
                vec![character.clone(), pose.clone()]
            }
        }
        .join("/")
    }
}

impl ImageBrowserItem {
    pub(crate) fn to_string(&self) -> String {
        match self {
            ImageBrowserItem::Back => vec!["Back".to_string()],
            ImageBrowserItem::Character(character) => vec![character.clone()],
            ImageBrowserItem::CharacterPose(character, pose) => {
                vec![character.clone(), pose.clone()]
            }
            ImageBrowserItem::CharacterPoseEmotion(character, pose, emotion) => {
                vec![character.clone(), pose.clone(), emotion.clone()]
            }
        }
        .join("/")
    }
}

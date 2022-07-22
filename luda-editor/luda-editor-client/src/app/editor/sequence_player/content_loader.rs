use crate::app::types::*;
use std::{collections::LinkedList, sync::Arc, sync::Mutex};

pub(super) struct ContentLoader {
    sequence: Arc<Sequence>,
    loading_contents: Mutex<LinkedList<LoadingContent>>,
    camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
}

enum LoadingContent {
    CharacterImage { character: CameraAngleCharacter },
}

impl ContentLoader {
    pub(super) fn new(
        sequence: Arc<Sequence>,
        camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
    ) -> Self {
        let mut loader = Self {
            sequence,
            loading_contents: Mutex::new(LinkedList::new()),
            camera_angle_image_loader,
        };

        loader.start_loading();

        loader
    }
    fn start_loading(&mut self) {
        let mut loading_contents = self.loading_contents.lock().unwrap();
        for track in self.sequence.tracks.iter() {
            match track.as_ref() {
                Track::Camera(camera_track) => {
                    for clip in camera_track.clips.iter() {
                        match clip.camera_angle.character.as_ref() {
                            None => continue,
                            Some(character) => {
                                let path = character.character_pose_emotion.to_path();
                                let image = self
                                    .camera_angle_image_loader
                                    .try_load_character_image(&path);
                                if image.is_none() {
                                    loading_contents.push_back(LoadingContent::CharacterImage {
                                        character: character.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
                Track::Subtitle(_) => {
                    // NOTE: namui starts engine after loading fonts.
                }
            }
        }
    }
    pub fn is_loaded(&self) -> bool {
        let mut loading_contents = self.loading_contents.lock().unwrap();
        if loading_contents.is_empty() {
            return true;
        }

        while let Some(loading_content) = loading_contents.front() {
            match loading_content {
                LoadingContent::CharacterImage { character } => {
                    let path = character.character_pose_emotion.to_path();
                    if self
                        .camera_angle_image_loader
                        .try_load_character_image(&path)
                        .is_none()
                    {
                        return false;
                    }
                    loading_contents.pop_front();
                }
            }
        }

        true
    }
}

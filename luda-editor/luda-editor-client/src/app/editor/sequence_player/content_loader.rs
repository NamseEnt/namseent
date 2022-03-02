use crate::app::types::*;
use std::{collections::LinkedList, sync::Arc, sync::Mutex};

pub(super) struct ContentLoader {
    sequence: Arc<Sequence>,
    loading_contents: Mutex<LinkedList<LoadingContent>>,
}

enum LoadingContent {
    Image(String),
}

impl ContentLoader {
    pub(super) fn new(
        sequence: Arc<Sequence>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> Self {
        let mut loader = Self {
            sequence,
            loading_contents: Mutex::new(LinkedList::new()),
        };

        loader.start_loading(camera_angle_image_loader);

        loader
    }
    fn start_loading(&mut self, camera_angle_image_loader: &dyn CameraAngleImageLoader) {
        let managers = namui::managers();
        let mut loading_contents = self.loading_contents.lock().unwrap();
        for track in self.sequence.tracks.iter() {
            match track.as_ref() {
                Track::Camera(camera_track) => {
                    for clip in camera_track.clips.iter() {
                        let character_pose_emotion = match &clip.camera_angle.character_pose_emotion
                        {
                            Some(character_pose_emotion) => character_pose_emotion,
                            None => continue,
                        };

                        let image_source = camera_angle_image_loader
                            .get_character_image_source(&character_pose_emotion);

                        if image_source.is_none() {
                            panic!("image source is none");
                        }
                        match image_source.unwrap() {
                            namui::ImageSource::Url(url) => {
                                if managers.image_manager.clone().try_load(&url).is_some() {
                                    continue;
                                }
                                loading_contents.push_back(LoadingContent::Image(url));
                            }
                            namui::ImageSource::Image(_) => {
                                continue;
                            }
                        }
                    }
                }
                Track::Subtitle(_) => {
                    // NOTE: namui starts engine after loading fonts.
                }
                Track::Background(_) => todo!(),
            }
        }
    }
    pub fn is_loaded(&self) -> bool {
        let mut loading_contents = self.loading_contents.lock().unwrap();
        if loading_contents.is_empty() {
            return true;
        }

        let managers = namui::managers();
        while let Some(loading_content) = loading_contents.front() {
            match loading_content {
                LoadingContent::Image(url) => {
                    if managers.image_manager.clone().try_load(&url).is_none() {
                        return false;
                    }
                    loading_contents.pop_front();
                }
            }
        }

        true
    }
}

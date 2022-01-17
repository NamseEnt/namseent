use crate::app::types::*;
use std::{collections::LinkedList, rc::Rc};

pub(super) struct ContentLoader {
    sequence: Rc<Sequence>,
    loading_contents: LinkedList<LoadingContent>,
}

enum LoadingContent {
    Image(String),
}

impl ContentLoader {
    pub(super) fn new(
        sequence: Rc<Sequence>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> Self {
        let mut loader = Self {
            sequence,
            loading_contents: LinkedList::new(),
        };

        loader.start_loading(camera_angle_image_loader);

        loader
    }
    fn start_loading(&mut self, camera_angle_image_loader: &dyn CameraAngleImageLoader) {
        let managers = namui::managers();
        for track in &self.sequence.tracks {
            match track {
                Track::Camera(camera_track) => {
                    for clip in &camera_track.clips {
                        let image_source = camera_angle_image_loader
                            .get_image_source(&clip.camera_angle.character_pose_emotion);

                        if image_source.is_none() {
                            panic!("image source is none");
                        }
                        match image_source.unwrap() {
                            namui::ImageSource::Url(url) => {
                                if managers.image_manager.clone().try_load(&url).is_some() {
                                    continue;
                                }
                                self.loading_contents.push_back(LoadingContent::Image(url));
                            }
                            namui::ImageSource::Image(_) => {
                                continue;
                            }
                        }
                    }
                }
                Track::Subtitle(subtitle_track) => {
                    // NOTE: namui starts engine after loading fonts.
                }
            }
        }
    }
    pub fn is_loaded(&mut self) -> bool {
        if self.loading_contents.is_empty() {
            return true;
        }

        let managers = namui::managers();
        while let Some(loading_content) = self.loading_contents.front() {
            match loading_content {
                LoadingContent::Image(url) => {
                    if managers.image_manager.clone().try_load(&url).is_none() {
                        return false;
                    }
                    self.loading_contents.pop_front();
                }
            }
        }

        true
    }
}

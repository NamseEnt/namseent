use super::*;
use crate::app::storage::GithubStorage;
use dashmap::{DashMap, DashSet};
use namui::prelude::*;
use std::fmt::Debug;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAngle {
    pub character: Option<CameraAngleCharacter>,
    pub background: Option<CameraAngleBackground>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAngleCharacter {
    pub character_pose_emotion: CharacterPoseEmotion,
    pub source_01_circumscribed: Circumscribed,
    pub crop_screen_01_rect: Rect<f32>,
}
impl CameraAngleCharacter {
    pub fn default(character_pose_emotion: &CharacterPoseEmotion) -> Self {
        Self {
            character_pose_emotion: character_pose_emotion.clone(),
            source_01_circumscribed: Circumscribed {
                center: Xy { x: 0.5, y: 0.5 },
                radius: 0.5,
            },
            crop_screen_01_rect: Rect::Ltrb {
                left: 0.0,
                top: 0.0,
                right: 1.0,
                bottom: 1.0,
            },
        }
    }
    pub fn render(
        &self,
        wh: Wh<Px>,
        camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
    ) -> RenderingTree {
        let path = self.character_pose_emotion.to_path();
        let image = camera_angle_image_loader.try_load_character_image(&path);
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let clip_rect = Rect::Ltrb {
            left: num::clamp(self.crop_screen_01_rect.left(), 0.0, 1.0) * wh.width,
            top: num::clamp(self.crop_screen_01_rect.top(), 0.0, 1.0) * wh.height,
            right: num::clamp(self.crop_screen_01_rect.right(), 0.0, 1.0) * wh.width,
            bottom: num::clamp(self.crop_screen_01_rect.bottom(), 0.0, 1.0) * wh.height,
        };

        let image_length = self.source_01_circumscribed.radius * 2.0 * wh.length();
        let image_source_size = image.size();
        let image_source_length = image_source_size.length();
        let image_size = Wh {
            width: image_source_size.width * (image_length / image_source_length),
            height: image_source_size.height * (image_length / image_source_length),
        };
        let image_center = Xy {
            x: self.source_01_circumscribed.center.x * wh.width,
            y: self.source_01_circumscribed.center.y * wh.height,
        };
        let image_rect = Rect::Xywh {
            x: image_center.x - image_size.width / 2.0,
            y: image_center.y - image_size.height / 2.0,
            width: image_size.width,
            height: image_size.height,
        };

        clip(
            PathBuilder::new().add_rect(clip_rect),
            ClipOp::Intersect,
            namui::image(ImageParam {
                source: ImageSource::Image(image),
                rect: image_rect,
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
            }),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAngleBackground {
    pub name: String,
    pub source_01_circumscribed: Circumscribed,
}
impl CameraAngleBackground {
    pub(crate) fn default(name: &str) -> CameraAngleBackground {
        Self {
            name: name.to_string(),
            source_01_circumscribed: Circumscribed {
                center: Xy { x: 0.5, y: 0.5 },
                radius: 0.5,
            },
        }
    }
    pub fn render(
        &self,
        wh: Wh<Px>,
        camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
    ) -> RenderingTree {
        let path = format!("{}.png", self.name);
        let image = camera_angle_image_loader.try_load_background_image(&path);
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let clip_rect = Rect::Ltrb {
            left: px(0.0),
            top: px(0.0),
            right: wh.width,
            bottom: wh.height,
        };

        let image_source_size = image.size();
        let image_source_length = image_source_size.length();
        let circumscribed_to_wh_ratio =
            wh.length() / (self.source_01_circumscribed.radius * 2.0 * image_source_length);
        let zoomed_size = Wh {
            width: image_source_size.width * circumscribed_to_wh_ratio,
            height: image_source_size.height * circumscribed_to_wh_ratio,
        };

        let image_rect = Rect::Xywh {
            x: -self.source_01_circumscribed.center.x * zoomed_size.width + wh.width / 2.0,
            y: -self.source_01_circumscribed.center.y * zoomed_size.height + wh.height / 2.0,
            width: zoomed_size.width,
            height: zoomed_size.height,
        };

        clip(
            PathBuilder::new().add_rect(clip_rect),
            ClipOp::Intersect,
            namui::image(ImageParam {
                source: ImageSource::Image(image),
                rect: image_rect,
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
            }),
        )
    }
    pub fn to_path(&self) -> String {
        format!("{}.jpg", self.name)
    }
}

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
pub trait CameraAngleImageLoader: Debug + Send + Sync {
    fn try_load_character_image(&self, character: &String) -> Option<Arc<Image>>;
    fn try_load_background_image(&self, background: &String) -> Option<Arc<Image>>;
}

#[derive(Debug)]
pub struct LudaEditorCameraAngleImageLoader {
    storage: Arc<dyn GithubStorage>,
    loading_character_images: Arc<DashSet<String>>,
    loading_background_images: Arc<DashSet<String>>,
    character_images: Arc<DashMap<String, Arc<Image>>>,
    background_images: Arc<DashMap<String, Arc<Image>>>,
}
impl LudaEditorCameraAngleImageLoader {
    pub fn new(storage: Arc<dyn GithubStorage>) -> Self {
        Self {
            storage,
            loading_character_images: Arc::new(DashSet::new()),
            loading_background_images: Arc::new(DashSet::new()),
            character_images: Arc::new(DashMap::new()),
            background_images: Arc::new(DashMap::new()),
        }
    }
}
impl CameraAngleImageLoader for LudaEditorCameraAngleImageLoader {
    fn try_load_character_image(&self, character_path: &String) -> Option<Arc<Image>> {
        if let Some(image) = self.character_images.get(character_path) {
            return Some(image.clone());
        }
        if self.loading_character_images.contains(character_path) {
            return None;
        }
        self.loading_character_images.insert(character_path.clone());

        let character_path = character_path.clone();
        let loading_character_images = self.loading_character_images.clone();
        let character_images = self.character_images.clone();
        let storage = self.storage.clone();
        spawn_local(async move {
            match storage.get_character_image(character_path.as_str()).await {
                Ok(image) => {
                    character_images.insert(character_path.clone(), image);
                    loading_character_images.remove(&character_path);
                }
                Err(error) => {
                    namui::log!("fail to load character image: {:#?}", error);
                }
            }
        });

        None
    }

    fn try_load_background_image(&self, background_path: &String) -> Option<Arc<Image>> {
        if let Some(image) = self.background_images.get(background_path) {
            return Some(image.clone());
        }
        if self.loading_background_images.contains(background_path) {
            return None;
        }
        self.loading_background_images
            .insert(background_path.clone());

        let background_path = background_path.clone();
        let loading_background_images = self.loading_background_images.clone();
        let background_images = self.background_images.clone();
        let storage = self.storage.clone();
        spawn_local(async move {
            match storage.get_character_image(background_path.as_str()).await {
                Ok(image) => {
                    background_images.insert(background_path.clone(), image);
                    loading_background_images.remove(&background_path);
                }
                Err(error) => {
                    namui::log!("fail to load background image: {:#?}", error);
                }
            }
        });
        None
    }
}

impl CameraAngle {
    pub fn render(
        &self,
        wh: Wh<Px>,
        camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
    ) -> RenderingTree {
        render([
            self.background
                .as_ref()
                .map_or(RenderingTree::Empty, |background| {
                    background.render(wh, camera_angle_image_loader.clone())
                }),
            self.character
                .as_ref()
                .map_or(RenderingTree::Empty, |character| {
                    character.render(wh, camera_angle_image_loader)
                }),
        ])
    }
}

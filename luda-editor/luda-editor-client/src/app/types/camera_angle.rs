use super::*;
use namui::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAngle {
    pub character: Option<CameraAngleCharacter>,
    pub background: Option<CameraAngleBackground>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAngleCharacter {
    pub character_pose_emotion: CharacterPoseEmotion,
    pub source_01_circumscribed: Circumscribed,
    pub crop_screen_01_rect: LtrbRect,
}
impl CameraAngleCharacter {
    pub fn default(character_pose_emotion: &CharacterPoseEmotion) -> Self {
        Self {
            character_pose_emotion: character_pose_emotion.clone(),
            source_01_circumscribed: Circumscribed {
                center: Xy { x: 0.5, y: 0.5 },
                radius: 0.5,
            },
            crop_screen_01_rect: LtrbRect {
                left: 0.0,
                top: 0.0,
                right: 1.0,
                bottom: 1.0,
            },
        }
    }
    pub fn render(
        &self,
        wh: &Wh<f32>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> RenderingTree {
        let image_source = camera_angle_image_loader.get_character_image_source(self);
        let image = match image_source {
            ImageSource::Url(url) => namui::system::image::try_load(&url),
            ImageSource::Image(image) => Some(image),
        };
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let clip_rect = LtrbRect {
            left: num::clamp(self.crop_screen_01_rect.left, 0.0, 1.0) * wh.width,
            top: num::clamp(self.crop_screen_01_rect.top, 0.0, 1.0) * wh.height,
            right: num::clamp(self.crop_screen_01_rect.right, 0.0, 1.0) * wh.width,
            bottom: num::clamp(self.crop_screen_01_rect.bottom, 0.0, 1.0) * wh.height,
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
        let image_xywh = XywhRect {
            x: image_center.x - image_size.width / 2.0,
            y: image_center.y - image_size.height / 2.0,
            width: image_size.width,
            height: image_size.height,
        };

        clip(
            PathBuilder::new().add_rect(&clip_rect),
            ClipOp::Intersect,
            namui::image(ImageParam {
                source: ImageSource::Image(image),
                xywh: image_xywh,
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
        wh: &Wh<f32>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> RenderingTree {
        let image_source = camera_angle_image_loader.get_background_image_source(self);
        let image = match image_source {
            ImageSource::Url(url) => namui::system::image::try_load(&url),
            ImageSource::Image(image) => Some(image),
        };
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        let clip_rect = LtrbRect {
            left: 0.0,
            top: 0.0,
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

        let image_xywh = XywhRect {
            x: -self.source_01_circumscribed.center.x * zoomed_size.width + wh.width / 2.0,
            y: -self.source_01_circumscribed.center.y * zoomed_size.height + wh.height / 2.0,
            width: zoomed_size.width,
            height: zoomed_size.height,
        };

        clip(
            PathBuilder::new().add_rect(&clip_rect),
            ClipOp::Intersect,
            namui::image(ImageParam {
                source: ImageSource::Image(image),
                xywh: image_xywh,
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
            }),
        )
    }
}

pub trait CameraAngleImageLoader {
    fn get_character_image_source(&self, character: &CameraAngleCharacter) -> ImageSource;
    fn get_background_image_source(&self, background: &CameraAngleBackground) -> ImageSource;
}

pub struct LudaEditorServerCameraAngleImageLoader;
impl CameraAngleImageLoader for LudaEditorServerCameraAngleImageLoader {
    fn get_character_image_source(&self, character: &CameraAngleCharacter) -> ImageSource {
        let url = namui::Url::parse(&format!(
            "http://localhost:3030/resources/characterImages{}",
            character.character_pose_emotion.to_url()
        ))
        .unwrap();
        ImageSource::Url(url)
    }

    fn get_background_image_source(&self, background: &CameraAngleBackground) -> ImageSource {
        let url = namui::Url::parse(&format!(
            "http://localhost:3030/resources/backgrounds/{}.jpeg",
            background.name
        ))
        .unwrap();
        ImageSource::Url(url)
    }
}

impl CameraAngle {
    pub fn render(
        &self,
        wh: &Wh<f32>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> RenderingTree {
        render![
            self.background
                .as_ref()
                .map_or(RenderingTree::Empty, |background| {
                    background.render(wh, camera_angle_image_loader)
                }),
            self.character
                .as_ref()
                .map_or(RenderingTree::Empty, |character| {
                    character.render(wh, camera_angle_image_loader)
                }),
        ]
    }
}

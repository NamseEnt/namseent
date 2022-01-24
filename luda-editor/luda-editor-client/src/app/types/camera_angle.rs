use super::*;
use namui::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraAngle {
    pub character_pose_emotion: CharacterPoseEmotion,
    pub source_01_circumscribed: Circumscribed,
    pub crop_screen_01_rect: LtrbRect,
}

pub trait CameraAngleImageLoader {
    fn get_image_source(
        &self,
        character_pose_emotion: &CharacterPoseEmotion,
    ) -> Option<ImageSource>;
}

pub struct LudaEditorServerCameraAngleImageLoader;
impl CameraAngleImageLoader for LudaEditorServerCameraAngleImageLoader {
    fn get_image_source(
        &self,
        character_pose_emotion: &CharacterPoseEmotion,
    ) -> Option<ImageSource> {
        let url = format!(
            "http://localhost:3030/resources/images/{}-{}-{}.png",
            character_pose_emotion.0, character_pose_emotion.1, character_pose_emotion.2
        );
        Some(ImageSource::Url(url))
    }
}

impl CameraAngle {
    pub fn render(
        &self,
        wh: &Wh<f32>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> RenderingTree {
        let image_source = camera_angle_image_loader.get_image_source(&self.character_pose_emotion);
        if image_source.is_none() {
            return RenderingTree::Empty;
        }
        let image = match image_source.unwrap() {
            ImageSource::Url(url) => namui::managers().image_manager.clone().try_load(&url),
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

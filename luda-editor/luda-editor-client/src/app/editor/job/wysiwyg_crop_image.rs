use super::JobExecute;
use crate::app::{
    editor::clip_editor::camera_clip_editor::wysiwyg_editor::cropper::{
        CropperHandle, CropperHandleDirection,
    },
    types::*,
};
use namui::prelude::*;

#[derive(Debug, Clone)]
pub struct WysiwygCropImageJob {
    pub clip_id: String,
    pub start_global_mouse_xy: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
    pub handle: CropperHandle,
    pub container_size: namui::Wh<f32>,
}

impl JobExecute for WysiwygCropImageJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        match sequence.replace_clip(&self.clip_id, |clip: &CameraClip| {
            let mut clip = clip.clone();
            self.crop_camera_angle(&mut clip.camera_angle);
            Ok(clip)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Clip not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl WysiwygCropImageJob {
    pub fn crop_camera_angle(&self, camera_angle: &mut CameraAngle) {
        let mouse_diff_xy = self.last_global_mouse_xy - self.start_global_mouse_xy;

        let next_ltrb_rect = LtrbRect {
            left: match self.handle.handle_direction {
                CropperHandleDirection::TopLeft
                | CropperHandleDirection::BottomLeft
                | CropperHandleDirection::Left => num::clamp(
                    camera_angle.crop_screen_01_rect.left
                        + mouse_diff_xy.x / self.container_size.width,
                    0.0,
                    camera_angle.crop_screen_01_rect.right,
                ),
                _ => camera_angle.crop_screen_01_rect.left,
            },
            top: match self.handle.handle_direction {
                CropperHandleDirection::TopLeft
                | CropperHandleDirection::TopRight
                | CropperHandleDirection::Top => num::clamp(
                    camera_angle.crop_screen_01_rect.top
                        + mouse_diff_xy.y / self.container_size.height,
                    0.0,
                    camera_angle.crop_screen_01_rect.bottom,
                ),
                _ => camera_angle.crop_screen_01_rect.top,
            },
            right: match self.handle.handle_direction {
                CropperHandleDirection::TopRight
                | CropperHandleDirection::BottomRight
                | CropperHandleDirection::Right => num::clamp(
                    camera_angle.crop_screen_01_rect.right
                        + mouse_diff_xy.x / self.container_size.width,
                    camera_angle.crop_screen_01_rect.left,
                    1.0,
                ),
                _ => camera_angle.crop_screen_01_rect.right,
            },
            bottom: match self.handle.handle_direction {
                CropperHandleDirection::BottomLeft
                | CropperHandleDirection::BottomRight
                | CropperHandleDirection::Bottom => num::clamp(
                    camera_angle.crop_screen_01_rect.bottom
                        + mouse_diff_xy.y / self.container_size.height,
                    camera_angle.crop_screen_01_rect.top,
                    1.0,
                ),
                _ => camera_angle.crop_screen_01_rect.bottom,
            },
        };

        camera_angle.crop_screen_01_rect = next_ltrb_rect;
    }
}

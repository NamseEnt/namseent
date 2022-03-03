use super::JobExecute;
use crate::app::{
    editor::clip_editor::camera_clip_editor::{
        wysiwyg_editor::resizer::{ResizerHandle, ResizerHandleDirection},
        WysiwygTarget,
    },
    types::*,
};
use namui::prelude::*;

#[derive(Debug, Clone)]
pub struct WysiwygResizeImageJob {
    pub target: WysiwygTarget,
    pub clip_id: String,
    pub start_global_mouse_xy: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
    pub handle: ResizerHandle,
    pub center_xy: Xy<f32>,
    pub container_size: namui::Wh<f32>,
    pub image_size_ratio: namui::Wh<f32>,
}

impl JobExecute for WysiwygResizeImageJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        match sequence.replace_clip(&self.clip_id, |clip: &CameraClip| {
            let mut clip = clip.clone();
            self.resize_camera_angle(&mut clip.camera_angle);
            Ok(clip)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Clip not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl WysiwygResizeImageJob {
    pub fn resize_camera_angle(&self, camera_angle: &mut CameraAngle) {
        let mouse_diff_xy = self.last_global_mouse_xy - self.start_global_mouse_xy;

        let source_01_circumscribed = resize_by_center(
            &self.handle,
            &self.center_xy,
            &mouse_diff_xy,
            &self.container_size,
            &self.image_size_ratio,
        );
        match self.target {
            WysiwygTarget::Character => {
                let character = camera_angle.character.as_mut();
                if let Some(character) = character {
                    character.source_01_circumscribed = source_01_circumscribed;
                }
            }
            WysiwygTarget::Background => {
                let background = camera_angle.background.as_mut();
                if let Some(background) = background {
                    background.source_01_circumscribed = source_01_circumscribed;
                }
            }
        };
    }
}

fn get_y_in_vector(xy1: &Xy<f32>, xy2: &Xy<f32>, x: f32) -> Option<f32> {
    if xy1.x == xy2.x {
        None
    } else if xy1.y == xy2.y {
        Some(xy1.y)
    } else {
        let Xy { x: x1, y: y1 } = xy1;
        let Xy { x: x2, y: y2 } = xy2;
        let a = (y2 - y1) / (x2 - x1);
        let b = y1 - a * x1;
        Some(a * x + b)
    }
}
fn get_x_in_vector(xy1: &Xy<f32>, xy2: &Xy<f32>, y: f32) -> Option<f32> {
    if xy1.x == xy2.x {
        Some(xy1.x)
    } else if xy1.y == xy2.y {
        None
    } else {
        let Xy { x: x1, y: y1 } = xy1;
        let Xy { x: x2, y: y2 } = xy2;
        let a = (y2 - y1) / (x2 - x1);
        let b = y1 - a * x1;
        Some((y - b) / a)
    }
}
// NOTE : I make resizing by center but not sure it is the best way to resize.
// You can test resizing by anchor, not center.
fn resize_by_center(
    handle: &ResizerHandle,
    center_xy: &Xy<f32>,
    diff_xy: &Xy<f32>,
    container_size: &Wh<f32>,
    image_size_ratio: &Wh<f32>,
) -> Circumscribed {
    let designated_xy = handle.xy + diff_xy;

    let projected_xy_by_x = get_y_in_vector(center_xy, &handle.xy, designated_xy.x).map(|y| Xy {
        x: designated_xy.x,
        y,
    });
    let projected_xy_by_y = get_x_in_vector(center_xy, &handle.xy, designated_xy.y).map(|x| Xy {
        x,
        y: designated_xy.y,
    });

    let candidates = [projected_xy_by_x, projected_xy_by_y];

    let projected_length = candidates
        .iter()
        .filter_map(|candidate| candidate.map(|xy| (xy - center_xy).length()))
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();

    let radius = match handle.handle_direction {
        ResizerHandleDirection::TopLeft
        | ResizerHandleDirection::TopRight
        | ResizerHandleDirection::BottomRight
        | ResizerHandleDirection::BottomLeft => projected_length,
        ResizerHandleDirection::Top | ResizerHandleDirection::Bottom => {
            projected_length / image_size_ratio.height * image_size_ratio.length()
        }
        ResizerHandleDirection::Right | ResizerHandleDirection::Left => {
            projected_length / image_size_ratio.width * image_size_ratio.length()
        }
    };

    Circumscribed {
        center: Xy {
            x: center_xy.x / container_size.width,
            y: center_xy.y / container_size.height,
        },
        radius: radius / container_size.length(),
    }
}

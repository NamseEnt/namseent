use crate::editor::{
    types::{CameraAngle, CameraClip, Clip},
    Timeline,
};

#[derive(Debug, Clone)]
pub struct WysiwygMoveImageJob {
    pub start_global_mouse_xy: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
    pub container_size: namui::Wh<f32>,
}

impl WysiwygMoveImageJob {
    pub fn execute(&self, timeline: &mut Timeline) {
        let selected_clip = timeline
            .selected_clip_id
            .as_ref()
            .and_then(|id| timeline.sequence.get_clip(&id));

        let selected_camera_clip = match selected_clip {
            Some(clip) => match clip {
                Clip::Camera(camera_clip) => Ok(camera_clip),
                Clip::Subtitle(_) => Err("Camera clip expected, but Subtitle clip selected"),
            },
            None => Err("No clip selected"),
        };
        if selected_camera_clip.is_err() {
            return;
        }
        let mut selected_camera_clip = selected_camera_clip.unwrap().clone();
        let camera_angle = &mut selected_camera_clip.camera_angle;
        self.move_camera_angle(camera_angle);

        timeline
            .sequence
            .update_camera_clip(&selected_camera_clip.id.clone(), selected_camera_clip);
    }

    pub fn move_camera_angle(&self, camera_angle: &mut CameraAngle) {
        let mouse_diff_xy = self.last_global_mouse_xy - self.start_global_mouse_xy;

        camera_angle.source_point_rect_length_ratio.x +=
            mouse_diff_xy.x / self.container_size.width;
        camera_angle.source_point_rect_length_ratio.y +=
            mouse_diff_xy.y / self.container_size.height;
    }
}

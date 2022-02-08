use super::{get_camera_track_id, JobExecute};
use crate::app::types::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeDirection {
    Left,
    Right,
}
#[derive(Debug, Clone)]
pub struct ResizeCameraClipJob {
    pub clip_id: String,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
    pub resize_direction: ResizeDirection,
    pub is_moved: bool,
}

impl JobExecute for ResizeCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let camera_track_id = get_camera_track_id(&sequence);
        match sequence.replace_track(&camera_track_id, |track: &CameraTrack| {
            let mut track = track.clone();
            self.resize_clip_in_track(&mut track);
            Ok(track)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl ResizeCameraClipJob {
    pub fn get_delta_time(&self) -> Time {
        (self.last_mouse_position_in_time - self.click_anchor_in_time)
            * match self.resize_direction {
                ResizeDirection::Left => -1.0,
                ResizeDirection::Right => 1.0,
            }
    }
    pub fn resize_clip_in_track(&self, track: &mut CameraTrack) {
        track.resize_clip_delta(&self.clip_id, self.get_delta_time());
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn increase_by_left_direction_should_work() {
        let sequence = mock_sequence(&["0", "1", "2"], &[]);
        let job = ResizeCameraClipJob {
            clip_id: "1".to_string(),
            click_anchor_in_time: Time::from_ms(0.0),
            last_mouse_position_in_time: Time::from_ms(-0.5),
            is_moved: true,
            resize_direction: ResizeDirection::Left,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);

        let clips = extract_camera_clips(&result);
        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[0].end_at, Time::from_ms(1.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0));
        assert_eq!(clips[1].end_at, Time::from_ms(2.5));
        assert_eq!(clips[2].start_at, Time::from_ms(2.5));
        assert_eq!(clips[2].end_at, Time::from_ms(3.5));
    }
    #[test]
    #[wasm_bindgen_test]
    fn decrease_by_left_direction_should_work() {
        let sequence = mock_sequence(&["0", "1", "2"], &[]);
        let job = ResizeCameraClipJob {
            clip_id: "1".to_string(),
            click_anchor_in_time: Time::from_ms(0.0),
            last_mouse_position_in_time: Time::from_ms(0.5),
            is_moved: true,
            resize_direction: ResizeDirection::Left,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);

        let clips = extract_camera_clips(&result);
        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[0].end_at, Time::from_ms(1.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0));
        assert_eq!(clips[1].end_at, Time::from_ms(1.5));
        assert_eq!(clips[2].start_at, Time::from_ms(1.5));
        assert_eq!(clips[2].end_at, Time::from_ms(2.5));
    }
    #[test]
    #[wasm_bindgen_test]
    fn increase_by_right_direction_should_work() {
        let sequence = mock_sequence(&["0", "1", "2"], &[]);
        let job = ResizeCameraClipJob {
            clip_id: "1".to_string(),
            click_anchor_in_time: Time::from_ms(0.0),
            last_mouse_position_in_time: Time::from_ms(0.5),
            is_moved: true,
            resize_direction: ResizeDirection::Right,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);

        let clips = extract_camera_clips(&result);
        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[0].end_at, Time::from_ms(1.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0));
        assert_eq!(clips[1].end_at, Time::from_ms(2.5));
        assert_eq!(clips[2].start_at, Time::from_ms(2.5));
        assert_eq!(clips[2].end_at, Time::from_ms(3.5));
    }
    #[test]
    #[wasm_bindgen_test]
    fn decrease_by_right_direction_should_work() {
        let sequence = mock_sequence(&["0", "1", "2"], &[]);
        let job = ResizeCameraClipJob {
            clip_id: "1".to_string(),
            click_anchor_in_time: Time::from_ms(0.0),
            last_mouse_position_in_time: Time::from_ms(-0.5),
            is_moved: true,
            resize_direction: ResizeDirection::Right,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);

        let clips = extract_camera_clips(&result);
        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[0].end_at, Time::from_ms(1.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0));
        assert_eq!(clips[1].end_at, Time::from_ms(1.5));
        assert_eq!(clips[2].start_at, Time::from_ms(1.5));
        assert_eq!(clips[2].end_at, Time::from_ms(2.5));
    }
}

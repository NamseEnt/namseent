use super::JobExecute;
use crate::app::types::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeDirection {
    Left,
    Right,
}
#[derive(Debug, Clone)]
pub struct ResizeClipJob {
    pub clip_id: String,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
    pub resize_direction: ResizeDirection,
    pub is_moved: bool,
}

impl JobExecute for ResizeClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let track_id = match sequence.find_track_by_clip_id(&self.clip_id) {
            Some(track) => track.get_id().to_string(),
            None => return Err(format!("cannot find track of clip_id {}", self.clip_id)),
        };
        match sequence.replace_track(&track_id, |mut track: ResizableTrack| {
            Ok(self.resize_clip_in_track(track))
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl ResizeClipJob {
    pub fn get_delta_time(&self) -> Time {
        (self.last_mouse_position_in_time - self.click_anchor_in_time)
            * match self.resize_direction {
                ResizeDirection::Left => -1.0,
                ResizeDirection::Right => 1.0,
            }
    }
    pub fn resize_clip_in_track<TTrack>(&self, track: TTrack) -> TTrack
    where
        TTrack: From<ResizableTrack> + Into<ResizableTrack>,
    {
        let mut resizable_track: ResizableTrack = track.into();
        resizable_track.resize_clip_delta(&self.clip_id, self.get_delta_time());
        resizable_track.into()
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
        let job = ResizeClipJob {
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
        let job = ResizeClipJob {
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
        let job = ResizeClipJob {
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
        let job = ResizeClipJob {
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

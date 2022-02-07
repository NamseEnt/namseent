use super::JobExecute;
use crate::app::types::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct MoveCameraClipJob {
    pub clip_ids: BTreeSet<String>,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
    pub is_moved: bool,
}

impl JobExecute for MoveCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let camera_track_id = get_camera_track_id(&sequence);
        match sequence.replace_track(&camera_track_id, |track: &CameraTrack| {
            let mut track = track.clone();
            track.move_clips_delta(
                &self.clip_ids.iter().collect::<Vec<_>>(),
                self.get_delta_time(),
            );
            Ok(track)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl MoveCameraClipJob {
    pub fn get_delta_time(&self) -> Time {
        self.last_mouse_position_in_time - self.click_anchor_in_time
    }
}

pub(crate) fn get_camera_track_id(sequence: &Sequence) -> String {
    sequence
        .tracks
        .iter()
        .find_map(|track| match track.as_ref() {
            Track::Camera(track) => Some(track.id.clone()),
            _ => None,
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use namui::prelude::*;
    use std::sync::Arc;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn move_camera_clip_backward_should_works() {
        // before : 0 1 2 3 4
        // job : move 1 between 3 and 4
        // result : 0 2 3 1 4

        let sequence = mock_sequence(&["0", "1", "2", "3", "4"], &[]);
        let job = MoveCameraClipJob {
            clip_ids: vec!["1".to_string()].into_iter().collect(),
            click_anchor_in_time: Time::from_ms(1.0),
            last_mouse_position_in_time: Time::from_ms(3.5),
            is_moved: true,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "2", "3", "1", "4"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn move_camera_clip_forward_should_works() {
        // before : 0 1 2 3 4
        // job : move 3 between 0 and 1
        // result : 0 3 1 2 4

        let sequence = mock_sequence(&["0", "1", "2", "3", "4"], &[]);
        let job = MoveCameraClipJob {
            clip_ids: vec!["3".to_string()].into_iter().collect(),
            click_anchor_in_time: Time::from_ms(3.0),
            last_mouse_position_in_time: Time::from_ms(0.5),
            is_moved: true,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "3", "1", "2", "4"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn move_camera_multiple_clips_backward_should_works() {
        // before : 0 1 2 3 4 5 6
        // job : move 1, 3, 4 by + 2.25
        // result : 0 2 1 5 3 4 6

        let sequence = mock_sequence(&["0", "1", "2", "3", "4", "5", "6"], &[]);
        let job = MoveCameraClipJob {
            clip_ids: vec!["1".to_string(), "3".to_string(), "4".to_string()]
                .into_iter()
                .collect(),
            click_anchor_in_time: Time::from_ms(0.0),
            last_mouse_position_in_time: Time::from_ms(2.25),
            is_moved: true,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "2", "1", "5", "3", "4", "6"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn move_camera_multiple_clips_forward_should_works() {
        // before : 0 1 2 3 4 5 6
        // job : move 1, 2, 5 by - 1.75
        // result : 1 2 0 3 5 4 6

        let sequence = mock_sequence(&["0", "1", "2", "3", "4", "5", "6"], &[]);
        let job = MoveCameraClipJob {
            clip_ids: vec!["1".to_string(), "2".to_string(), "5".to_string()]
                .into_iter()
                .collect(),
            click_anchor_in_time: Time::from_ms(0.0),
            last_mouse_position_in_time: Time::from_ms(-1.75),
            is_moved: true,
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["1", "2", "0", "3", "5", "4", "6"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }
}

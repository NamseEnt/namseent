use super::JobExecute;
use crate::app::types::*;
use namui::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AddCameraClipJob {
    pub camera_clip: Arc<CameraClip>,
    pub time_to_insert: Time,
}

impl JobExecute for AddCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let camera_track_id = get_camera_track_id(&sequence);
        match sequence.replace_track(&camera_track_id, |mut track: CameraTrack| {
            track.insert_clip(self.camera_clip.clone(), self.time_to_insert);
            Ok(track)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
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
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_for_empty_track_should_works() {
        let sequence = mock_sequence(&[], &[]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("0", Time::Ms(0.0), Time::Ms(1.0)),
            time_to_insert: Time::Ms(1.0),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_between_clips_should_works() {
        // before : 0 1 2 3
        // job : insert 4 between 1 and 2
        // result : 0 1 4 2 3

        let sequence = mock_sequence(&["0", "1", "2", "3"], &[]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("4", Time::Ms(0.0), Time::Ms(1.0)),
            time_to_insert: Time::Ms(1.75),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "4", "2", "3"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_front_should_works() {
        // before : 0 1 2 3
        // job : insert 4 at front
        // result : 4 0 1 2 3

        let sequence = mock_sequence(&["0", "1", "2", "3"], &[]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("4", Time::Ms(0.0), Time::Ms(1.0)),
            time_to_insert: Time::Ms(0.0),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["4", "0", "1", "2", "3"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_back_should_works() {
        // before : 0 1 2 3
        // job : insert 4 at back
        // result : 0 1 2 3 4

        let sequence = mock_sequence(&["0", "1", "2", "3"], &[]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("4", Time::Ms(0.0), Time::Ms(1.0)),
            time_to_insert: Time::Ms(4.0),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2", "3", "4"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }
}

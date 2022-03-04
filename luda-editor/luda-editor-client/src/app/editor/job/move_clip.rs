use super::JobExecute;
use crate::app::types::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct MoveClipJob {
    pub clip_ids: BTreeSet<String>,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
    pub is_moved: bool,
}

impl JobExecute for MoveClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        if self.clip_ids.is_empty() {
            return Err("Nothing to move".to_string());
        }
        let sequence = sequence.clone();
        let first_clip_id = self.clip_ids.iter().next().unwrap();
        let track_id = match sequence.find_track_by_clip_id(first_clip_id) {
            Some(track) => track.get_id().to_string(),
            None => return Err(format!("cannot find track of clip id {}", first_clip_id)),
        };
        match sequence.replace_track(&track_id, |mut track: Track| {
            Ok(self.move_clip_in_track(track))
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

impl MoveClipJob {
    pub fn get_delta_time(&self) -> Time {
        self.last_mouse_position_in_time - self.click_anchor_in_time
    }
    pub fn move_clip_in_track<TTrack>(&self, track: TTrack) -> TTrack
    where
        TTrack: From<Track> + Into<Track>,
    {
        let mut track: Track = track.into();
        track.move_clips_delta(
            &self.clip_ids.iter().collect::<Vec<_>>(),
            self.get_delta_time(),
        );
        track.into()
    }
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
        let job = MoveClipJob {
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
        let job = MoveClipJob {
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
        let job = MoveClipJob {
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
        let job = MoveClipJob {
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

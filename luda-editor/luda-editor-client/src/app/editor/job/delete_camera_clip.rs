use super::{get_camera_track_id, JobExecute};
use crate::app::types::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct DeleteCameraClipJob {
    pub clip_ids: BTreeSet<String>,
}

impl JobExecute for DeleteCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let camera_track_id = get_camera_track_id(&sequence);
        match sequence.replace_track(&camera_track_id, |mut track: CameraTrack| {
            track.delete_clips(&self.clip_ids.iter().collect::<Vec<_>>());
            Ok(track)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use namui::prelude::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn delete_clip_should_push_clips_front() {
        let sequence = mock_sequence(&[], &["0", "1", "2", "3", "4"], &[]);
        let job = DeleteCameraClipJob {
            clip_ids: vec!["1".to_string(), "3".to_string()].into_iter().collect(),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "2", "4"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);

        let clips = extract_camera_clips(&result);
        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[0].end_at, Time::from_ms(1.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0));
        assert_eq!(clips[1].end_at, Time::from_ms(2.0));
        assert_eq!(clips[2].start_at, Time::from_ms(2.0));
        assert_eq!(clips[2].end_at, Time::from_ms(3.0));
    }
}

use super::{get_camera_track_id, JobExecute};
use crate::app::types::*;
use std::sync::Arc;

pub struct UpdateCameraClipJob {
    pub clip_id: String,
    pub update: Arc<dyn Fn(&CameraClip) -> CameraClip + Send + Sync>,
}

impl JobExecute for UpdateCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let camera_track_id = get_camera_track_id(&sequence);
        match sequence.replace_track(&camera_track_id, |track: CameraTrack| {
            match track.replace_clip(&self.clip_id, |clip| Ok((self.update)(clip))) {
                UpdateResult::Updated(track) => Ok(track),
                UpdateResult::NotUpdated => Err("Camera clip not found".to_string()),
                UpdateResult::Err(err) => Err(err),
            }
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
    fn throw_error_if_clip_not_exists() {
        let sequence = mock_sequence(&[], &[]);
        let job = UpdateCameraClipJob {
            clip_id: "".to_string(),
            update: Arc::new(|camera_clip| camera_clip.clone()),
        };

        assert_eq!(job.execute(&sequence).is_err(), true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn update_camera_clip_should_works() {
        let sequence = mock_sequence(&["0"], &[]);
        let start_time = sequence.get_clip("0").unwrap().get_start_time();
        let next_start_time = start_time + Time::from_ms(1.0);

        let job = UpdateCameraClipJob {
            clip_id: "0".to_string(),
            update: Arc::new(move |camera_clip| {
                let mut camera_clip = camera_clip.clone();
                camera_clip.start_at = next_start_time;
                camera_clip
            }),
        };

        let result = job.execute(&sequence).unwrap();
        assert_eq!(
            result.get_clip("0").unwrap().get_start_time(),
            next_start_time
        );
    }
}
